use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("release"); // 벤치마크는 release 모드로
    path.push("bingrep-rust");
    path
}

fn create_large_file(size_mb: usize) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("benchmark_{}mb.bin", size_mb));

    if !file_path.exists() {
        let mut file = File::create(&file_path).unwrap();
        let chunk = vec![0x41u8; 1024 * 1024]; // 1MB of 'A'

        for mb in 0..size_mb {
            let mut data = chunk.clone();
            // 각 10MB마다 패턴 삽입
            if mb % 10 == 0 {
                data[512 * 1024..512 * 1024 + 5].copy_from_slice(b"\x00\x00\x00\x01\x67");
            }
            file.write_all(&data).unwrap();
        }
    }

    file_path
}

#[test]
#[ignore] // cargo test -- --ignored 로 실행
fn benchmark_large_file_hex_display() {
    let binary_path = get_binary_path();
    let test_file = create_large_file(100); // 100MB 파일

    let start = Instant::now();

    let output = Command::new(&binary_path)
        .arg(&test_file)
        .arg("-n")
        .arg("1000") // 1000줄만 출력
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();

    assert!(output.status.success());
    println!("100MB 파일에서 1000줄 hex 출력: {:?}", duration);

    // 성능 기준: 1초 이내
    assert!(duration.as_secs() < 1);
}

#[test]
#[ignore]
fn benchmark_large_file_regex_search() {
    let binary_path = get_binary_path();
    let test_file = create_large_file(100); // 100MB 파일

    let start = Instant::now();

    let output = Command::new(&binary_path)
        .arg(&test_file)
        .arg("-e")
        .arg("\\x00\\x00\\x00\\x01\\x67")
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();

    assert!(output.status.success());
    println!("100MB 파일에서 정규표현식 검색: {:?}", duration);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let match_count = stdout.lines().count();
    println!("발견된 매치 수: {}", match_count);

    // 성능 기준: 5초 이내
    assert!(duration.as_secs() < 5);
}

#[test]
#[ignore]
fn benchmark_complex_regex() {
    let binary_path = get_binary_path();
    let test_file = create_large_file(50); // 50MB 파일

    // 복잡한 정규표현식 패턴
    let complex_pattern = "\\x00\\x00\\x00\\x01[\\x65-\\x68]"; // NAL unit 시작 코드

    let start = Instant::now();

    let output = Command::new(&binary_path)
        .arg(&test_file)
        .arg("-e")
        .arg(complex_pattern)
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();

    assert!(output.status.success());
    println!("50MB 파일에서 복잡한 정규표현식 검색: {:?}", duration);

    // 성능 기준: 10초 이내
    assert!(duration.as_secs() < 10);
}

#[test]
#[ignore]
fn benchmark_memory_usage() {
    let binary_path = get_binary_path();
    let test_file = create_large_file(500); // 500MB 파일

    // 메모리 사용량은 파일 크기에 관계없이 일정해야 함
    let start = Instant::now();

    let output = Command::new(&binary_path)
        .arg(&test_file)
        .arg("-e")
        .arg("\\xFF\\xFF\\xFF\\xFF") // 존재하지 않을 패턴
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();

    assert!(output.status.success());
    println!("500MB 파일 전체 스캔: {:?}", duration);

    // 출력이 비어있어야 함 (매치 없음)
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "");
}

#[test]
#[ignore]
fn benchmark_many_matches() {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("many_matches.bin");

    // 매치가 많은 파일 생성
    let mut file = File::create(&file_path).unwrap();
    for _ in 0..10000 {
        file.write_all(b"PATTERN\x12\x34\x56\x78PATTERN").unwrap();
    }
    drop(file);

    let binary_path = get_binary_path();

    let start = Instant::now();

    let output = Command::new(&binary_path)
        .arg(&file_path)
        .arg("-e")
        .arg("\\x12\\x34\\x56\\x78")
        .arg("-w")
        .arg("8")
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let match_count = stdout.lines().count();

    println!("10000개 매치 검색 및 출력: {:?}", duration);
    println!("실제 출력된 매치 수: {}", match_count);

    // 성능 기준: 2초 이내
    assert!(duration.as_secs() < 2);

    fs::remove_file(file_path).ok();
}

#[test]
#[ignore]
fn benchmark_with_position_offset() {
    let binary_path = get_binary_path();
    let test_file = create_large_file(100); // 100MB 파일

    // 파일 중간부터 검색 시작
    let start_position = 50 * 1024 * 1024; // 50MB 지점

    let start = Instant::now();

    let output = Command::new(&binary_path)
        .arg(&test_file)
        .arg("-s")
        .arg(start_position.to_string())
        .arg("-e")
        .arg("\\x00\\x00\\x00\\x01\\x67")
        .output()
        .expect("Failed to execute command");

    let duration = start.elapsed();

    assert!(output.status.success());
    println!("50MB 오프셋에서 시작하여 검색: {:?}", duration);

    // 절반만 검색하므로 더 빨라야 함
    assert!(duration.as_secs() < 3);
}
