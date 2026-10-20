//! The bootstrap server owns open documents and does not access project files.

use std::collections::HashMap;
use std::error::Error;

use lsp_server::{Connection, Message, Notification, Request, Response};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::syntax::{Document, Syntax};
use crate::text::PositionEncoding;

const MAX_OPEN_DOCUMENTS: usize = 256;
type ServerResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Lifecycle {
    BeforeInitialize,
    AwaitInitialized,
    Ready,
    Shutdown,
}

struct Server {
    lifecycle: Lifecycle,
    syntax: Syntax,
    documents: HashMap<String, Document>,
    encoding: PositionEncoding,
    hierarchical_symbols: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenParams {
    text_document: OpenDocument,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpenDocument {
    uri: String,
    language_id: String,
    version: i32,
    text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangeParams {
    text_document: VersionedIdentifier,
    content_changes: Vec<TextChange>,
}

#[derive(Deserialize)]
struct VersionedIdentifier {
    uri: String,
    version: i32,
}

#[derive(Deserialize)]
struct TextChange {
    text: String,
    range: Option<Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentParams {
    text_document: DocumentIdentifier,
}

#[derive(Deserialize)]
struct DocumentIdentifier {
    uri: String,
}

pub fn run_stdio() -> ServerResult<u8> {
    crate::transport::run(serve)
}

/// The in-memory connection permits protocol tests without a subprocess.
pub fn serve(connection: &Connection) -> ServerResult<u8> {
    let mut server = Server {
        lifecycle: Lifecycle::BeforeInitialize,
        syntax: Syntax::new()?,
        documents: HashMap::new(),
        encoding: PositionEncoding::Utf16,
        hierarchical_symbols: false,
    };
    for message in &connection.receiver {
        match message {
            Message::Request(request) => server.request(connection, request)?,
            Message::Notification(notification) => {
                if notification.method == "exit" {
                    return Ok(u8::from(server.lifecycle != Lifecycle::Shutdown));
                }
                if let Err(error) = server.notification(connection, notification) {
                    eprintln!("quicksilver: {error}");
                }
            }
            Message::Response(_) => {}
        }
    }
    Ok(u8::from(server.lifecycle != Lifecycle::Shutdown))
}

impl Server {
    fn request(&mut self, connection: &Connection, request: Request) -> ServerResult<()> {
        if self.lifecycle == Lifecycle::Shutdown {
            return send_error(connection, request, -32600, "The server is shut down.");
        }
        if request.method == "initialize" {
            if self.lifecycle != Lifecycle::BeforeInitialize {
                return send_error(
                    connection,
                    request,
                    -32600,
                    "The server is already initialized.",
                );
            }
            if !request.params.is_object() {
                return send_error(
                    connection,
                    request,
                    -32602,
                    "Initialize parameters must be an object.",
                );
            }
            self.encoding = select_encoding(&request.params);
            self.hierarchical_symbols = request
                .params
                .pointer(
                    "/capabilities/textDocument/documentSymbol/hierarchicalDocumentSymbolSupport",
                )
                .and_then(Value::as_bool)
                .unwrap_or(false);
            self.lifecycle = Lifecycle::AwaitInitialized;
            return send_result(
                connection,
                request,
                json!({
                    "capabilities": {
                        "positionEncoding": self.encoding.as_str(),
                        "textDocumentSync": {"openClose": true, "change": 1},
                        "documentSymbolProvider": true,
                        "experimental": {"quicksilver": {
                            "stage": "bootstrap",
                            "diagnosticCoverage": "syntax-only",
                            "projectExecution": false,
                            "statusMethod": "quicksilver/status"
                        }}
                    },
                    "serverInfo": {"name": "quicksilver", "version": env!("CARGO_PKG_VERSION")}
                }),
            );
        }
        if self.lifecycle != Lifecycle::Ready {
            return send_error(
                connection,
                request,
                -32002,
                "The server is not initialized.",
            );
        }
        match request.method.as_str() {
            "shutdown" => {
                self.lifecycle = Lifecycle::Shutdown;
                self.documents.clear();
                send_result(connection, request, Value::Null)
            }
            "quicksilver/status" => send_result(
                connection,
                request,
                json!({
                    "stage": "bootstrap",
                    "openDocuments": self.documents.len(),
                    "positionEncoding": self.encoding.as_str(),
                    "diagnosticCoverage": "syntax-only",
                    "semanticReady": false,
                    "projectExecution": false
                }),
            ),
            "textDocument/documentSymbol" => {
                let params = match serde_json::from_value::<DocumentParams>(request.params.clone())
                {
                    Ok(params) => params,
                    Err(_) => {
                        return send_error(
                            connection,
                            request,
                            -32602,
                            "A textDocument URI is required.",
                        )
                    }
                };
                let Some(document) = self.documents.get(&params.text_document.uri) else {
                    return send_error(
                        connection,
                        request,
                        -32602,
                        "Open this Elixir document before the request.",
                    );
                };
                match document.symbols(self.encoding, self.hierarchical_symbols) {
                    Ok(symbols) => send_result(connection, request, symbols),
                    Err(error) => send_error(connection, request, -32803, &error),
                }
            }
            _ => send_error(
                connection,
                request,
                -32601,
                "This method is not implemented.",
            ),
        }
    }

    fn notification(
        &mut self,
        connection: &Connection,
        notification: Notification,
    ) -> ServerResult<()> {
        if notification.method == "initialized" && self.lifecycle == Lifecycle::AwaitInitialized {
            self.lifecycle = Lifecycle::Ready;
            return Ok(());
        }
        if self.lifecycle != Lifecycle::Ready {
            return Ok(());
        }
        match notification.method.as_str() {
            "textDocument/didOpen" => {
                let params: OpenParams = serde_json::from_value(notification.params)?;
                let open = params.text_document;
                if open.language_id != "elixir" || open.uri.is_empty() {
                    return Err("The bootstrap accepts open Elixir documents only.".into());
                }
                if self.documents.contains_key(&open.uri) {
                    return Err("The document is already open. Use didChange to update it.".into());
                }
                if self.documents.len() >= MAX_OPEN_DOCUMENTS {
                    return Err(format!(
                        "The bootstrap permits at most {MAX_OPEN_DOCUMENTS} open documents."
                    )
                    .into());
                }
                let document = self
                    .syntax
                    .parse(open.uri.clone(), open.version, open.text)?;
                publish_diagnostics(connection, &document, self.encoding)?;
                self.documents.insert(open.uri, document);
            }
            "textDocument/didChange" => {
                let params: ChangeParams = serde_json::from_value(notification.params)?;
                let Some(previous) = self.documents.get(&params.text_document.uri) else {
                    return Err("The document is not open.".into());
                };
                if params.text_document.version <= previous.version {
                    return Err(
                        "The document version did not increase. The update was rejected.".into(),
                    );
                }
                if params
                    .content_changes
                    .iter()
                    .any(|change| change.range.is_some())
                {
                    return Err("The bootstrap requires full document updates. The ranged update was rejected.".into());
                }
                let Some(change) = params.content_changes.into_iter().last() else {
                    return Err("The document update contains no text.".into());
                };
                let uri = params.text_document.uri;
                let document =
                    self.syntax
                        .parse(uri.clone(), params.text_document.version, change.text)?;
                publish_diagnostics(connection, &document, self.encoding)?;
                self.documents.insert(uri, document);
            }
            "textDocument/didClose" => {
                let params: DocumentParams = serde_json::from_value(notification.params)?;
                let uri = params.text_document.uri;
                if self.documents.remove(&uri).is_some() {
                    connection
                        .sender
                        .send(Message::Notification(Notification::new(
                            "textDocument/publishDiagnostics".to_owned(),
                            json!({"uri": uri, "diagnostics": []}),
                        )))?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn select_encoding(params: &Value) -> PositionEncoding {
    if let Some(encodings) = params
        .pointer("/capabilities/general/positionEncodings")
        .and_then(Value::as_array)
    {
        for encoding in encodings {
            match encoding.as_str() {
                Some("utf-8") => return PositionEncoding::Utf8,
                Some("utf-16") => return PositionEncoding::Utf16,
                _ => {}
            }
        }
    }
    PositionEncoding::Utf16
}

fn publish_diagnostics(
    connection: &Connection,
    document: &Document,
    encoding: PositionEncoding,
) -> ServerResult<()> {
    let diagnostics = document.diagnostics(encoding)?;
    connection
        .sender
        .send(Message::Notification(Notification::new(
            "textDocument/publishDiagnostics".to_owned(),
            json!({"uri": document.uri, "version": document.version, "diagnostics": diagnostics}),
        )))?;
    Ok(())
}

fn send_result(connection: &Connection, request: Request, result: Value) -> ServerResult<()> {
    connection
        .sender
        .send(Message::Response(Response::new_ok(request.id, result)))?;
    Ok(())
}

fn send_error(
    connection: &Connection,
    request: Request,
    code: i32,
    message: &str,
) -> ServerResult<()> {
    connection.sender.send(Message::Response(Response::new_err(
        request.id,
        code,
        message.to_owned(),
    )))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding_follows_client_preference_with_utf16_fallback() {
        assert_eq!(select_encoding(&json!({})), PositionEncoding::Utf16);
        assert_eq!(
            select_encoding(
                &json!({"capabilities": {"general": {"positionEncodings": ["utf-8", "utf-16"]}}})
            ),
            PositionEncoding::Utf8
        );
        assert_eq!(
            select_encoding(
                &json!({"capabilities": {"general": {"positionEncodings": ["utf-32", "utf-16", "utf-8"]}}})
            ),
            PositionEncoding::Utf16
        );
    }

    #[test]
    fn invalid_changes_preserve_the_last_valid_document() {
        let (connection, _client) = Connection::memory();
        let mut server = Server {
            lifecycle: Lifecycle::Ready,
            syntax: Syntax::new().unwrap(),
            documents: HashMap::new(),
            encoding: PositionEncoding::Utf16,
            hierarchical_symbols: true,
        };
        let uri = "file:///unchanged.ex";
        server.notification(&connection, Notification::new("textDocument/didOpen".to_owned(), json!({
            "textDocument": {"uri": uri, "languageId": "elixir", "version": 1, "text": "def original(), do: :ok"}
        }))).unwrap();
        for changes in [
            json!([{"text": null}]),
            json!([{"range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}}, "text": "new"}]),
            json!([]),
        ] {
            let result = server.notification(
                &connection,
                Notification::new(
                    "textDocument/didChange".to_owned(),
                    json!({
                        "textDocument": {"uri": uri, "version": 2}, "contentChanges": changes
                    }),
                ),
            );
            assert!(result.is_err());
            let document = server.documents.get(uri).unwrap();
            assert_eq!(document.version, 1);
            assert_eq!(
                document.symbols(PositionEncoding::Utf16, true).unwrap()[0]["name"],
                "original/0"
            );
        }
    }
}
