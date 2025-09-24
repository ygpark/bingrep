use clap::Parser;

#[derive(Parser)]
#[command(name = "bingrep")]
#[command(about = "바이너리 파일 정규표현식 검색 도구")]
#[command(version = "0.1.0")]
pub struct Cli {
    /// 입력 파일 경로
    pub file_path: Option<String>,

    /// 정규표현식 패턴 (예: -e "\x00\x00\x00\x01\x67")
    #[arg(short = 'e', long = "regex")]
    pub expression: Option<String>,

    /// 한 줄에 표시할 바이트 개수 (기본값: 16)
    #[arg(short = 'w', long = "width", default_value = "16")]
    pub line_width: usize,

    /// 출력할 라인 수 (0: 무제한)
    #[arg(short = 'n', long = "line", default_value = "0")]
    pub limit: usize,

    /// 시작 위치 (바이트 단위)
    #[arg(short = 's', long = "position", default_value = "0")]
    pub position: u64,

    /// 바이트 문자열 분리 기호
    #[arg(short = 't', long = "separator", default_value = " ")]
    pub separator: String,

    /// 오프셋 출력 안함
    #[arg(long = "hideoffset")]
    pub hide_offset: bool,
}

impl Cli {
    pub fn show_help() {
        let name = "bingrep-rust";
        println!();
        println!("{} <파일경로> [옵션]", name);
        println!();
        println!("옵션:");
        println!("  -h, --help              도움말 표시");
        println!("  -e, --regex <PATTERN>   정규표현식 패턴");
        println!("  -w, --width <N>         한 줄에 표시할 바이트 개수 (기본값: 16)");
        println!("  -n, --line <N>          출력할 라인 수 (0: 무제한, 기본값: 0)");
        println!("  -s, --position <N>      시작 위치 (바이트 단위, 기본값: 0)");
        println!("  -t, --separator <STR>   바이트 문자열 분리 기호 (기본값: \" \")");
        println!("      --hideoffset        오프셋 출력 안함");
        println!();
        println!("정규표현식:");
        println!();
        println!("  이 프로그램의 정규표현식은 Rust regex 라이브러리의 문법을 따릅니다.");
        println!("  예시) -e \"\\x00\\x00\\x00\\x01\\x67\"");
        println!();
        println!("Example 01 파일 내용을 HEX값으로 출력:");
        println!();
        println!("\t{} \"path_to_file.txt\"", name);
        println!("\t{} \"path_to_file.txt\" -n 10    (10줄만 출력)", name);
        println!();
        println!("Example 02 파일 내용을 정규표현식으로 검색:");
        println!();
        println!("\t{} \"path_to_file.txt\" -e \"\\x00\\x00\\x00\\x01\\x67\" -w 100", name);
        println!();
    }
}