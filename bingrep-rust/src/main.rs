use bingrep_rust::cli::Cli;
use bingrep_rust::config::Config;
use bingrep_rust::error::{BingrepError, Result};
use bingrep_rust::regex_processor::RegexProcessor;
use bingrep_rust::stream::FileProcessor;
use clap::Parser;
use std::fs::File;
use std::io::{Seek, SeekFrom};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check file path
    let file_path = match cli.file_path {
        Some(path) => path,
        None => {
            Cli::show_help();
            return Ok(());
        }
    };

    // Open file
    let mut file = File::open(&file_path)
        .map_err(BingrepError::Io)?;
    let file_size = file.metadata()
        .map_err(BingrepError::Io)?.len();

    // Create configuration and processor
    let config = Config::default();

    // Validate line width
    if !config.validate_width(cli.line_width) {
        return Err(BingrepError::InvalidWidth(cli.line_width));
    }

    let mut processor = FileProcessor::new(config);

    // Seek to starting position
    file.seek(SeekFrom::Start(cli.position))
        .map_err(BingrepError::Io)?;

    // Process file with or without regex
    if let Some(expression) = cli.expression {
        let regex = RegexProcessor::compile_pattern(&expression)?;
        processor.process_stream_by_regex(
            &mut file,
            &regex,
            cli.line_width,
            cli.limit,
            &cli.separator,
            !cli.hide_offset,
        )?;
    } else {
        processor.process_file_stream(
            &mut file,
            cli.line_width,
            cli.limit,
            &cli.separator,
            !cli.hide_offset,
            file_size,
        )?;
    }

    Ok(())
}