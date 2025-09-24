use std::io::{Read, Result};

/// Buffer manager for efficient memory reuse during file processing
pub struct BufferManager {
    main_buffer: Vec<u8>,
    extra_buffer: Vec<u8>,
    temp_buffer: Vec<u8>,
}

impl BufferManager {
    pub fn new(buffer_size: usize, max_extra_size: usize) -> Self {
        Self {
            main_buffer: vec![0u8; buffer_size],
            extra_buffer: vec![0u8; max_extra_size],
            temp_buffer: Vec::new(),
        }
    }

    /// Get a reference to the main buffer
    pub fn get_main_buffer(&mut self) -> &mut Vec<u8> {
        &mut self.main_buffer
    }

    /// Get a reference to the extra buffer, resizing if needed
    pub fn get_extra_buffer(&mut self, needed_size: usize) -> &mut Vec<u8> {
        if self.extra_buffer.len() < needed_size {
            self.extra_buffer.resize(needed_size, 0);
        }
        &mut self.extra_buffer
    }

    /// Read data into main buffer
    pub fn read_into_main<R: Read>(&mut self, reader: &mut R) -> Result<usize> {
        reader.read(&mut self.main_buffer)
    }

    /// Read data into extra buffer
    pub fn read_into_extra<R: Read>(&mut self, reader: &mut R, size: usize) -> Result<usize> {
        let buffer = self.get_extra_buffer(size);
        let bytes_read = reader.read(&mut buffer[..size])?;
        Ok(bytes_read)
    }

    /// Combine data from main buffer and extra buffer into temp buffer
    pub fn combine_buffers(
        &mut self,
        main_start: usize,
        main_end: usize,
        extra_size: usize,
    ) -> &[u8] {
        self.temp_buffer.clear();
        self.temp_buffer.extend_from_slice(&self.main_buffer[main_start..main_end]);
        self.temp_buffer.extend_from_slice(&self.extra_buffer[..extra_size]);
        &self.temp_buffer
    }

    /// Get slice from main buffer
    pub fn get_main_slice(&self, start: usize, end: usize) -> &[u8] {
        &self.main_buffer[start..end]
    }

    /// Get slice from extra buffer
    pub fn get_extra_slice(&self, size: usize) -> &[u8] {
        &self.extra_buffer[..size]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_buffer_manager_creation() {
        let manager = BufferManager::new(1024, 512);
        assert_eq!(manager.main_buffer.len(), 1024);
        assert_eq!(manager.extra_buffer.len(), 512);
    }

    #[test]
    fn test_read_into_main() {
        let mut manager = BufferManager::new(10, 5);
        let data = b"Hello World";
        let mut cursor = Cursor::new(data);

        let bytes_read = manager.read_into_main(&mut cursor).unwrap();
        assert_eq!(bytes_read, 10); // Only 10 bytes fit in buffer
        assert_eq!(&manager.main_buffer[..bytes_read], b"Hello Worl");
    }

    #[test]
    fn test_extra_buffer_resize() {
        let mut manager = BufferManager::new(10, 5);

        // Request larger buffer than initial size
        let buffer = manager.get_extra_buffer(20);
        assert_eq!(buffer.len(), 20);
    }

    #[test]
    fn test_combine_buffers() {
        let mut manager = BufferManager::new(10, 10);
        manager.main_buffer[0..5].copy_from_slice(b"Hello");
        manager.extra_buffer[0..5].copy_from_slice(b"World");

        let combined = manager.combine_buffers(0, 5, 5);
        assert_eq!(combined, b"HelloWorld");
    }
}