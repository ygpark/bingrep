use bingrep_rust::regex_processor::RegexProcessor;
use bingrep_rust::pcre2_processor::Pcre2Processor;
use std::time::{Duration, Instant};

fn main() {
    println!("=== 정규식 엔진 시간 복잡도 테스트 ===\n");

    // ReDoS 취약한 패턴들 테스트
    let evil_patterns = [
        ("Nested quantifiers", r"(a+)+b"),
        ("Alternation with quantifiers", r"(a|a)*b"),
        ("Catastrophic backtracking", r"(a*)*b"),
    ];

    // 매칭 실패하는 문자열들 (worst case)
    let medium_failing = "a".repeat(20) + "c";
    let long_failing = "a".repeat(50) + "c";
    let very_long_failing = "a".repeat(100) + "c";

    let evil_inputs = [
        ("Short failing", "aaaaaaaaac"),
        ("Medium failing", &medium_failing),
        ("Long failing", &long_failing),
        ("Very long failing", &very_long_failing),
    ];

    for (pattern_name, pattern) in &evil_patterns {
        println!("🧪 패턴: {} ({})", pattern_name, pattern);

        // Rust regex 컴파일 테스트
        let rust_compile_start = Instant::now();
        let rust_result = RegexProcessor::compile_pattern(pattern);
        let rust_compile_time = rust_compile_start.elapsed();

        // PCRE2 컴파일 테스트
        let pcre2_compile_start = Instant::now();
        let pcre2_result = Pcre2Processor::compile_pattern(pattern);
        let pcre2_compile_time = pcre2_compile_start.elapsed();

        println!("  📝 컴파일 시간:");
        println!("    Rust regex: {:?}", rust_compile_time);
        println!("    PCRE2: {:?}", pcre2_compile_time);

        match (rust_result, pcre2_result) {
            (Ok(rust_regex), Ok(pcre2_regex)) => {
                println!("  ⏱️  매칭 시간 (길이별):");

                for (input_name, input) in &evil_inputs {
                    println!("    {} ({} chars):", input_name, input.len());

                    // Rust regex 테스트
                    let rust_start = Instant::now();
                    let rust_matches: Vec<_> = rust_regex.find_iter(input.as_bytes()).collect();
                    let rust_duration = rust_start.elapsed();

                    // PCRE2 테스트
                    let pcre2_start = Instant::now();
                    let pcre2_matches = Pcre2Processor::find_matches(&pcre2_regex, input.as_bytes());
                    let pcre2_duration = pcre2_start.elapsed();

                    println!("      Rust regex: {:?} ({} matches)", rust_duration, rust_matches.len());
                    println!("      PCRE2: {:?} ({} matches)", pcre2_duration, pcre2_matches.len());

                    // 시간 복잡도 경고
                    if rust_duration > Duration::from_millis(10) {
                        println!("      ⚠️  Rust regex 느림!");
                    }
                    if pcre2_duration > Duration::from_millis(10) {
                        println!("      ⚠️  PCRE2 느림!");
                    }
                }
            }
            (Err(e), _) => println!("  ❌ Rust regex 컴파일 실패: {}", e),
            (_, Err(e)) => println!("  ❌ PCRE2 컴파일 실패: {}", e),
        }
        println!();
    }

    // 선형성 테스트
    println!("=== 선형 시간 복잡도 검증 ===");
    let linear_pattern = r"\x00\x01\x02";

    if let (Ok(rust_regex), Ok(pcre2_regex)) = (
        RegexProcessor::compile_pattern(linear_pattern),
        Pcre2Processor::compile_pattern(linear_pattern)
    ) {
        println!("패턴: {}", linear_pattern);

        let sizes = [1000, 10000, 100000, 1000000];

        for &size in &sizes {
            let test_data = generate_linear_test_data(size);

            let rust_start = Instant::now();
            let rust_matches: Vec<_> = rust_regex.find_iter(&test_data).collect();
            let rust_duration = rust_start.elapsed();

            let pcre2_start = Instant::now();
            let pcre2_matches = Pcre2Processor::find_matches(&pcre2_regex, &test_data);
            let pcre2_duration = pcre2_start.elapsed();

            println!("  📊 크기 {}: ", size);
            println!("    Rust regex: {:?} ({} matches, {:.2} ns/byte)",
                    rust_duration, rust_matches.len(),
                    rust_duration.as_nanos() as f64 / size as f64);
            println!("    PCRE2: {:?} ({} matches, {:.2} ns/byte)",
                    pcre2_duration, pcre2_matches.len(),
                    pcre2_duration.as_nanos() as f64 / size as f64);
        }
    }

    // 극한 케이스 테스트
    println!("\n=== 극한 케이스 테스트 ===");
    extreme_case_test();
}

fn generate_linear_test_data(size: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);
    let pattern = b"\x00\x01\x02";

    for i in 0..size {
        if i % 1000 == 0 && i + pattern.len() < size {
            data.extend_from_slice(pattern);
        } else {
            data.push((i % 256) as u8);
        }
    }
    data
}

fn extreme_case_test() {
    // 매우 긴 문자열에서 패턴 검색
    let huge_size = 10_000_000; // 10MB
    println!("🔬 극한 테스트: {}MB 데이터", huge_size / 1024 / 1024);

    let pattern = r"\xFF\xFE\xFD";

    if let (Ok(rust_regex), Ok(pcre2_regex)) = (
        RegexProcessor::compile_pattern(pattern),
        Pcre2Processor::compile_pattern(pattern)
    ) {
        let huge_data = generate_extreme_test_data(huge_size);

        println!("  🔍 Rust regex 테스트...");
        let rust_start = Instant::now();
        let rust_matches: Vec<_> = rust_regex.find_iter(&huge_data).collect();
        let rust_duration = rust_start.elapsed();

        println!("  🔧 PCRE2 테스트...");
        let pcre2_start = Instant::now();
        let pcre2_matches = Pcre2Processor::find_matches(&pcre2_regex, &huge_data);
        let pcre2_duration = pcre2_start.elapsed();

        println!("  📈 결과:");
        println!("    Rust regex: {:?} ({} matches, {:.2} MB/s)",
                rust_duration, rust_matches.len(),
                (huge_size as f64 / 1024.0 / 1024.0) / rust_duration.as_secs_f64());
        println!("    PCRE2: {:?} ({} matches, {:.2} MB/s)",
                pcre2_duration, pcre2_matches.len(),
                (huge_size as f64 / 1024.0 / 1024.0) / pcre2_duration.as_secs_f64());

        // 선형성 확인
        let rust_ns_per_byte = rust_duration.as_nanos() as f64 / huge_size as f64;
        let pcre2_ns_per_byte = pcre2_duration.as_nanos() as f64 / huge_size as f64;

        println!("  ⏳ 바이트당 처리 시간:");
        println!("    Rust regex: {:.3} ns/byte", rust_ns_per_byte);
        println!("    PCRE2: {:.3} ns/byte", pcre2_ns_per_byte);

        if rust_ns_per_byte < 10.0 {
            println!("    ✅ Rust regex: 선형 시간 복잡도 확인");
        } else {
            println!("    ⚠️  Rust regex: 예상보다 느림");
        }

        if pcre2_ns_per_byte < 10.0 {
            println!("    ✅ PCRE2: 선형 시간 복잡도 확인");
        } else {
            println!("    ⚠️  PCRE2: 예상보다 느림");
        }
    }
}

fn generate_extreme_test_data(size: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);
    let target_pattern = b"\xFF\xFE\xFD";

    // 몇 개의 실제 패턴 삽입
    let pattern_count = 10;
    let pattern_interval = size / pattern_count;

    for i in 0..size {
        if i % pattern_interval == 0 && i + target_pattern.len() < size {
            data.extend_from_slice(target_pattern);
        } else {
            // 의사 랜덤 데이터로 캐시 미스 유발
            data.push(((i.wrapping_mul(1103515245).wrapping_add(12345)) % 256) as u8);
        }
    }

    data
}