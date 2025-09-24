use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;
use bingrep_rust::regex_processor::RegexProcessor;
use bingrep_rust::pcre2_processor::Pcre2Processor;

// Test data generators
fn generate_test_data(size: usize, pattern_density: f32) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);
    let pattern = b"\x00\x00\x00\x01\x67"; // H.264 SPS NAL unit start
    let pattern_interval = ((1.0 / pattern_density) as usize).max(pattern.len());

    let mut pos = 0;
    while pos < size {
        if pos % pattern_interval == 0 && pos + pattern.len() <= size {
            data.extend_from_slice(pattern);
            pos += pattern.len();
        } else {
            data.push((pos % 256) as u8);
            pos += 1;
        }
    }
    data
}

fn generate_large_binary_data(size: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);
    let patterns = [
        b"\x4D\x5A".as_slice(), // PE header
        b"\x7F\x45\x4C\x46".as_slice(), // ELF header
        b"\x89\x50\x4E\x47".as_slice(), // PNG signature
        b"\xFF\xD8\xFF".as_slice(), // JPEG signature
    ];

    for i in 0..size {
        if i % 1024 == 0 && i + 4 < size {
            let pattern = patterns[i % patterns.len()];
            if i + pattern.len() <= size {
                data.extend_from_slice(pattern);
                continue;
            }
        }
        data.push((i % 256) as u8);
    }
    data
}

// Benchmark simple pattern matching
fn bench_simple_pattern(c: &mut Criterion) {
    let data_sizes = [1024, 8192, 65536, 262144]; // 1KB to 256KB
    let pattern = "\\x00\\x00\\x00\\x01\\x67"; // H.264 SPS pattern

    let mut group = c.benchmark_group("simple_pattern_matching");

    for &size in &data_sizes {
        let data = generate_test_data(size, 0.01); // 1% pattern density

        group.throughput(Throughput::Bytes(size as u64));

        // Rust regex benchmark
        group.bench_with_input(BenchmarkId::new("rust_regex", size), &data, |b, data| {
            let regex = RegexProcessor::compile_pattern(pattern).unwrap();
            b.iter(|| {
                let matches: Vec<_> = regex.find_iter(black_box(data)).collect();
                black_box(matches);
            });
        });

        // PCRE2 benchmark
        group.bench_with_input(BenchmarkId::new("pcre2", size), &data, |b, data| {
            let regex = Pcre2Processor::compile_pattern(pattern).unwrap();
            b.iter(|| {
                let matches = Pcre2Processor::find_matches(&regex, black_box(data));
                black_box(matches);
            });
        });
    }

    group.finish();
}

// Benchmark complex patterns with quantifiers
fn bench_quantifier_patterns(c: &mut Criterion) {
    let data_sizes = [8192, 32768, 131072];
    let patterns = [
        ("simple_quantifier", "\\x00{4}"), // Exactly 4 zeros
        ("range_quantifier", "\\x00{2,6}"), // 2-6 zeros
        ("plus_quantifier", "\\xFF+"), // One or more 0xFF
        ("star_quantifier", "\\x20*\\x0A"), // Zero or more spaces before newline
    ];

    for &size in &data_sizes {
        let data = generate_large_binary_data(size);

        let mut group = c.benchmark_group(&format!("quantifier_patterns_{}KB", size / 1024));
        group.throughput(Throughput::Bytes(size as u64));

        for (name, pattern) in &patterns {
            // Rust regex benchmark
            if let Ok(regex) = RegexProcessor::compile_pattern(pattern) {
                group.bench_function(&format!("{}_rust_regex", name), |b| {
                    b.iter(|| {
                        let matches: Vec<_> = regex.find_iter(black_box(&data)).collect();
                        black_box(matches);
                    });
                });
            }

            // PCRE2 benchmark
            if let Ok(regex) = Pcre2Processor::compile_pattern(pattern) {
                group.bench_function(&format!("{}_pcre2", name), |b| {
                    b.iter(|| {
                        let matches = Pcre2Processor::find_matches(&regex, black_box(&data));
                        black_box(matches);
                    });
                });
            }
        }

        group.finish();
    }
}

// Benchmark worst-case scenarios
fn bench_worst_case_patterns(c: &mut Criterion) {
    let size = 65536; // 64KB
    let data = generate_test_data(size, 0.1); // 10% pattern density (high match rate)

    let patterns = [
        ("high_frequency", "\\x00"), // Very common byte
        ("alternation", "\\x00|\\x01|\\xFF"), // Multiple alternatives
        ("complex_hex", "\\x00\\x00\\x01[\\x67-\\x6F]"), // Complex pattern with character class
    ];

    let mut group = c.benchmark_group("worst_case_patterns");
    group.throughput(Throughput::Bytes(size as u64));
    group.measurement_time(Duration::from_secs(10)); // Longer measurement time for complex patterns

    for (name, pattern) in &patterns {
        // Only test patterns that both engines can handle
        let rust_result = RegexProcessor::compile_pattern(pattern);
        let pcre2_result = Pcre2Processor::compile_pattern(pattern);

        if rust_result.is_ok() && pcre2_result.is_ok() {
            let rust_regex = rust_result.unwrap();
            let pcre2_regex = pcre2_result.unwrap();

            group.bench_function(&format!("{}_rust_regex", name), |b| {
                b.iter(|| {
                    let matches: Vec<_> = rust_regex.find_iter(black_box(&data)).collect();
                    black_box(matches);
                });
            });

            group.bench_function(&format!("{}_pcre2", name), |b| {
                b.iter(|| {
                    let matches = Pcre2Processor::find_matches(&pcre2_regex, black_box(&data));
                    black_box(matches);
                });
            });
        }
    }

    group.finish();
}

// Benchmark compilation time
fn bench_compilation_time(c: &mut Criterion) {
    let patterns = [
        ("simple_hex", "\\x00\\x01\\x02\\x03"),
        ("quantifier", "\\x00{2,8}"),
        ("alternation", "\\x41|\\x42|\\x43|\\x44"),
        ("complex", "\\x00{2,4}\\x01[\\x67-\\x6F]\\x00*"),
    ];

    let mut group = c.benchmark_group("compilation_time");

    for (name, pattern) in &patterns {
        group.bench_function(&format!("{}_rust_regex_compile", name), |b| {
            b.iter(|| {
                let regex = RegexProcessor::compile_pattern(black_box(pattern));
                black_box(regex);
            });
        });

        group.bench_function(&format!("{}_pcre2_compile", name), |b| {
            b.iter(|| {
                let regex = Pcre2Processor::compile_pattern(black_box(pattern));
                black_box(regex);
            });
        });
    }

    group.finish();
}

// Memory usage comparison benchmark
fn bench_memory_usage(c: &mut Criterion) {
    let sizes = [1024, 8192, 65536, 262144];
    let pattern = "\\x00\\x00\\x01\\x67"; // H.264 SPS pattern

    let rust_regex = RegexProcessor::compile_pattern(pattern).unwrap();
    let pcre2_regex = Pcre2Processor::compile_pattern(pattern).unwrap();

    let mut group = c.benchmark_group("memory_efficiency");

    for &size in &sizes {
        let data = generate_test_data(size, 0.05); // 5% pattern density

        group.throughput(Throughput::Bytes(size as u64));

        // Measure memory-efficient processing
        group.bench_with_input(BenchmarkId::new("rust_regex_chunked", size), &data, |b, data| {
            let chunk_size = 4096;
            b.iter(|| {
                let mut total_matches = 0;
                for chunk in data.chunks(chunk_size) {
                    let matches: Vec<_> = rust_regex.find_iter(black_box(chunk)).collect();
                    total_matches += matches.len();
                }
                black_box(total_matches);
            });
        });

        group.bench_with_input(BenchmarkId::new("pcre2_chunked", size), &data, |b, data| {
            let chunk_size = 4096;
            b.iter(|| {
                let mut total_matches = 0;
                for chunk in data.chunks(chunk_size) {
                    let matches = Pcre2Processor::find_matches(&pcre2_regex, black_box(chunk));
                    total_matches += matches.len();
                }
                black_box(total_matches);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_pattern,
    bench_quantifier_patterns,
    bench_worst_case_patterns,
    bench_compilation_time,
    bench_memory_usage
);

criterion_main!(benches);