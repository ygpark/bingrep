use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance measurement utilities for regex engine comparison
pub struct BenchmarkUtils;

impl BenchmarkUtils {
    /// Generate test data with controlled pattern distribution
    pub fn generate_test_data(size: usize, patterns: &[&[u8]], density: f32) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);
        let pattern_interval = ((1.0 / density) as usize).max(10);

        let mut pos = 0;
        let mut pattern_idx = 0;

        while pos < size {
            if pos % pattern_interval == 0 && pos + patterns[pattern_idx].len() <= size {
                data.extend_from_slice(patterns[pattern_idx]);
                pos += patterns[pattern_idx].len();
                pattern_idx = (pattern_idx + 1) % patterns.len();
            } else {
                // Generate pseudo-random but deterministic data
                data.push(((pos * 17 + 42) % 256) as u8);
                pos += 1;
            }
        }

        data
    }

    /// Generate realistic binary file data
    pub fn generate_binary_file_data(size: usize) -> Vec<u8> {
        let headers = [
            b"\x4D\x5A".as_slice(),                         // PE
            b"\x7F\x45\x4C\x46".as_slice(),                 // ELF
            b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A".as_slice(), // PNG
            b"\xFF\xD8\xFF\xE0".as_slice(),                 // JPEG
            b"\x00\x00\x00\x01\x67".as_slice(),             // H.264 SPS
            b"\x00\x00\x00\x01\x68".as_slice(),             // H.264 PPS
        ];

        Self::generate_test_data(size, &headers, 0.001) // 0.1% header density
    }

    /// Measure execution time and throughput
    pub fn measure_performance<F, R>(data_size: usize, operation: F) -> PerformanceResult
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let _result = operation();
        let duration = start.elapsed();

        PerformanceResult {
            duration,
            throughput_mb_per_sec: (data_size as f64) / (1024.0 * 1024.0) / duration.as_secs_f64(),
            data_size,
        }
    }

    /// Run multiple iterations and collect statistics
    pub fn benchmark_with_iterations<F, R>(
        iterations: usize,
        data_size: usize,
        mut operation: F,
    ) -> BenchmarkStatistics
    where
        F: FnMut() -> R,
    {
        let mut durations = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let start = Instant::now();
            let _result = operation();
            durations.push(start.elapsed());
        }

        BenchmarkStatistics::from_durations(durations, data_size)
    }

    /// Compare two regex engines side by side
    pub fn compare_engines<F1, F2, R1, R2>(
        engine1_name: &str,
        engine2_name: &str,
        data_size: usize,
        iterations: usize,
        mut engine1_op: F1,
        mut engine2_op: F2,
    ) -> EngineComparison
    where
        F1: FnMut() -> R1,
        F2: FnMut() -> R2,
    {
        let engine1_stats = Self::benchmark_with_iterations(iterations, data_size, &mut engine1_op);
        let engine2_stats = Self::benchmark_with_iterations(iterations, data_size, &mut engine2_op);

        EngineComparison {
            engine1_name: engine1_name.to_string(),
            engine2_name: engine2_name.to_string(),
            engine1_stats,
            engine2_stats,
        }
    }

    /// Create test patterns for different scenarios
    pub fn get_test_patterns() -> HashMap<&'static str, &'static str> {
        let mut patterns = HashMap::new();
        patterns.insert("h264_sps", "\\x00\\x00\\x00\\x01\\x67");
        patterns.insert("h264_pps", "\\x00\\x00\\x00\\x01\\x68");
        patterns.insert("pe_header", "\\x4D\\x5A");
        patterns.insert("elf_header", "\\x7F\\x45\\x4C\\x46");
        patterns.insert("png_signature", "\\x89\\x50\\x4E\\x47");
        patterns.insert("jpeg_signature", "\\xFF\\xD8\\xFF");
        patterns.insert("null_sequence", "\\x00{4}");
        patterns.insert("range_quantifier", "\\x00{2,6}");
        patterns.insert("plus_quantifier", "\\xFF+");
        patterns.insert("alternation", "\\x41|\\x42|\\x43");
        patterns
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceResult {
    pub duration: Duration,
    pub throughput_mb_per_sec: f64,
    pub data_size: usize,
}

#[derive(Debug, Clone)]
pub struct BenchmarkStatistics {
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub avg_duration: Duration,
    pub median_duration: Duration,
    pub throughput_mb_per_sec: f64,
    pub data_size: usize,
    pub iterations: usize,
}

impl BenchmarkStatistics {
    fn from_durations(mut durations: Vec<Duration>, data_size: usize) -> Self {
        durations.sort();

        let iterations = durations.len();
        let min_duration = durations[0];
        let max_duration = durations[iterations - 1];
        let median_duration = durations[iterations / 2];

        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / iterations as u32;

        let throughput_mb_per_sec =
            (data_size as f64) / (1024.0 * 1024.0) / avg_duration.as_secs_f64();

        Self {
            min_duration,
            max_duration,
            avg_duration,
            median_duration,
            throughput_mb_per_sec,
            data_size,
            iterations,
        }
    }

    pub fn print_summary(&self, name: &str) {
        println!("\n{} Performance Summary:", name);
        println!(
            "  Data size: {} bytes ({:.2} MB)",
            self.data_size,
            self.data_size as f64 / (1024.0 * 1024.0)
        );
        println!("  Iterations: {}", self.iterations);
        println!("  Min time: {:?}", self.min_duration);
        println!("  Max time: {:?}", self.max_duration);
        println!("  Avg time: {:?}", self.avg_duration);
        println!("  Median time: {:?}", self.median_duration);
        println!("  Throughput: {:.2} MB/s", self.throughput_mb_per_sec);
    }
}

#[derive(Debug)]
pub struct EngineComparison {
    pub engine1_name: String,
    pub engine2_name: String,
    pub engine1_stats: BenchmarkStatistics,
    pub engine2_stats: BenchmarkStatistics,
}

impl EngineComparison {
    pub fn print_comparison(&self) {
        self.engine1_stats.print_summary(&self.engine1_name);
        self.engine2_stats.print_summary(&self.engine2_name);

        println!("\n=== Comparison ===");
        let speedup = self.engine2_stats.avg_duration.as_secs_f64()
            / self.engine1_stats.avg_duration.as_secs_f64();
        if speedup > 1.0 {
            println!(
                "{} is {:.2}x faster than {}",
                self.engine1_name, speedup, self.engine2_name
            );
        } else {
            println!(
                "{} is {:.2}x faster than {}",
                self.engine2_name,
                1.0 / speedup,
                self.engine1_name
            );
        }

        let throughput_ratio =
            self.engine1_stats.throughput_mb_per_sec / self.engine2_stats.throughput_mb_per_sec;
        println!(
            "Throughput ratio ({}/{}): {:.2}",
            self.engine1_name, self.engine2_name, throughput_ratio
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_test_data() {
        let patterns = [b"AB".as_slice(), b"CD".as_slice()];
        let data = BenchmarkUtils::generate_test_data(100, &patterns, 0.1);
        assert_eq!(data.len(), 100);

        // Should contain at least some pattern data
        let data_str = String::from_utf8_lossy(&data);
        assert!(data_str.contains("AB") || data_str.contains("CD"));
    }

    #[test]
    fn test_measure_performance() {
        let result = BenchmarkUtils::measure_performance(1000, || {
            std::thread::sleep(Duration::from_millis(1));
        });

        assert_eq!(result.data_size, 1000);
        assert!(result.duration >= Duration::from_millis(1));
    }

    #[test]
    fn test_benchmark_statistics() {
        let durations = vec![
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(15),
        ];
        let stats = BenchmarkStatistics::from_durations(durations, 1000);

        assert_eq!(stats.iterations, 3);
        assert_eq!(stats.min_duration, Duration::from_millis(10));
        assert_eq!(stats.max_duration, Duration::from_millis(20));
        assert_eq!(stats.median_duration, Duration::from_millis(15));
    }
}
