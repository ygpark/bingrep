use bingrep_rust::benchmark_utils::BenchmarkUtils;
use bingrep_rust::pcre2_processor::Pcre2Processor;
use bingrep_rust::regex_processor::RegexProcessor;
use std::time::Instant;

fn main() {
    println!("=== Rust regex vs PCRE2 성능 벤치마크 ===\n");

    // 테스트 패턴들
    let patterns = [
        ("H.264 SPS", "\\x00\\x00\\x00\\x01\\x67"),
        ("PE Header", "\\x4D\\x5A"),
        ("Quantifier", "\\x00{4}"),
        ("Complex", "\\x00{2,4}\\x01"),
    ];

    // 테스트 데이터 크기들
    let sizes = [1024, 8192, 65536];

    for (pattern_name, pattern) in &patterns {
        println!("🔍 패턴: {} ({})", pattern_name, pattern);

        for &size in &sizes {
            println!(
                "  📊 데이터 크기: {} bytes ({:.1} KB)",
                size,
                size as f64 / 1024.0
            );

            // 테스트 데이터 생성
            let test_data = BenchmarkUtils::generate_binary_file_data(size);

            // Rust regex 컴파일 및 테스트
            let rust_compile_start = Instant::now();
            let rust_result = RegexProcessor::compile_pattern(pattern);
            let rust_compile_time = rust_compile_start.elapsed();

            // PCRE2 컴파일 및 테스트
            let pcre2_compile_start = Instant::now();
            let pcre2_result = Pcre2Processor::compile_pattern(pattern);
            let pcre2_compile_time = pcre2_compile_start.elapsed();

            match (rust_result, pcre2_result) {
                (Ok(rust_regex), Ok(pcre2_regex)) => {
                    // 성능 비교 실행
                    let comparison = BenchmarkUtils::compare_engines(
                        "Rust regex",
                        "PCRE2",
                        size,
                        10, // 10번 반복
                        || {
                            let matches: Vec<_> = rust_regex.find_iter(&test_data).collect();
                            matches.len()
                        },
                        || {
                            let matches = Pcre2Processor::find_matches(&pcre2_regex, &test_data)
                                .unwrap_or_default();
                            matches.len()
                        },
                    );

                    println!("    ⚡ Rust regex:");
                    println!("      - 컴파일: {:?}", rust_compile_time);
                    println!(
                        "      - 평균 실행: {:?}",
                        comparison.engine1_stats.avg_duration
                    );
                    println!(
                        "      - 처리량: {:.2} MB/s",
                        comparison.engine1_stats.throughput_mb_per_sec
                    );

                    println!("    🔧 PCRE2:");
                    println!("      - 컴파일: {:?}", pcre2_compile_time);
                    println!(
                        "      - 평균 실행: {:?}",
                        comparison.engine2_stats.avg_duration
                    );
                    println!(
                        "      - 처리량: {:.2} MB/s",
                        comparison.engine2_stats.throughput_mb_per_sec
                    );

                    // 승자 표시
                    let rust_total = rust_compile_time + comparison.engine1_stats.avg_duration;
                    let pcre2_total = pcre2_compile_time + comparison.engine2_stats.avg_duration;

                    if rust_total < pcre2_total {
                        let speedup = pcre2_total.as_secs_f64() / rust_total.as_secs_f64();
                        println!("    🏆 Rust regex가 {:.2}x 빠름", speedup);
                    } else {
                        let speedup = rust_total.as_secs_f64() / pcre2_total.as_secs_f64();
                        println!("    🏆 PCRE2가 {:.2}x 빠름", speedup);
                    }

                    println!();
                }
                (Err(e1), _) => println!("    ❌ Rust regex 컴파일 실패: {}", e1),
                (_, Err(e2)) => println!("    ❌ PCRE2 컴파일 실패: {}", e2),
            }
        }
        println!();
    }

    // 메모리 효율성 테스트
    println!("=== 메모리 효율성 테스트 ===");
    let large_data = BenchmarkUtils::generate_binary_file_data(1024 * 1024); // 1MB
    let pattern = "\\x00\\x00\\x01\\x67";

    if let (Ok(rust_regex), Ok(pcre2_regex)) = (
        RegexProcessor::compile_pattern(pattern),
        Pcre2Processor::compile_pattern(pattern),
    ) {
        let chunk_size = 4096;
        let chunks: Vec<_> = large_data.chunks(chunk_size).collect();

        println!(
            "큰 데이터 ({} MB)를 {}바이트 청크로 나누어 처리:",
            large_data.len() / (1024 * 1024),
            chunk_size
        );

        // Rust regex 청크 처리
        let rust_chunk_start = Instant::now();
        let mut rust_total_matches = 0;
        for chunk in &chunks {
            rust_total_matches += rust_regex.find_iter(chunk).count();
        }
        let rust_chunk_time = rust_chunk_start.elapsed();

        // PCRE2 청크 처리
        let pcre2_chunk_start = Instant::now();
        let mut pcre2_total_matches = 0;
        for chunk in &chunks {
            pcre2_total_matches += Pcre2Processor::find_matches(&pcre2_regex, chunk)
                .unwrap_or_default()
                .len();
        }
        let pcre2_chunk_time = pcre2_chunk_start.elapsed();

        println!(
            "🔍 Rust regex: {} 매치, {:?}",
            rust_total_matches, rust_chunk_time
        );
        println!(
            "🔧 PCRE2: {} 매치, {:?}",
            pcre2_total_matches, pcre2_chunk_time
        );

        if rust_chunk_time < pcre2_chunk_time {
            let speedup = pcre2_chunk_time.as_secs_f64() / rust_chunk_time.as_secs_f64();
            println!("🏆 청크 처리에서 Rust regex가 {:.2}x 빠름", speedup);
        } else {
            let speedup = rust_chunk_time.as_secs_f64() / pcre2_chunk_time.as_secs_f64();
            println!("🏆 청크 처리에서 PCRE2가 {:.2}x 빠름", speedup);
        }
    }

    println!("\n=== 벤치마크 완료 ===");
}
