use bingrep_rust::cli::Cli;
use bingrep_rust::config::Config;
use bingrep_rust::error::Result;
use bingrep_rust::regex_processor::RegexProcessor;
use bingrep_rust::stream::FileProcessor;
use clap::Parser;
use std::fs::File;
use std::io::{Seek, SeekFrom};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check file path
    let file_path = match &cli.file_path {
        Some(path) => path.clone(),
        None => {
            // Clap will automatically show help when no file path is provided
            eprintln!("Error: 파일 경로가 필요합니다.\n");
            eprintln!("사용법: bingrep-rust <파일경로> [옵션]");
            eprintln!("도움말: bingrep-rust --help");
            return Ok(());
        }
    };

    // Open file
    let mut file = File::open(&file_path)?;
    let file_size = file.metadata()?.len();

    // Create configuration and validate CLI parameters
    let config = Config::default();
    config.validate_cli(&cli)?;

    let mut processor = FileProcessor::new(config);

    // Seek to starting position
    file.seek(SeekFrom::Start(cli.position))?;

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