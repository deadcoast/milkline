// Integration test for performance optimizations
use milk_lib::performance;

#[test]
fn test_performance_metrics_initialization() {
    performance::init_performance_tracking();
    let metrics = performance::get_metrics();
    assert!(metrics.is_some());
}

#[test]
fn test_cache_hit_rate_calculation() {
    performance::init_performance_tracking();
    
    // Record some cache operations
    performance::record_cache_hit();
    performance::record_cache_hit();
    performance::record_cache_miss();
    
    let metrics = performance::get_metrics().unwrap();
    assert_eq!(metrics.metadata_cache_hits, 2);
    assert_eq!(metrics.metadata_cache_misses, 1);
    
    // Hit rate should be 2/3 = 0.666...
    let hit_rate = metrics.cache_hit_rate();
    assert!((hit_rate - 0.666).abs() < 0.01);
}

#[test]
fn test_memory_tracking() {
    performance::init_performance_tracking();
    performance::update_memory_usage();
    
    let metrics = performance::get_metrics().unwrap();
    
    // On macOS, memory usage should be tracked
    #[cfg(target_os = "macos")]
    {
        assert!(metrics.memory_usage_bytes.is_some());
        assert!(metrics.peak_memory_bytes.is_some());
        
        if let Some(usage_mb) = metrics.memory_usage_mb() {
            println!("Current memory usage: {:.2} MB", usage_mb);
            // Memory should be reasonable (less than 1GB for tests)
            assert!(usage_mb < 1024.0);
        }
    }
}

#[test]
fn test_startup_time_recording() {
    use std::time::Duration;
    
    performance::init_performance_tracking();
    
    let duration = Duration::from_millis(1500);
    performance::record_startup_time(duration);
    
    let metrics = performance::get_metrics().unwrap();
    assert_eq!(metrics.startup_time_ms, Some(1500));
}

#[test]
fn test_playlist_operations_tracking() {
    performance::init_performance_tracking();
    
    performance::record_playlist_operation();
    performance::record_playlist_operation();
    performance::record_playlist_operation();
    
    let metrics = performance::get_metrics().unwrap();
    assert_eq!(metrics.playlist_operations, 3);
}
