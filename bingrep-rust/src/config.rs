/// Configuration constants and defaults for bingrep
#[derive(Debug, Clone)]
pub struct Config {
    pub buffer_size: usize,
    pub buffer_padding: usize,
    pub max_line_width: usize,
    pub min_line_width: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            buffer_size: 64 * 1024,        // 64KB for better performance
            buffer_padding: 1024,          // To handle patterns across buffer boundaries
            max_line_width: 8192,          // Maximum bytes per line
            min_line_width: 1,             // Minimum bytes per line
        }
    }
}

impl Config {
    pub fn validate_width(&self, width: usize) -> bool {
        width >= self.min_line_width && width <= self.max_line_width
    }

    pub fn get_buffer_size(&self) -> usize {
        // Use smaller buffer size for width < 1024 to avoid memory waste
        if self.max_line_width < 1024 {
            4096
        } else {
            self.buffer_size
        }
    }
}