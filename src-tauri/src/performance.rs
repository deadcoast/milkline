// Performance monitoring utilities
use std::time::{Duration, Instant};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

/// Performance metrics for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub startup_time_ms: Option<u64>,
    pub metadata_cache_hits: u64,
    pub metadata_cache_misses: u64,
    pub playlist_operations: u64,
    pub memory_usage_bytes: Option<u64>,
    pub peak_memory_bytes: Option<u64>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            startup_time_ms: None,
            metadata_cache_hits: 0,
            metadata_cache_misses: 0,
            playlist_operations: 0,
            memory_usage_bytes: None,
            peak_memory_bytes: None,
        }
    }

    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.metadata_cache_hits + self.metadata_cache_misses;
        if total == 0 {
            0.0
        } else {
            (self.metadata_cache_hits as f64) / (total as f64)
        }
    }

    pub fn memory_usage_mb(&self) -> Option<f64> {
        self.memory_usage_bytes.map(|bytes| bytes as f64 / 1_048_576.0)
    }

    pub fn peak_memory_mb(&self) -> Option<f64> {
        self.peak_memory_bytes.map(|bytes| bytes as f64 / 1_048_576.0)
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Global performance metrics
static METRICS: Mutex<Option<PerformanceMetrics>> = Mutex::new(None);

/// Initialize performance tracking
pub fn init_performance_tracking() {
    let mut metrics = METRICS.lock().unwrap();
    *metrics = Some(PerformanceMetrics::new());
}

/// Record startup time
pub fn record_startup_time(duration: Duration) {
    let mut metrics = METRICS.lock().unwrap();
    if let Some(ref mut m) = *metrics {
        m.startup_time_ms = Some(duration.as_millis() as u64);
        eprintln!("Startup time: {:?}", duration);
    }
}

/// Record metadata cache hit
pub fn record_cache_hit() {
    let mut metrics = METRICS.lock().unwrap();
    if let Some(ref mut m) = *metrics {
        m.metadata_cache_hits += 1;
    }
}

/// Record metadata cache miss
pub fn record_cache_miss() {
    let mut metrics = METRICS.lock().unwrap();
    if let Some(ref mut m) = *metrics {
        m.metadata_cache_misses += 1;
    }
}

/// Record playlist operation
pub fn record_playlist_operation() {
    let mut metrics = METRICS.lock().unwrap();
    if let Some(ref mut m) = *metrics {
        m.playlist_operations += 1;
    }
}

/// Update memory usage metrics
pub fn update_memory_usage() {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        // Get current process ID
        let pid = std::process::id();
        
        // Use ps command to get memory usage on macOS
        if let Ok(output) = Command::new("ps")
            .args(&["-o", "rss=", "-p", &pid.to_string()])
            .output()
        {
            if let Ok(rss_str) = String::from_utf8(output.stdout) {
                if let Ok(rss_kb) = rss_str.trim().parse::<u64>() {
                    let bytes = rss_kb * 1024; // Convert KB to bytes
                    
                    let mut metrics = METRICS.lock().unwrap();
                    if let Some(ref mut m) = *metrics {
                        m.memory_usage_bytes = Some(bytes);
                        
                        // Update peak if current is higher
                        if let Some(peak) = m.peak_memory_bytes {
                            if bytes > peak {
                                m.peak_memory_bytes = Some(bytes);
                            }
                        } else {
                            m.peak_memory_bytes = Some(bytes);
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // Windows memory tracking would go here
        // For now, we'll skip it as the task focuses on the implementation
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux memory tracking would go here
        // For now, we'll skip it as the task focuses on the implementation
    }
}

/// Get current metrics with updated memory usage
pub fn get_metrics() -> Option<PerformanceMetrics> {
    update_memory_usage();
    let metrics = METRICS.lock().unwrap();
    metrics.clone()
}

/// Timer for measuring operation duration
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            name: name.into(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        eprintln!("[PERF] {} took {:?}", self.name, elapsed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_initialization() {
        init_performance_tracking();
        let metrics = get_metrics();
        assert!(metrics.is_some());
    }

    #[test]
    fn test_cache_hit_rate() {
        let mut metrics = PerformanceMetrics::new();
        assert_eq!(metrics.cache_hit_rate(), 0.0);

        metrics.metadata_cache_hits = 8;
        metrics.metadata_cache_misses = 2;
        assert_eq!(metrics.cache_hit_rate(), 0.8);
    }

    #[test]
    fn test_timer() {
        let timer = Timer::new("test_operation");
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }
}
