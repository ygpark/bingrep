//! # bingrep-rust
//!
//! A binary file regular expression search tool written in Rust.
//!
//! This library provides functionality for searching binary files using regular expressions
//! with support for hexadecimal escape sequences (\\xHH format).
//!
//! ## Main Components
//!
//! * `cli` - Command-line interface handling
//! * `config` - Configuration and validation
//! * `regex_processor` - Regular expression compilation and processing
//! * `pcre2_processor` - Alternative PCRE2 regex engine support
//! * `stream` - File streaming and pattern matching
//! * `buffer_manager` - Efficient buffer management for large files
//! * `output` - Hexadecimal output formatting
//! * `error` - Error types and handling
//!
//! ## Example Usage
//!
//! ```no_run
//! use bingrep_rust::{Config, RegexProcessor, FileProcessor};
//! use std::fs::File;
//!
//! let config = Config::default();
//! let mut processor = FileProcessor::new(config);
//! let regex = RegexProcessor::compile_pattern("\\x00\\x01").unwrap();
//! // Process file with regex...
//! ```

pub mod cli;
pub mod config;
pub mod regex_processor;
pub mod pcre2_processor;
pub mod output;
pub mod stream;
pub mod error;
pub mod buffer_manager;
pub mod benchmark_utils;

pub use cli::Cli;
pub use config::Config;
pub use error::{BingrepError, Result};
pub use regex_processor::RegexProcessor;
pub use stream::FileProcessor;

/// Re-export commonly used types
pub use std::io;
pub use regex::bytes::Regex;