use crate::error::{BingrepError, Result};
use regex::bytes::Regex;

pub struct RegexProcessor;

impl RegexProcessor {
    /// Process and compile a regex pattern that may contain hex escapes and quantifiers
    pub fn compile_pattern(expression: &str) -> Result<Regex> {
        let pattern = if expression.contains("\\x") && !Self::has_regex_metacharacters(expression) {
            // Simple \xHH pattern - convert to binary then escape for regex
            let binary_pattern = Self::parse_hex_pattern(expression);
            if binary_pattern.is_empty() {
                return Err(BingrepError::InvalidPattern(
                    "No valid hex pattern found".to_string(),
                ));
            }
            Self::escape_bytes_for_regex(&binary_pattern)
        } else {
            // Pattern with regex metacharacters - convert only \xHH while preserving quantifiers
            Self::convert_hex_escapes_in_pattern(expression)
        };

        Regex::new(&pattern).map_err(BingrepError::from)
    }

    /// Parse \xHH sequences into bytes
    pub fn parse_hex_pattern(pattern: &str) -> Vec<u8> {
        let mut result = Vec::new();
        let mut chars = pattern.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == 'x' || next_ch == 'X' {
                        chars.next(); // consume 'x' or 'X'

                        // Parse next 2 characters as hex
                        let hex1 = chars.next();
                        let hex2 = chars.next();

                        if let (Some(h1), Some(h2)) = (hex1, hex2) {
                            let hex_str = format!("{}{}", h1, h2);
                            if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                                result.push(byte);
                                continue;
                            }
                        }
                    }
                }
            }
            // Ignore non-hex characters for simple patterns
        }

        result
    }

    /// Escape bytes for regex use
    pub fn escape_bytes_for_regex(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|&b| {
                // All bytes are escaped as \xHH for binary regex
                format!("\\x{:02x}", b)
            })
            .collect()
    }

    /// Check if pattern contains regex metacharacters
    fn has_regex_metacharacters(pattern: &str) -> bool {
        pattern.chars().any(|c| {
            matches!(
                c,
                '+' | '*' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '^' | '$'
            )
        })
    }

    /// Convert hex escapes in pattern while preserving other regex syntax
    fn convert_hex_escapes_in_pattern(pattern: &str) -> String {
        let mut result = String::new();
        let mut chars = pattern.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == 'x' || next_ch == 'X' {
                        chars.next(); // consume 'x'

                        // Parse next 2 characters as hex
                        let hex1 = chars.next();
                        let hex2 = chars.next();

                        if let (Some(h1), Some(h2)) = (hex1, hex2) {
                            if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                                // Convert byte to regex form
                                result.push_str(&format!("\\x{:02x}", byte));
                                continue;
                            }
                        }
                        // If conversion fails, preserve original
                        result.push('\\');
                        result.push('x');
                        if let Some(h1) = hex1 {
                            result.push(h1);
                        }
                        if let Some(h2) = hex2 {
                            result.push(h2);
                        }
                    } else {
                        result.push('\\');
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(ch);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_pattern_basic() {
        let pattern = "\\x00\\x01\\x02\\xFF";
        let result = RegexProcessor::parse_hex_pattern(pattern);
        assert_eq!(result, vec![0x00, 0x01, 0x02, 0xFF]);
    }

    #[test]
    fn test_parse_hex_pattern_mixed_case() {
        let pattern = "\\x0a\\x0B\\xfF\\xAA";
        let result = RegexProcessor::parse_hex_pattern(pattern);
        assert_eq!(result, vec![0x0a, 0x0B, 0xFF, 0xAA]);
    }

    #[test]
    fn test_parse_hex_pattern_with_text() {
        let pattern = "prefix\\x41\\x42\\x43suffix";
        let result = RegexProcessor::parse_hex_pattern(pattern);
        assert_eq!(result, vec![0x41, 0x42, 0x43]);
    }

    #[test]
    fn test_escape_bytes_for_regex_basic() {
        let bytes = vec![0x00, 0x01, 0x41, 0xFF];
        let result = RegexProcessor::escape_bytes_for_regex(&bytes);
        assert_eq!(result, "\\x00\\x01\\x41\\xff");
    }

    #[test]
    fn test_compile_pattern_simple() {
        let result = RegexProcessor::compile_pattern("\\x00\\x01\\x02");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_pattern_with_quantifier() {
        let result = RegexProcessor::compile_pattern("\\x58{2,3}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_has_regex_metacharacters() {
        assert!(RegexProcessor::has_regex_metacharacters("\\x58{2}"));
        assert!(RegexProcessor::has_regex_metacharacters("\\x58+"));
        assert!(!RegexProcessor::has_regex_metacharacters("\\x58\\x59"));
    }
}