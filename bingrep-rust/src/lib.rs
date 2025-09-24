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

/// Re-export commonly used types
pub use std::io;
pub use regex::bytes::Regex;