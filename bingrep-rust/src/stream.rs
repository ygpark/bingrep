use crate::buffer_manager::BufferManager;
use crate::config::Config;
use crate::error::Result;
use crate::output::OutputFormatter;
use regex::bytes::Regex;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// File processor for handling binary file searching and hex dump operations
pub struct FileProcessor {
    config: Config,
    buffer_manager: BufferManager,
}

impl FileProcessor {
    /// Create a new FileProcessor with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration settings for buffer sizes and limits
    pub fn new(config: Config) -> Self {
        let buffer_size = config.buffer_size;
        let max_extra_size = config.max_line_width.max(1024); // At least 1KB for extra buffer
        let buffer_manager = BufferManager::new(buffer_size, max_extra_size);

        Self {
            config,
            buffer_manager,
        }
    }

    /// Process file without regex - simple hex dump
    ///
    /// Reads a file and outputs its contents in hexadecimal format.
    ///
    /// # Arguments
    ///
    /// * `file` - File to read from
    /// * `width` - Number of bytes to display per line
    /// * `limit` - Maximum number of lines to output (0 for unlimited)
    /// * `separator` - String to separate hex bytes
    /// * `show_offset` - Whether to display offset values
    /// * `file_size` - Total size of the file for offset formatting
    pub fn process_file_stream(
        &mut self,
        file: &mut File,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
        file_size: u64,
    ) -> Result<()> {
        let mut pos = file.stream_position()?;
        let mut line = 0;
        let hex_offset_length = OutputFormatter::calculate_hex_offset_length(file_size);

        // Get a reusable buffer of the right size
        let buffer = self.buffer_manager.get_extra_buffer(width);

        loop {
            let bytes_read = file.read(&mut buffer[..width])?;
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
    ///
    /// Searches a file for regex pattern matches and outputs matching regions.
    ///
    /// # Arguments
    ///
    /// * `file` - File to search in
    /// * `regex` - Compiled regex pattern to search for
    /// * `width` - Number of bytes to display per match
    /// * `limit` - Maximum number of matches to output (0 for unlimited)
    /// * `separator` - String to separate hex bytes
    /// * `show_offset` - Whether to display offset values
    pub fn process_stream_by_regex(
        &mut self,
        file: &mut File,
        regex: &Regex,
        width: usize,
        limit: usize,
        separator: &str,
        show_offset: bool,
    ) -> Result<()> {
        let buffer_size = self.config.get_buffer_size(width);
        let buffer_padding = self.config.buffer_padding;

        let mut line = 0;
        let mut last_hit_pos: i64 = -1;

        let file_size = file.metadata()?.len();
        let hex_offset_length = OutputFormatter::calculate_hex_offset_length(file_size);

        loop {
            let start_offset = file.stream_position()?;
            let bytes_read = self.buffer_manager.read_into_main(file)?;

            if bytes_read == 0 {
                break;
            }

            // Process regex matches directly without collecting into vector
            let buffer_slice = self.buffer_manager.get_main_slice(0, bytes_read);
            let mut matches_to_process = Vec::new();

            // Only collect match positions that we actually need to process
            for mat in regex.find_iter(buffer_slice) {
                let match_start = mat.start();
                let new_hit_pos = start_offset + match_start as u64;

                // Skip duplicates early
                if new_hit_pos as i64 > last_hit_pos {
                    matches_to_process.push(match_start);
                    // Limit collection for memory efficiency
                    if limit > 0 && matches_to_process.len() >= limit - line {
                        break;
                    }
                }
            }

            for match_start in matches_to_process {
                let new_hit_pos = start_offset + match_start as u64;

                // Prevent duplicates
                if new_hit_pos as i64 <= last_hit_pos {
                    continue;
                }

                // Handle buffer overflow - seek to match position if needed
                if match_start + width > bytes_read && bytes_read == buffer_size {
                    file.seek(SeekFrom::Start(new_hit_pos))?;
                    last_hit_pos = new_hit_pos as i64;
                    break;
                }

                line += 1;

                // Read width bytes from match position
                let hex_string = self.read_match_data(
                    file,
                    match_start,
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
        &mut self,
        file: &mut File,
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
            let current_pos = file.stream_position()?;
            file.seek(SeekFrom::Start(start_offset + end_pos as u64))?;

            let extra_needed = width - actual_width;
            let extra_read = self.buffer_manager.read_into_extra(file, extra_needed)?;

            // Combine data using buffer manager
            let combined_data = self.buffer_manager.combine_buffers(
                match_start,
                end_pos,
                extra_read,
            );

            file.seek(SeekFrom::Start(current_pos))?;

            Ok(OutputFormatter::format_bytes_as_hex(combined_data, separator))
        } else {
            let main_slice = self.buffer_manager.get_main_slice(match_start, end_pos);
            Ok(OutputFormatter::format_bytes_as_hex(main_slice, separator))
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
        let mut processor = FileProcessor::new(config);

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