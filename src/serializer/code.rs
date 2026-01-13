//! Code block serialization logic.

use comrak::nodes::NodeCodeBlock;

use super::Serializer;

impl<'a> Serializer<'a> {
    /// Try to format code using an external formatter.
    ///
    /// Returns `Some(formatted_code)` if a formatter is configured for the language
    /// and succeeds. Returns `None` if no formatter is configured or if the formatter
    /// fails (in which case a warning is added).
    #[cfg(not(target_arch = "wasm32"))]
    fn try_format_code(&mut self, language: &str, code: &str) -> Option<String> {
        use super::formatter::run_formatter;

        let formatter = self.options.code_formatters.get(language)?;

        match run_formatter(&formatter.command, code, formatter.timeout_secs) {
            Ok(formatted) => Some(formatted),
            Err(e) => {
                // Add warning with line 0 for now (we don't have source position here)
                // This will be improved when we have access to the node's source position
                self.add_warning(
                    0,
                    format!(
                        "code formatter '{}' failed for language '{}': {}",
                        formatter.command.join(" "),
                        language,
                        e
                    ),
                );
                None
            }
        }
    }

    /// WASM: use the callback if provided.
    #[cfg(target_arch = "wasm32")]
    fn try_format_code(&mut self, language: &str, code: &str) -> Option<String> {
        #[cfg(feature = "wasm")]
        if let Some(ref callback) = self.code_formatter_callback {
            return callback(language, code);
        }
        None
    }

    /// Serialize a code block with indent for description list details.
    pub(super) fn serialize_code_block_with_indent(&mut self, code: &NodeCodeBlock, indent: &str) {
        let fence_char = self.options.fence_char;
        let min_len = self.options.min_fence_length;
        let base_fence: String = std::iter::repeat_n(fence_char, min_len).collect();
        let long_fence: String = std::iter::repeat_n(fence_char, min_len + 1).collect();

        // Determine language for formatter lookup
        let language = if code.info.is_empty() {
            &self.options.default_language
        } else {
            &code.info
        };

        // Try to format the code if a formatter is configured
        let formatted_literal = if !language.is_empty() {
            self.try_format_code(language, &code.literal)
        } else {
            None
        };
        let literal = formatted_literal.as_deref().unwrap_or(&code.literal);

        let fence = if literal.contains(&base_fence) {
            &long_fence
        } else {
            &base_fence
        };
        self.output.push_str(fence);
        if !code.info.is_empty() {
            if self.options.space_after_fence {
                self.output.push(' ');
            }
            self.output.push_str(&code.info);
        }
        self.output.push('\n');
        // Add indent to each line of code (skip indent for empty lines)
        for line in literal.lines() {
            if !line.is_empty() {
                self.output.push_str(indent);
                self.output.push_str(line);
            }
            self.output.push('\n');
        }
        // Handle trailing newline in literal
        if !literal.ends_with('\n') && !literal.is_empty() {
            self.output.push('\n');
        }
        self.output.push_str(indent);
        self.output.push_str(fence);
        self.output.push('\n');
    }

    pub(super) fn serialize_code_block(&mut self, info: &str, literal: &str) {
        // Determine the minimum fence length from options
        let min_fence_length = self.options.min_fence_length;
        let fence_char = self.options.fence_char;

        // Use default_language if no language specified (empty string means no language)
        let language = if info.is_empty() {
            &self.options.default_language
        } else {
            info
        };

        // Try to format the code if a formatter is configured
        let formatted_literal = if !language.is_empty() {
            self.try_format_code(language, literal)
        } else {
            None
        };
        let content = formatted_literal.as_deref().unwrap_or(literal);

        // Find the longest sequence of fence characters in the content
        let max_fence_in_content = content
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim_start();
                if trimmed.starts_with(fence_char) {
                    Some(trimmed.chars().take_while(|&c| c == fence_char).count())
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        // Fence length must be greater than any fence sequence in content
        let fence_length = std::cmp::max(min_fence_length, max_fence_in_content + 1);
        let fence: String = std::iter::repeat_n(fence_char, fence_length).collect();

        // Opening fence
        if self.in_block_quote {
            self.output.push_str("> ");
        }
        self.output.push_str(&fence);
        if !language.is_empty() {
            if self.options.space_after_fence {
                self.output.push(' ');
            }
            self.output.push_str(language);
        }
        self.output.push('\n');

        // Content lines
        for line in content.lines() {
            if self.in_block_quote {
                if line.is_empty() {
                    self.output.push('>');
                } else {
                    self.output.push_str("> ");
                }
            }
            self.output.push_str(line);
            self.output.push('\n');
        }

        // Closing fence
        if self.in_block_quote {
            self.output.push_str("> ");
        }
        self.output.push_str(&fence);
        self.output.push('\n');
    }

    /// Serialize a code block with indentation prefix on each line.
    /// Used for code blocks inside list items.
    pub(super) fn serialize_code_block_indented(
        &mut self,
        info: &str,
        literal: &str,
        indent: &str,
    ) {
        // Determine the minimum fence length from options
        let min_fence_length = self.options.min_fence_length;
        let fence_char = self.options.fence_char;

        // Use default_language if no language specified
        let language = if info.is_empty() {
            &self.options.default_language
        } else {
            info
        };

        // Try to format the code if a formatter is configured
        let formatted_literal = if !language.is_empty() {
            self.try_format_code(language, literal)
        } else {
            None
        };
        let content = formatted_literal.as_deref().unwrap_or(literal);

        // Find the longest sequence of fence characters in the content
        let max_fence_in_content = content
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim_start();
                if trimmed.starts_with(fence_char) {
                    Some(trimmed.chars().take_while(|&c| c == fence_char).count())
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        // Fence length must be greater than any fence sequence in content
        let fence_length = std::cmp::max(min_fence_length, max_fence_in_content + 1);
        let fence: String = std::iter::repeat_n(fence_char, fence_length).collect();

        // Output opening fence with optional language
        self.output.push_str(&fence);
        if !info.is_empty() {
            if self.options.space_after_fence {
                self.output.push(' ');
            }
            self.output.push_str(info);
        }
        self.output.push('\n');

        // Output content with indentation (skip indent for empty lines)
        for line in content.lines() {
            if self.in_block_quote {
                self.output.push_str("> ");
            }
            if !line.is_empty() {
                self.output.push_str(indent);
                self.output.push_str(line);
            }
            self.output.push('\n');
        }

        // Output closing fence with indentation
        if self.in_block_quote {
            self.output.push_str("> ");
        }
        self.output.push_str(indent);
        self.output.push_str(&fence);
        self.output.push('\n');
    }
}
