//! Byte offsets and LSP positions use one explicit encoding per connection.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PositionEncoding {
    Utf8,
    #[default]
    Utf16,
}

impl PositionEncoding {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Utf8 => "utf-8",
            Self::Utf16 => "utf-16",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub struct LineIndex {
    starts: Vec<usize>,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        let mut starts = vec![0];
        starts.extend(
            text.as_bytes()
                .iter()
                .enumerate()
                .filter_map(|(index, byte)| (*byte == b'\n').then_some(index + 1)),
        );
        Self { starts }
    }

    fn line_bounds(&self, text: &str, line: usize) -> Option<(usize, usize)> {
        let start = *self.starts.get(line)?;
        let mut end = self.starts.get(line + 1).copied().unwrap_or(text.len());
        if end > start && text.as_bytes()[end - 1] == b'\n' {
            end -= 1;
            if end > start && text.as_bytes()[end - 1] == b'\r' {
                end -= 1;
            }
        }
        Some((start, end))
    }

    /// Newline bytes map to the end of their source line.
    pub fn position(
        &self,
        text: &str,
        offset: usize,
        encoding: PositionEncoding,
    ) -> Option<Position> {
        if offset > text.len() || !text.is_char_boundary(offset) {
            return None;
        }
        let line = self.starts.partition_point(|start| *start <= offset) - 1;
        let (start, end) = self.line_bounds(text, line)?;
        let prefix = &text[start..offset.min(end)];
        let character = match encoding {
            PositionEncoding::Utf8 => prefix.len(),
            PositionEncoding::Utf16 => prefix.encode_utf16().count(),
        };
        Some(Position {
            line: u32::try_from(line).ok()?,
            character: u32::try_from(character).ok()?,
        })
    }

    /// Invalid code-unit boundaries return no byte offset.
    pub fn byte_offset(
        &self,
        text: &str,
        position: Position,
        encoding: PositionEncoding,
    ) -> Option<usize> {
        let (start, end) = self.line_bounds(text, position.line as usize)?;
        let line = &text[start..end];
        let character = position.character as usize;
        match encoding {
            PositionEncoding::Utf8 => (character <= line.len() && line.is_char_boundary(character))
                .then_some(start + character),
            PositionEncoding::Utf16 => {
                let mut units = 0;
                for (offset, scalar) in line.char_indices() {
                    if units == character {
                        return Some(start + offset);
                    }
                    units += scalar.len_utf16();
                    if units > character {
                        return None;
                    }
                }
                (units == character).then_some(end)
            }
        }
    }

    pub fn range(
        &self,
        text: &str,
        start: usize,
        end: usize,
        encoding: PositionEncoding,
    ) -> Option<Range> {
        if start > end {
            return None;
        }
        Some(Range {
            start: self.position(text, start, encoding)?,
            end: self.position(text, end, encoding)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn astral_and_combining_characters_use_protocol_units() {
        let text = "a😀e\u{301}z\nnext";
        let index = LineIndex::new(text);
        assert_eq!(
            index.position(text, 5, PositionEncoding::Utf16),
            Some(Position {
                line: 0,
                character: 3
            })
        );
        assert_eq!(
            index.position(text, 5, PositionEncoding::Utf8),
            Some(Position {
                line: 0,
                character: 5
            })
        );
        assert_eq!(
            index.byte_offset(
                text,
                Position {
                    line: 0,
                    character: 5
                },
                PositionEncoding::Utf16
            ),
            Some(8)
        );
        assert_eq!(
            index.byte_offset(
                text,
                Position {
                    line: 0,
                    character: 2
                },
                PositionEncoding::Utf16
            ),
            None
        );
        assert_eq!(index.position(text, 2, PositionEncoding::Utf8), None);
    }

    #[test]
    fn crlf_and_trailing_empty_line_have_valid_positions() {
        let text = "one\r\ntwo\n";
        let index = LineIndex::new(text);
        assert_eq!(
            index.position(text, 4, PositionEncoding::Utf16),
            Some(Position {
                line: 0,
                character: 3
            })
        );
        assert_eq!(
            index.position(text, text.len(), PositionEncoding::Utf16),
            Some(Position {
                line: 2,
                character: 0
            })
        );
        assert_eq!(
            index.byte_offset(
                text,
                Position {
                    line: 0,
                    character: 4
                },
                PositionEncoding::Utf16
            ),
            None
        );
    }

    #[test]
    fn positions_round_trip_at_every_visible_character_boundary() {
        let text = "α😀\r\ncafe\u{301}\n";
        let index = LineIndex::new(text);
        for encoding in [PositionEncoding::Utf8, PositionEncoding::Utf16] {
            for (offset, character) in text.char_indices() {
                if character == '\r' || character == '\n' {
                    continue;
                }
                let position = index.position(text, offset, encoding).unwrap();
                assert_eq!(index.byte_offset(text, position, encoding), Some(offset));
            }
            let end = index.position(text, text.len(), encoding).unwrap();
            assert_eq!(index.byte_offset(text, end, encoding), Some(text.len()));
        }
    }

    #[test]
    fn out_of_range_positions_and_offsets_return_no_result() {
        let text = "example";
        let index = LineIndex::new(text);
        assert_eq!(
            index.position(text, usize::MAX, PositionEncoding::Utf16),
            None
        );
        assert_eq!(
            index.byte_offset(
                text,
                Position {
                    line: u32::MAX,
                    character: u32::MAX
                },
                PositionEncoding::Utf16
            ),
            None
        );
        assert_eq!(index.range(text, 4, 2, PositionEncoding::Utf8), None);
    }
}
