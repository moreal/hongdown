//! Link and image serialization logic.

use comrak::nodes::{AstNode, NodeValue};

use super::Serializer;

impl<'a> Serializer<'a> {
    /// Format a reference-style link and write to output buffer.
    pub(super) fn format_reference_link(
        &mut self,
        output: &mut String,
        text: &str,
        label: &str,
        url: &str,
        title: &str,
    ) {
        if label.starts_with('\x01') {
            // Collapsed reference: [text][]
            let actual_label = label.strip_prefix('\x01').unwrap();
            output.push('[');
            output.push_str(text);
            output.push_str("][]");

            self.add_reference(actual_label.to_string(), url.to_string(), title.to_string());
        } else if text == label {
            // Shortcut reference: [text]
            output.push('[');
            output.push_str(text);
            output.push(']');

            self.add_reference(label.to_string(), url.to_string(), title.to_string());
        } else {
            // Full reference: [text][label]
            output.push('[');
            output.push_str(text);
            output.push_str("][");
            output.push_str(label);
            output.push(']');

            self.add_reference(label.to_string(), url.to_string(), title.to_string());
        }
    }

    /// Format an inline-style link and write to output buffer.
    pub(super) fn format_inline_link(output: &mut String, text: &str, url: &str, title: &str) {
        output.push('[');
        output.push_str(text);
        output.push_str("](");
        output.push_str(url);
        if !title.is_empty() {
            output.push_str(" \"");
            output.push_str(title);
            output.push('"');
        }
        output.push(')');
    }

    /// Format an autolink and write to output buffer.
    pub(super) fn format_autolink(output: &mut String, url: &str) {
        output.push('<');
        output.push_str(url);
        output.push('>');
    }

    /// Format an external link as reference style and write to output buffer.
    pub(super) fn format_external_link_as_reference(
        &mut self,
        output: &mut String,
        text: &str,
        url: &str,
        title: &str,
    ) {
        // Normalize: replace SoftBreak markers with spaces for shortcut refs
        let normalized_text = text.replace('\x00', " ");
        output.push('[');
        output.push_str(&normalized_text);
        output.push(']');

        self.add_reference(normalized_text, url.to_string(), title.to_string());
    }

    /// Format a reference-style image and write to output buffer.
    pub(super) fn format_reference_image(
        &mut self,
        output: &mut String,
        text: &str,
        label: &str,
        url: &str,
        title: &str,
    ) {
        if label.starts_with('\x01') {
            // Collapsed reference: ![alt][]
            let actual_label = label.strip_prefix('\x01').unwrap();
            output.push_str("![");
            output.push_str(text);
            output.push_str("][]");

            self.add_reference(actual_label.to_string(), url.to_string(), title.to_string());
        } else if text == label {
            // Shortcut reference: ![alt]
            output.push_str("![");
            output.push_str(text);
            output.push(']');

            self.add_reference(label.to_string(), url.to_string(), title.to_string());
        } else {
            // Full reference: ![alt][label]
            output.push_str("![");
            output.push_str(text);
            output.push_str("][");
            output.push_str(label);
            output.push(']');

            self.add_reference(label.to_string(), url.to_string(), title.to_string());
        }
    }

    /// Format an inline-style image and write to output buffer.
    pub(super) fn format_inline_image(output: &mut String, alt_text: &str, url: &str, title: &str) {
        output.push_str("![");
        output.push_str(alt_text);
        output.push_str("](");
        output.push_str(url);
        if !title.is_empty() {
            output.push_str(" \"");
            output.push_str(title);
            output.push('"');
        }
        output.push(')');
    }

    pub(super) fn serialize_link<'b>(&mut self, node: &'b AstNode<'b>, url: &str, title: &str) {
        // Check if link contains an image (badge-style link)
        let contains_image = node
            .children()
            .any(|child| matches!(&child.data.borrow().value, NodeValue::Image(_)));

        // Check if this is an autolink (link text equals URL)
        let raw_text = self.collect_raw_text(node);
        let is_autolink = title.is_empty() && raw_text == url;

        // Check if original was reference style
        if let Some((text, label)) = self.get_reference_style_info(node) {
            // For badge-style, serialize children first to get image content
            if contains_image {
                // Badge-style with reference: [![alt][img-ref]][link-ref]
                self.output.push('[');
                for child in node.children() {
                    self.serialize_node(child);
                }
                self.output.push_str("][");
                let actual_label = label.strip_prefix('\x01').unwrap_or(&label);
                self.output.push_str(actual_label);
                self.output.push(']');
                self.add_reference(actual_label.to_string(), url.to_string(), title.to_string());
            } else {
                // Use helper for non-badge reference links
                let mut output = String::new();
                self.format_reference_link(&mut output, &text, &label, url, title);
                self.output.push_str(&output);
            }
        } else if contains_image {
            // Badge-style inline: [![alt](img-url)](link-url)
            self.output.push('[');
            for child in node.children() {
                self.serialize_node(child);
            }
            self.output.push_str("](");
            self.output.push_str(url);
            if !title.is_empty() {
                self.output.push_str(" \"");
                self.output.push_str(title);
                self.output.push('"');
            }
            self.output.push(')');
        } else if is_autolink {
            Self::format_autolink(&mut self.output, url);
        } else if Self::is_external_url(url) {
            let link_text = self.collect_text(node);
            let mut output = String::new();
            self.format_external_link_as_reference(&mut output, &link_text, url, title);
            self.output.push_str(&output);
        } else {
            // Relative/local URL: keep as inline link
            let link_text = self.collect_text(node);
            Self::format_inline_link(&mut self.output, &link_text, url, title);
        }
    }

    pub(super) fn serialize_image<'b>(&mut self, node: &'b AstNode<'b>, url: &str, title: &str) {
        // Collect the alt text
        let alt_text = self.collect_text(node);

        // Check if original was reference style
        if let Some((text, label)) = self.get_reference_style_info(node) {
            // Use a temporary buffer to avoid double borrow
            let mut output = String::new();
            self.format_reference_image(&mut output, &text, &label, url, title);
            self.output.push_str(&output);
            return;
        }

        // Inline style: ![alt](url)
        Self::format_inline_image(&mut self.output, &alt_text, url, title);
    }
}
