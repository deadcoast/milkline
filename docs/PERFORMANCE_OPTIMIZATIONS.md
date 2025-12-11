# Performance Optimizations

This document describes the performance optimizations implemented for the milk player application to meet the requirements of task 27.

## Overview

The following optimizations have been implemented to ensure the application meets the performance targets:

- **Target**: <100MB RAM usage during idle playback
- **Target**: <2 second startup time
- **Target**: 30+ FPS visualization

## Implemented Optimizations

### 1. Lazy Loading for Playlist Tracks

**Location**: `src/lib/components/Playlist.svelte`

**Implementation**:

- Metadata is now loaded on-demand when the user hovers over a track
- Tracks are displayed immediately with basic information
- Full metadata (including album art) is fetched only when needed
- Reduces initial memory footprint and improves playlist loading speed

**Benefits**:

- Faster playlist loading (no upfront metadata extraction)
- Reduced memory usage (metadata loaded only for visible/interacted tracks)
- Better user experience (immediate UI response)

### 2. Visualizer Throttling

**Location**: `src/lib/components/Visualizer.svelte`

**Implementation**:

- Adaptive frame rate based on window state:
  - **Focused**: 30 FPS (full quality)
  - **Unfocused**: 10 FPS (reduced CPU usage)
  - **Hidden**: 5 FPS (minimal CPU usage)
- Uses `visibilitychange` API to detect when window is hidden
- Uses `focus`/`blur` events to detect window focus state

**Benefits**:

- Reduced CPU usage when application is not in focus
- Minimal resource consumption when window is hidden
- Maintains smooth visualization when user is actively watching

### 3. Optimized Cache Sizes

**Backend Location**: `src-tauri/src/metadata.rs`
**Frontend Location**: `src/lib/stores/metadataCache.ts`

**Implementation**:

- Reduced backend metadata cache from 1000 to 500 entries
- Reduced frontend metadata cache from 1000 to 500 entries
- Implemented LRU (Least Recently Used) eviction with access frequency tracking
- Each cache entry is approximately 200 bytes
- Total cache memory: ~100KB (500 entries × 200 bytes)

**Benefits**:

- Reduced memory footprint
- Better cache hit rates due to frequency-aware eviction
- Maintains performance for frequently accessed tracks

### 4. Memory Profiling

**Location**: `src-tauri/src/performance.rs`

**Implementation**:

- Added memory usage tracking (current and peak)
- Platform-specific implementation (macOS using `ps` command)
- Memory metrics exposed via IPC commands:
  - `get_memory_usage()` - Current memory usage in MB
  - `get_peak_memory()` - Peak memory usage in MB
  - `get_performance_metrics()` - Complete metrics including memory

**Benefits**:

- Real-time monitoring of memory usage
- Ability to detect memory leaks
- Performance regression detection

### 5. Performance Monitoring Component

**Location**: `src/lib/components/PerformanceMonitor.svelte`

**Implementation**:

- Real-time performance dashboard
- Displays:
  - Startup time
  - Current and peak memory usage
  - Metadata cache hit rate
  - Playlist operations count
- Updates every 2 seconds
- Toggleable visibility

**Benefits**:

- Easy performance monitoring during development
- Visual feedback on optimization effectiveness
- Helps identify performance bottlenecks

## Performance Metrics

### Startup Time

- **Target**: <2 seconds
- **Tracking**: Measured from application start to window display
- **Location**: `src-tauri/src/lib.rs` (run function)

### Memory Usage

- **Target**: <100MB during idle playback
- **Tracking**: Real-time via `ps` command on macOS
- **Optimizations**:
  - Reduced cache sizes (500 entries each)
  - Lazy loading of metadata
  - Efficient LRU eviction

### Cache Performance

- **Metric**: Cache hit rate
- **Target**: >70% hit rate for optimal performance
- **Tracking**: Metadata cache hits vs misses
- **Display**: Performance monitor shows hit rate percentage

### Visualizer Performance

- **Target**: 30+ FPS when focused
- **Adaptive**: 10 FPS when unfocused, 5 FPS when hidden
- **Tracking**: Frame timing in render loop
- **Optimization**: Throttling based on window state

## Testing

### Manual Testing

1. **Memory Usage Test**:
   - Open Performance Monitor
   - Load a large playlist (100+ tracks)
   - Verify memory stays under 100MB
   - Check peak memory after extended use

2. **Lazy Loading Test**:
   - Open a playlist with many tracks
   - Observe that tracks load instantly
   - Hover over tracks to trigger metadata loading
   - Verify "Loading metadata..." indicator appears

3. **Visualizer Throttling Test**:
   - Start audio playback with visualizer active
   - Check console for "30 FPS" message
   - Blur window (click elsewhere)
   - Check console for "10 FPS" message
   - Minimize window
   - Check console for "5 FPS" message

4. **Startup Time Test**:
   - Close application completely
   - Start application
   - Check Performance Monitor for startup time
   - Verify it's under 2 seconds

### Automated Testing

- Performance metrics tests: `src-tauri/tests/performance_test.rs`
- Tests cover:
  - Metrics initialization
  - Cache hit rate calculation
  - Memory tracking
  - Startup time recording
  - Playlist operations tracking

## Future Optimizations

### Potential Improvements

1. **Virtual Scrolling**: Implement virtual scrolling for very large playlists
2. **Web Workers**: Move metadata processing to web workers
3. **IndexedDB**: Use IndexedDB for persistent frontend caching
4. **Streaming Metadata**: Stream metadata in chunks for large libraries
5. **Image Optimization**: Compress album art before caching

### Monitoring

- Continue tracking performance metrics in production
- Set up alerts for memory usage exceeding 100MB
- Monitor cache hit rates to optimize cache size
- Track startup time across different hardware

## Configuration

### Cache Size Configuration

To adjust cache sizes, modify:

- Backend: `src-tauri/src/metadata.rs` - `MetadataExtractor::new()`
- Frontend: `src/lib/stores/metadataCache.ts` - `MAX_CACHE_SIZE`

### Visualizer Frame Rates

To adjust frame rates, modify:

- `src/lib/components/Visualizer.svelte`:
  - `TARGET_FPS` - Focused frame rate (default: 30)
  - `TARGET_FPS_UNFOCUSED` - Unfocused frame rate (default: 10)
  - `TARGET_FPS_HIDDEN` - Hidden frame rate (default: 5)

## Conclusion

These optimizations ensure the milk player meets its performance targets while maintaining a smooth user experience. The combination of lazy loading, adaptive throttling, optimized caching, and real-time monitoring provides a solid foundation for a lightweight, responsive application.
