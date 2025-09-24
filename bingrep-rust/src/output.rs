/// Utilities for formatting binary data as hexadecimal output
pub struct OutputFormatter;

impl OutputFormatter {
    /// Format bytes as hexadecimal string with given separator
    pub fn format_bytes_as_hex(bytes: &[u8], separator: &str) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(separator)
    }

    /// Format offset with proper padding based on file size
    pub fn format_offset(offset: u64, hex_offset_length: usize) -> String {
        format!("{:0width$X}h", offset, width = hex_offset_length)
    }

    /// Calculate the number of digits needed for hex offset display
    pub fn calculate_hex_offset_length(file_size: u64) -> usize {
        format!("{:X}", file_size).len()
    }

    /// Print a line with optional offset
    pub fn print_line(offset: u64, hex_data: &str, show_offset: bool, hex_offset_length: usize) {
        if show_offset {
            println!(
                "{} : {}",
                Self::format_offset(offset, hex_offset_length),
                hex_data
            );
        } else {
            println!("{}", hex_data);
        }
    }

    /// Format a line with offset (returns a string instead of printing)
    pub fn format_line_with_offset(
        offset: u64,
        hex_data: &str,
        hex_offset_length: usize,
    ) -> String {
        format!(
            "{} : {}",
            Self::format_offset(offset, hex_offset_length),
            hex_data
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes_as_hex() {
        let bytes = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"
        let result = OutputFormatter::format_bytes_as_hex(&bytes, " ");
        assert_eq!(result, "48 65 6C 6C 6F");
    }

    #[test]
    fn test_format_bytes_with_different_separators() {
        let bytes = vec![0x00, 0xFF, 0x42];

        let with_space = OutputFormatter::format_bytes_as_hex(&bytes, " ");
        assert_eq!(with_space, "00 FF 42");

        let with_dash = OutputFormatter::format_bytes_as_hex(&bytes, "-");
        assert_eq!(with_dash, "00-FF-42");

        let no_separator = OutputFormatter::format_bytes_as_hex(&bytes, "");
        assert_eq!(no_separator, "00FF42");
    }

    #[test]
    fn test_format_offset() {
        let result = OutputFormatter::format_offset(0x1234, 6);
        assert_eq!(result, "001234h");
    }

    #[test]
    fn test_calculate_hex_offset_length() {
        assert_eq!(OutputFormatter::calculate_hex_offset_length(0xFF), 2);
        assert_eq!(OutputFormatter::calculate_hex_offset_length(0x1000), 4);
        assert_eq!(OutputFormatter::calculate_hex_offset_length(0x100000), 6);
    }
}
