use crate::config::Config;
use crate::error::Result;
use crate::output::OutputFormatter;
use regex::bytes::Regex;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub struct FileProcessor {
    config: Config,
}

impl FileProcessor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Process file without regex - simple hex dump
    pub fn process_file_stream(
        &self,
        file: &mut File,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
        file_size: u64,
    ) -> Result<()> {
        let mut buffer = vec![0u8; width];
        let mut pos = file.stream_position()?;
        let mut line = 0;
        let hex_offset_length = OutputFormatter::calculate_hex_offset_length(file_size);

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            line += 1;

            let hex_string = OutputFormatter::format_bytes_as_hex(&buffer[..bytes_read], separator);
            OutputFormatter::print_line(pos, &hex_string, show_offset, hex_offset_length);

            pos += bytes_read as u64;

            // Check line limit
            if limit > 0 && line >= limit {
                break;
            }
        }

        Ok(())
    }

    /// Process file with regex pattern matching
    pub fn process_stream_by_regex(
        &self,
        file: &mut File,
        regex: &Regex,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
    ) -> Result<()> {
        let buffer_size = self.config.get_buffer_size();
        let buffer_padding = self.config.buffer_padding;

        let mut buffer = vec![0u8; buffer_size];
        let mut line = 0;
        let mut last_hit_pos: i64 = -1;

        let file_size = file.metadata()?.len();
        let hex_offset_length = OutputFormatter::calculate_hex_offset_length(file_size);

        loop {
            let start_offset = file.stream_position()?;
            let bytes_read = file.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            // Find regex matches
            let matches: Vec<_> = regex.find_iter(&buffer[..bytes_read]).collect();

            for mat in matches {
                let new_hit_pos = start_offset + mat.start() as u64;

                // Prevent duplicates
                if new_hit_pos as i64 <= last_hit_pos {
                    continue;
                }

                // Handle buffer overflow - seek to match position if needed
                if mat.start() + width > bytes_read && bytes_read == buffer_size {
                    file.seek(SeekFrom::Start(new_hit_pos))?;
                    last_hit_pos = new_hit_pos as i64;
                    break;
                }

                line += 1;

                // Read width bytes from match position
                let hex_string = self.read_match_data(
                    file,
                    &buffer,
                    mat.start(),
                    width,
                    bytes_read,
                    start_offset,
                    separator,
                )?;

                OutputFormatter::print_line(new_hit_pos, &hex_string, show_offset, hex_offset_length);
                last_hit_pos = new_hit_pos as i64;

                // Check line limit
                if limit > 0 && line >= limit {
                    return Ok(());
                }
            }

            // Read next buffer with overlap to handle patterns spanning boundaries
            if bytes_read == buffer_size {
                let new_pos = file
                    .stream_position()?
                    .saturating_sub(buffer_padding as u64);
                file.seek(SeekFrom::Start(new_pos))?;
            }
        }

        Ok(())
    }

    /// Read match data, handling cases where width extends beyond buffer
    fn read_match_data(
        &self,
        file: &mut File,
        buffer: &[u8],
        match_start: usize,
        width: usize,
        bytes_read: usize,
        start_offset: u64,
        separator: &str,
    ) -> Result<String> {
        let end_pos = std::cmp::min(match_start + width, bytes_read);
        let actual_width = end_pos - match_start;

        if actual_width < width && match_start + width > bytes_read {
            // Need to read additional data from file
            let mut result_bytes = buffer[match_start..end_pos].to_vec();

            // Read remaining data
            let current_pos = file.stream_position()?;
            file.seek(SeekFrom::Start(start_offset + end_pos as u64))?;
            let mut extra_buffer = vec![0u8; width - actual_width];
            let extra_read = file.read(&mut extra_buffer)?;
            result_bytes.extend_from_slice(&extra_buffer[..extra_read]);
            file.seek(SeekFrom::Start(current_pos))?;

            Ok(OutputFormatter::format_bytes_as_hex(&result_bytes, separator))
        } else {
            Ok(OutputFormatter::format_bytes_as_hex(
                &buffer[match_start..end_pos],
                separator,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_processor_creation() {
        let config = Config::default();
        let processor = FileProcessor::new(config);
        assert_eq!(processor.config.buffer_size, 64 * 1024);
    }

    #[test]
    fn test_process_file_stream() -> Result<()> {
        let config = Config::default();
        let processor = FileProcessor::new(config);

        // Create a temporary file with test data
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello World!").unwrap();
        temp_file.seek(SeekFrom::Start(0)).unwrap();

        let mut file = temp_file.reopen().unwrap();
        let file_size = file.metadata()?.len();

        // This would normally print, but in tests we just verify it doesn't error
        let result = processor.process_file_stream(&mut file, 16, 1, " ", false, file_size);
        assert!(result.is_ok());

        Ok(())
    }
}