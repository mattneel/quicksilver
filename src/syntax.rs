//! Syntax results describe source forms, without macro expansion.

use serde::Serialize;
use serde_json::{json, Value};
use tree_sitter::{Node, Parser, Tree};

use crate::text::{LineIndex, PositionEncoding, Range};

pub const MAX_DOCUMENT_BYTES: usize = 2 * 1024 * 1024;
const MAX_SYNTAX_DEPTH: usize = 256;
const MAX_DIAGNOSTICS: usize = 100;

pub struct Syntax {
    parser: Parser,
}

impl Syntax {
    pub fn new() -> Result<Self, String> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_elixir::LANGUAGE.into())
            .map_err(|error| format!("The Elixir grammar is incompatible: {error}"))?;
        Ok(Self { parser })
    }

    pub fn parse(&mut self, uri: String, version: i32, text: String) -> Result<Document, String> {
        if text.len() > MAX_DOCUMENT_BYTES {
            return Err(format!(
                "The document exceeds the {MAX_DOCUMENT_BYTES}-byte bootstrap limit."
            ));
        }
        let tree = self
            .parser
            .parse(&text, None)
            .ok_or_else(|| "The Elixir parser did not produce a syntax tree.".to_owned())?;
        let lines = LineIndex::new(&text);
        Ok(Document {
            uri,
            version,
            text,
            tree,
            lines,
        })
    }
}

pub struct Document {
    pub uri: String,
    pub version: i32,
    text: String,
    tree: Tree,
    lines: LineIndex,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbol {
    name: String,
    detail: String,
    kind: u32,
    range: Range,
    selection_range: Range,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<DocumentSymbol>,
}

impl Document {
    pub fn symbols(&self, encoding: PositionEncoding, hierarchical: bool) -> Result<Value, String> {
        let symbols = self.collect(self.tree.root_node(), encoding, 0)?;
        if hierarchical {
            return Ok(json!(symbols));
        }
        let mut flat = Vec::new();
        flatten_symbols(&symbols, &self.uri, None, &mut flat);
        Ok(Value::Array(flat))
    }

    pub fn diagnostics(&self, encoding: PositionEncoding) -> Result<Vec<Value>, String> {
        let mut nodes = vec![self.tree.root_node()];
        let mut diagnostics = Vec::new();
        while let Some(node) = nodes.pop() {
            if diagnostics.len() == MAX_DIAGNOSTICS {
                break;
            }
            if node.is_error() || node.is_missing() {
                let range = self.node_range(node, encoding)?;
                let message = if node.is_missing() {
                    format!("Syntax is incomplete. Expected {}.", node.kind())
                } else {
                    "Syntax is incomplete or invalid.".to_owned()
                };
                diagnostics.push(json!({
                    "range": range,
                    "severity": 1,
                    "code": "syntax-error",
                    "source": "quicksilver-syntax",
                    "message": message,
                    "data": {"coverage": "syntax-only", "documentVersion": self.version}
                }));
                if node.is_error() {
                    continue;
                }
            }
            let mut cursor = node.walk();
            let children: Vec<Node<'_>> = node.children(&mut cursor).collect();
            nodes.extend(children.into_iter().rev());
        }
        Ok(diagnostics)
    }

    fn node_range(&self, node: Node<'_>, encoding: PositionEncoding) -> Result<Range, String> {
        self.lines
            .range(&self.text, node.start_byte(), node.end_byte(), encoding)
            .ok_or_else(|| "A syntax range does not match the document encoding.".to_owned())
    }

    fn text_of(&self, node: Node<'_>) -> &str {
        &self.text[node.start_byte()..node.end_byte()]
    }

    fn collect(
        &self,
        node: Node<'_>,
        encoding: PositionEncoding,
        depth: usize,
    ) -> Result<Vec<DocumentSymbol>, String> {
        if depth > MAX_SYNTAX_DEPTH {
            return Err(
                "The syntax exceeds the bootstrap depth limit for document symbols.".to_owned(),
            );
        }
        if node.kind() == "call" {
            if let Some(target) = node.child_by_field_name("target") {
                if target.kind() == "identifier" {
                    let form = self.text_of(target);
                    if form == "quote" || form == "defimpl" {
                        return Ok(Vec::new());
                    }
                    if form == "defmodule" || form == "defprotocol" {
                        return self.module_symbol(node, form, encoding, depth);
                    }
                    if matches!(
                        form,
                        "def"
                            | "defp"
                            | "defmacro"
                            | "defmacrop"
                            | "defguard"
                            | "defguardp"
                            | "defdelegate"
                    ) {
                        return self.function_symbol(node, form, encoding);
                    }
                }
            }
        }
        let mut symbols = Vec::new();
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            symbols.extend(self.collect(child, encoding, depth + 1)?);
        }
        Ok(symbols)
    }

    fn module_symbol(
        &self,
        node: Node<'_>,
        form: &str,
        encoding: PositionEncoding,
        depth: usize,
    ) -> Result<Vec<DocumentSymbol>, String> {
        let Some(arguments) = named_child_of_kind(node, "arguments") else {
            return Ok(Vec::new());
        };
        let Some(name) = arguments.named_child(0) else {
            return Ok(Vec::new());
        };
        if name.kind() != "alias" {
            return Ok(Vec::new());
        }
        let mut children = Vec::new();
        if let Some(block) = named_child_of_kind(node, "do_block") {
            children = self.collect(block, encoding, depth + 1)?;
        }
        Ok(vec![DocumentSymbol {
            name: self.text_of(name).to_owned(),
            detail: format!("{form} (source syntax)"),
            kind: if form == "defprotocol" { 11 } else { 2 },
            range: self.node_range(node, encoding)?,
            selection_range: self.node_range(name, encoding)?,
            children,
        }])
    }

    fn function_symbol(
        &self,
        node: Node<'_>,
        form: &str,
        encoding: PositionEncoding,
    ) -> Result<Vec<DocumentSymbol>, String> {
        let Some(arguments) = named_child_of_kind(node, "arguments") else {
            return Ok(Vec::new());
        };
        let Some(head) = arguments.named_child(0) else {
            return Ok(Vec::new());
        };
        let Some((name, arity)) = self.function_head(head) else {
            return Ok(Vec::new());
        };
        Ok(vec![DocumentSymbol {
            name: format!("{}/{arity}", self.text_of(name)),
            detail: format!("{form} (source syntax)"),
            kind: 12,
            range: self.node_range(node, encoding)?,
            selection_range: self.node_range(name, encoding)?,
            children: Vec::new(),
        }])
    }

    fn function_head<'tree>(&self, mut node: Node<'tree>) -> Option<(Node<'tree>, usize)> {
        if node.kind() == "binary_operator" {
            let operator = node.child_by_field_name("operator")?;
            if self.text_of(operator) != "when" {
                return None;
            }
            node = node.child_by_field_name("left")?;
        }
        if node.kind() == "identifier" {
            return Some((node, 0));
        }
        if node.kind() != "call" {
            return None;
        }
        let target = node.child_by_field_name("target")?;
        if target.kind() != "identifier" {
            return None;
        }
        let arity = named_child_of_kind(node, "arguments").map_or(0, |arguments| {
            let mut cursor = arguments.walk();
            arguments
                .named_children(&mut cursor)
                .filter(|child| child.kind() != "comment")
                .count()
        });
        Some((target, arity))
    }
}

fn named_child_of_kind<'tree>(node: Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    let mut cursor = node.walk();
    let child = node
        .named_children(&mut cursor)
        .find(|child| child.kind() == kind);
    child
}

fn flatten_symbols(
    symbols: &[DocumentSymbol],
    uri: &str,
    container: Option<&str>,
    output: &mut Vec<Value>,
) {
    for symbol in symbols {
        let mut value = json!({
            "name": symbol.name,
            "kind": symbol.kind,
            "location": {"uri": uri, "range": symbol.range}
        });
        if let Some(container) = container {
            value["containerName"] = Value::String(container.to_owned());
        }
        output.push(value);
        flatten_symbols(&symbol.children, uri, Some(&symbol.name), output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> Document {
        Syntax::new()
            .unwrap()
            .parse("file:///example.ex".to_owned(), 7, text.to_owned())
            .unwrap()
    }

    #[test]
    fn declaration_outline_excludes_calls_comments_strings_and_quotes() {
        let document = parse(
            "defmodule Example do\n  # def fake(), do: :fake\n  @doc \"def fake(), do: :fake\"\n  \
             def hello(name), do: String.upcase(name)\n  quote do\n    def generated(), do: :ok\n  end\nend\n",
        );
        let symbols = document.symbols(PositionEncoding::Utf16, true).unwrap();
        assert_eq!(symbols.as_array().unwrap().len(), 1);
        assert_eq!(symbols[0]["name"], "Example");
        let children = symbols[0]["children"].as_array().unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0]["name"], "hello/1");
        assert_eq!(
            children[0]["selectionRange"]["start"],
            json!({"line": 3, "character": 6})
        );
        assert!(document
            .diagnostics(PositionEncoding::Utf16)
            .unwrap()
            .is_empty());
    }

    #[test]
    fn clauses_guards_and_defaults_remain_source_declarations() {
        let document = parse(
            "defmodule Sample do\n  def zero, do: 0\n  def guarded(value) when is_integer(value), do: value\n  \
             def defaults(value \\\\ 1), do: value\n  def clause(:a), do: :a\n  def clause(:b), do: :b\nend\n",
        );
        let symbols = document.symbols(PositionEncoding::Utf16, true).unwrap();
        let names: Vec<&str> = symbols[0]["children"]
            .as_array()
            .unwrap()
            .iter()
            .map(|symbol| symbol["name"].as_str().unwrap())
            .collect();
        assert_eq!(
            names,
            ["zero/0", "guarded/1", "defaults/1", "clause/1", "clause/1"]
        );
    }

    #[test]
    fn unicode_symbol_ranges_match_the_selected_encoding() {
        let document = parse("defmodule Unicode do\n  def café(value), do: value\nend\n");
        let utf16 = document.symbols(PositionEncoding::Utf16, true).unwrap();
        let utf8 = document.symbols(PositionEncoding::Utf8, true).unwrap();
        assert_eq!(
            utf16[0]["children"][0]["selectionRange"]["end"]["character"],
            10
        );
        assert_eq!(
            utf8[0]["children"][0]["selectionRange"]["end"]["character"],
            11
        );
    }

    #[test]
    fn flat_symbols_include_containers() {
        let document = parse("defmodule Example do\n  def hello(), do: :ok\nend\n");
        let symbols = document.symbols(PositionEncoding::Utf16, false).unwrap();
        assert_eq!(symbols.as_array().unwrap().len(), 2);
        assert_eq!(symbols[1]["containerName"], "Example");
        assert_eq!(symbols[1]["location"]["uri"], "file:///example.ex");
    }

    #[test]
    fn invalid_syntax_has_explicit_syntax_only_coverage() {
        let document = parse("defmodule Broken do\n  def broken(, do: :ok\n");
        let diagnostics = document.diagnostics(PositionEncoding::Utf16).unwrap();
        assert!(!diagnostics.is_empty());
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic["source"] == "quicksilver-syntax"));
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic["data"]["documentVersion"] == 7));
    }

    #[test]
    fn dynamic_module_names_do_not_claim_static_definitions() {
        let document = parse("defmodule unquote(name) do\n  def hidden(), do: :ok\nend\n");
        assert_eq!(
            document.symbols(PositionEncoding::Utf16, true).unwrap(),
            json!([])
        );
    }

    #[test]
    fn oversized_source_is_rejected_before_parse() {
        let text = " ".repeat(MAX_DOCUMENT_BYTES + 1);
        let error = Syntax::new()
            .unwrap()
            .parse("file:///large.ex".to_owned(), 1, text);
        assert!(error.is_err());
    }

    #[test]
    fn deeply_nested_syntax_returns_a_controlled_outline_error() {
        let depth = MAX_SYNTAX_DEPTH + 32;
        let text = format!("{}1{}", "(".repeat(depth), ")".repeat(depth));
        let document = parse(&text);
        let symbols = document.symbols(PositionEncoding::Utf16, true);
        assert!(symbols.is_err());
    }
}
