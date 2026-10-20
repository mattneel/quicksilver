//! Incoming frames have explicit limits before payload allocation.

use std::error::Error;
use std::io::{self, BufRead};
use std::thread;

use crossbeam_channel::bounded;
use lsp_server::{Connection, Message};
use serde_json::Value;

const MAX_HEADER_BYTES: usize = 8 * 1024;
const MAX_CONTENT_BYTES: usize = 16 * 1024 * 1024;
type TransportResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub(crate) fn run(serve: fn(&Connection) -> TransportResult<u8>) -> TransportResult<u8> {
    let (reader_sender, reader_receiver) = bounded(0);
    let (writer_sender, writer_receiver) = bounded::<Message>(0);
    let reader = thread::Builder::new()
        .name("quicksilver-input".to_owned())
        .spawn(move || -> io::Result<()> {
            let stdin = io::stdin();
            let mut input = stdin.lock();
            while let Some(message) = read_message(&mut input)? {
                let is_exit = matches!(&message, Message::Notification(notification) if notification.method == "exit");
                if reader_sender.send(message).is_err() || is_exit {
                    break;
                }
            }
            Ok(())
        })?;
    let writer = thread::Builder::new()
        .name("quicksilver-output".to_owned())
        .spawn(move || -> io::Result<()> {
            let stdout = io::stdout();
            let mut output = stdout.lock();
            for message in writer_receiver {
                message.write(&mut output)?;
            }
            Ok(())
        })?;
    let connection = Connection {
        sender: writer_sender,
        receiver: reader_receiver,
    };
    let result = serve(&connection);
    drop(connection);
    // Input can remain open after an output failure. Process exit ends the reader.
    let code = result?;
    join_io(reader)?;
    join_io(writer)?;
    Ok(code)
}

fn join_io(handle: thread::JoinHandle<io::Result<()>>) -> io::Result<()> {
    handle
        .join()
        .map_err(|_| io::Error::other("An LSP transport thread failed."))?
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn read_header_line(input: &mut impl BufRead, remaining: usize) -> io::Result<Option<Vec<u8>>> {
    let mut line = Vec::new();
    loop {
        let available = input.fill_buf()?;
        if available.is_empty() {
            return if line.is_empty() {
                Ok(None)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "An LSP header is incomplete.",
                ))
            };
        }
        let count = available
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(available.len(), |index| index + 1);
        if count > remaining.saturating_sub(line.len()) {
            return Err(invalid("LSP headers exceed the 8192-byte limit."));
        }
        line.extend_from_slice(&available[..count]);
        input.consume(count);
        if line.last() == Some(&b'\n') {
            if !line.ends_with(b"\r\n") {
                return Err(invalid("LSP headers require CRLF line endings."));
            }
            return Ok(Some(line));
        }
    }
}

fn read_message(input: &mut impl BufRead) -> io::Result<Option<Message>> {
    let mut remaining = MAX_HEADER_BYTES;
    let mut content_length = None;
    loop {
        let Some(line) = read_header_line(input, remaining)? else {
            return if remaining == MAX_HEADER_BYTES {
                Ok(None)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "LSP headers are incomplete.",
                ))
            };
        };
        remaining -= line.len();
        if line == b"\r\n" {
            break;
        }
        let header = std::str::from_utf8(&line[..line.len() - 2])
            .map_err(|_| invalid("LSP headers must contain ASCII text."))?;
        if !header.is_ascii() {
            return Err(invalid("LSP headers must contain ASCII text."));
        }
        let (name, value) = header
            .split_once(':')
            .ok_or_else(|| invalid("An LSP header is malformed."))?;
        if name.eq_ignore_ascii_case("Content-Length") {
            if content_length.is_some() {
                return Err(invalid(
                    "An LSP frame has duplicate Content-Length headers.",
                ));
            }
            let length = value
                .trim()
                .parse::<usize>()
                .map_err(|_| invalid("Content-Length must be a nonnegative integer."))?;
            if length > MAX_CONTENT_BYTES {
                return Err(invalid("The LSP payload exceeds the 16777216-byte limit."));
            }
            content_length = Some(length);
        }
    }
    let length = content_length.ok_or_else(|| invalid("An LSP frame requires Content-Length."))?;
    let mut payload = vec![0; length];
    input.read_exact(&mut payload)?;
    let value: Value = serde_json::from_slice(&payload)
        .map_err(|_| invalid("The LSP payload is invalid JSON."))?;
    if value.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        return Err(invalid("An LSP message requires jsonrpc version 2.0."));
    }
    serde_json::from_value(value)
        .map(Some)
        .map_err(|_| invalid("The LSP message shape is invalid."))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn huge_content_length_is_rejected_before_allocation() {
        let mut input = Cursor::new(b"Content-Length: 18446744073709551615\r\n\r\n");
        assert_eq!(
            read_message(&mut input).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn duplicate_lengths_and_oversized_headers_are_rejected() {
        let mut duplicate = Cursor::new(b"Content-Length: 2\r\nContent-Length: 2\r\n\r\n{}");
        assert!(read_message(&mut duplicate).is_err());
        let mut oversized = Cursor::new(vec![b'A'; MAX_HEADER_BYTES + 1]);
        assert_eq!(
            read_message(&mut oversized).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn concatenated_frames_and_utf8_byte_lengths_round_trip() {
        let payload = r#"{"jsonrpc":"2.0","method":"example","params":{"value":"😀"}}"#;
        let frame = format!("Content-Length: {}\r\n\r\n{payload}", payload.len());
        let mut input = Cursor::new(format!("{frame}{frame}"));
        assert!(matches!(
            read_message(&mut input).unwrap(),
            Some(Message::Notification(_))
        ));
        assert!(matches!(
            read_message(&mut input).unwrap(),
            Some(Message::Notification(_))
        ));
        assert!(read_message(&mut input).unwrap().is_none());
    }

    #[test]
    fn partial_payload_and_deep_json_fail_without_a_message() {
        let mut truncated = Cursor::new(b"Content-Length: 20\r\n\r\n{}");
        assert_eq!(
            read_message(&mut truncated).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
        let payload = format!("{}0{}", "[".repeat(256), "]".repeat(256));
        let mut nested = Cursor::new(format!(
            "Content-Length: {}\r\n\r\n{payload}",
            payload.len()
        ));
        assert_eq!(
            read_message(&mut nested).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
}
