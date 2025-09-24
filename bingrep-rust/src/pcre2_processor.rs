use crate::error::{BingrepError, Result};
use pcre2::bytes::Regex as Pcre2Regex;

pub struct Pcre2Processor;

impl Pcre2Processor {
    /// Process and compile a regex pattern using PCRE2 engine
    pub fn compile_pattern(expression: &str) -> Result<Pcre2Regex> {
        let pattern = if expression.contains("\\x") && !Self::has_regex_metacharacters(expression) {
            // Simple \xHH pattern - convert to binary then escape for regex
            let binary_pattern = Self::parse_hex_pattern(expression)?;
            if binary_pattern.is_empty() {
                return Err(BingrepError::InvalidPattern(
                    "No valid hex pattern found".to_string(),
                ));
            }
            Self::escape_bytes_for_regex(&binary_pattern)
        } else {
            // Pattern with regex metacharacters - convert only \xHH while preserving quantifiers
            Self::convert_hex_escapes_in_pattern(expression)?
        };

        Pcre2Regex::new(&pattern).map_err(|err| BingrepError::RegexCompilation(err.to_string()))
    }

    /// Parse \xHH sequences into bytes
    pub fn parse_hex_pattern(pattern: &str) -> Result<Vec<u8>> {
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

                        match (hex1, hex2) {
                            (Some(h1), Some(h2)) => {
                                let hex_str = format!("{}{}", h1, h2);
                                match u8::from_str_radix(&hex_str, 16) {
                                    Ok(byte) => result.push(byte),
                                    Err(_) => {
                                        return Err(BingrepError::InvalidPattern(format!(
                                            "Invalid hex sequence: \\x{}",
                                            hex_str
                                        )));
                                    }
                                }
                            }
                            (Some(h1), None) => {
                                return Err(BingrepError::InvalidPattern(format!(
                                    "Incomplete hex sequence: \\x{}",
                                    h1
                                )));
                            }
                            (None, _) => {
                                return Err(BingrepError::InvalidPattern(
                                    "Incomplete hex sequence: \\x".to_string(),
                                ));
                            }
                        }
                    }
                }
            }
            // Ignore non-hex characters for simple patterns
        }

        Ok(result)
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
    fn convert_hex_escapes_in_pattern(pattern: &str) -> Result<String> {
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

                        match (hex1, hex2) {
                            (Some(h1), Some(h2)) => {
                                let hex_str = format!("{}{}", h1, h2);
                                match u8::from_str_radix(&hex_str, 16) {
                                    Ok(byte) => {
                                        // Convert byte to regex form
                                        result.push_str(&format!("\\x{:02x}", byte));
                                    }
                                    Err(_) => {
                                        return Err(BingrepError::InvalidPattern(format!(
                                            "Invalid hex sequence in regex pattern: \\x{}",
                                            hex_str
                                        )));
                                    }
                                }
                            }
                            (Some(h1), None) => {
                                return Err(BingrepError::InvalidPattern(format!(
                                    "Incomplete hex sequence in regex pattern: \\x{}",
                                    h1
                                )));
                            }
                            (None, _) => {
                                return Err(BingrepError::InvalidPattern(
                                    "Incomplete hex sequence in regex pattern: \\x".to_string(),
                                ));
                            }
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

        Ok(result)
    }

    /// Find all matches in the given data using PCRE2
    pub fn find_matches(regex: &Pcre2Regex, data: &[u8]) -> Result<Vec<(usize, usize)>> {
        let mut matches = Vec::new();

        for mat in regex.find_iter(data) {
            match mat {
                Ok(m) => matches.push((m.start(), m.end())),
                Err(err) => {
                    return Err(BingrepError::RegexCompilation(format!(
                        "PCRE2 match error: {}",
                        err
                    )))
                }
            }
        }

        Ok(matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcre2_compile_pattern_simple() {
        let result = Pcre2Processor::compile_pattern("\\x00\\x01\\x02");
        assert!(result.is_ok());
    }

    #[test]
    fn test_pcre2_compile_pattern_with_quantifier() {
        let result = Pcre2Processor::compile_pattern("\\x58{2,3}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_pcre2_parse_hex_pattern_basic() {
        let pattern = "\\x00\\x01\\x02\\xFF";
        let result = Pcre2Processor::parse_hex_pattern(pattern).unwrap();
        assert_eq!(result, vec![0x00, 0x01, 0x02, 0xFF]);
    }

    #[test]
    fn test_pcre2_find_matches() {
        let regex = Pcre2Processor::compile_pattern("\\x41\\x42").unwrap(); // "AB"
        let data = b"XABCABDEFAB";
        let matches = Pcre2Processor::find_matches(&regex, data).unwrap();

        // Should find "AB" matches at positions 1, 4, and 9
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0], (1, 3));
        assert_eq!(matches[1], (4, 6));
        assert_eq!(matches[2], (9, 11));
    }
}
