//! Serializer for converting comrak AST to formatted Markdown.

use comrak::nodes::{AstNode, NodeValue};

use crate::Options;

/// Serializes a comrak AST node to a formatted Markdown string.
pub fn serialize<'a>(node: &'a AstNode<'a>, options: &Options) -> String {
    let mut serializer = Serializer::new(options);
    serializer.serialize_node(node);
    serializer.output
}

struct Serializer<'a> {
    output: String,
    #[allow(dead_code)]
    options: &'a Options,
}

impl<'a> Serializer<'a> {
    fn new(options: &'a Options) -> Self {
        Self {
            output: String::new(),
            options,
        }
    }

    fn serialize_node<'b>(&mut self, node: &'b AstNode<'b>) {
        match &node.data.borrow().value {
            NodeValue::Document => {
                self.serialize_children(node);
            }
            NodeValue::Heading(heading) => {
                self.serialize_heading(node, heading.level);
            }
            NodeValue::Paragraph => {
                self.serialize_children(node);
                self.output.push('\n');
            }
            NodeValue::Text(text) => {
                self.output.push_str(text);
            }
            NodeValue::SoftBreak => {
                self.output.push(' ');
            }
            NodeValue::LineBreak => {
                self.output.push('\n');
            }
            _ => {
                // For now, just recurse into children for unhandled nodes
                self.serialize_children(node);
            }
        }
    }

    fn serialize_heading<'b>(&mut self, node: &'b AstNode<'b>, level: u8) {
        // Collect heading text first
        let heading_text = self.collect_text(node);

        if level == 1 {
            // Setext-style with '='
            self.output.push_str(&heading_text);
            self.output.push('\n');
            self.output
                .push_str(&"=".repeat(heading_text.chars().count()));
            self.output.push('\n');
        } else if level == 2 {
            // Setext-style with '-'
            self.output.push_str(&heading_text);
            self.output.push('\n');
            self.output
                .push_str(&"-".repeat(heading_text.chars().count()));
            self.output.push('\n');
        } else {
            // ATX-style for level 3+
            self.output.push_str(&"#".repeat(level as usize));
            self.output.push(' ');
            self.output.push_str(&heading_text);
            self.output.push('\n');
        }
    }

    fn collect_text<'b>(&self, node: &'b AstNode<'b>) -> String {
        let mut text = String::new();
        self.collect_text_recursive(node, &mut text);
        text
    }

    fn collect_text_recursive<'b>(&self, node: &'b AstNode<'b>, text: &mut String) {
        match &node.data.borrow().value {
            NodeValue::Text(t) => {
                text.push_str(t);
            }
            NodeValue::SoftBreak => {
                text.push(' ');
            }
            _ => {
                for child in node.children() {
                    self.collect_text_recursive(child, text);
                }
            }
        }
    }

    fn serialize_children<'b>(&mut self, node: &'b AstNode<'b>) {
        for child in node.children() {
            self.serialize_node(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use comrak::{Arena, Options as ComrakOptions, parse_document};

    fn parse_and_serialize(input: &str) -> String {
        let arena = Arena::new();
        let options = ComrakOptions::default();
        let root = parse_document(&arena, input, &options);
        let format_options = Options::default();
        serialize(root, &format_options)
    }

    #[test]
    fn test_serialize_plain_text() {
        let result = parse_and_serialize("Hello, world!");
        assert_eq!(result, "Hello, world!\n");
    }

    #[test]
    fn test_serialize_multiline_paragraph() {
        let result = parse_and_serialize("Hello\nworld!");
        assert_eq!(result, "Hello world!\n");
    }

    #[test]
    fn test_serialize_h1_setext() {
        let result = parse_and_serialize("# Document Title");
        assert_eq!(result, "Document Title\n==============\n");
    }

    #[test]
    fn test_serialize_h2_setext() {
        let result = parse_and_serialize("## Section Name");
        assert_eq!(result, "Section Name\n------------\n");
    }

    #[test]
    fn test_serialize_h3_atx() {
        let result = parse_and_serialize("### Subsection");
        assert_eq!(result, "### Subsection\n");
    }

    #[test]
    fn test_serialize_h4_atx() {
        let result = parse_and_serialize("#### Deep Subsection");
        assert_eq!(result, "#### Deep Subsection\n");
    }
}
