<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getPerformanceMetrics, getCacheHitRate, getMemoryUsage, getPeakMemory } from '../tauri/ipc';
  import type { PerformanceMetrics } from '../tauri/ipc';

  let metrics: PerformanceMetrics | null = null;
  let memoryUsageMB: number | null = null;
  let peakMemoryMB: number | null = null;
  let cacheHitRate: number = 0;
  let updateInterval: number | null = null;
  let isVisible = false;

  async function updateMetrics() {
    try {
      metrics = await getPerformanceMetrics();
      memoryUsageMB = await getMemoryUsage();
      peakMemoryMB = await getPeakMemory();
      cacheHitRate = await getCacheHitRate();
    } catch (error) {
      console.error('Failed to fetch performance metrics:', error);
    }
  }

  function toggleVisibility() {
    isVisible = !isVisible;
  }

  onMount(() => {
    // Update metrics every 2 seconds
    updateMetrics();
    updateInterval = window.setInterval(updateMetrics, 2000);
  });

  onDestroy(() => {
    if (updateInterval !== null) {
      clearInterval(updateInterval);
    }
  });

  function formatMemory(mb: number | null): string {
    if (mb === null) return 'N/A';
    return `${mb.toFixed(2)} MB`;
  }

  function formatPercentage(rate: number): string {
    return `${(rate * 100).toFixed(1)}%`;
  }

  function formatTime(ms: number | null): string {
    if (ms === null) return 'N/A';
    return `${(ms / 1000).toFixed(2)}s`;
  }
</script>

<div class="performance-monitor">
  <button class="toggle-btn" on:click={toggleVisibility}>
    {isVisible ? '📊 Hide Stats' : '📊 Show Stats'}
  </button>

  {#if isVisible && metrics}
    <div class="metrics-panel">
      <h3>Performance Metrics</h3>
      
      <div class="metric-group">
        <h4>Startup</h4>
        <div class="metric">
          <span class="label">Startup Time:</span>
          <span class="value">{formatTime(metrics.startup_time_ms)}</span>
        </div>
      </div>

      <div class="metric-group">
        <h4>Memory Usage</h4>
        <div class="metric">
          <span class="label">Current:</span>
          <span class="value" class:warning={memoryUsageMB && memoryUsageMB > 100}>
            {formatMemory(memoryUsageMB)}
          </span>
        </div>
        <div class="metric">
          <span class="label">Peak:</span>
          <span class="value">{formatMemory(peakMemoryMB)}</span>
        </div>
        <div class="metric">
          <span class="label">Target:</span>
          <span class="value">{'< 100 MB'}</span>
        </div>
      </div>

      <div class="metric-group">
        <h4>Metadata Cache</h4>
        <div class="metric">
          <span class="label">Hit Rate:</span>
          <span class="value" class:good={cacheHitRate > 0.7}>
            {formatPercentage(cacheHitRate)}
          </span>
        </div>
        <div class="metric">
          <span class="label">Hits:</span>
          <span class="value">{metrics.metadata_cache_hits}</span>
        </div>
        <div class="metric">
          <span class="label">Misses:</span>
          <span class="value">{metrics.metadata_cache_misses}</span>
        </div>
      </div>

      <div class="metric-group">
        <h4>Operations</h4>
        <div class="metric">
          <span class="label">Playlist Ops:</span>
          <span class="value">{metrics.playlist_operations}</span>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .performance-monitor {
    position: fixed;
    top: 10px;
    right: 10px;
    z-index: 1000;
  }

  .toggle-btn {
    padding: 8px 16px;
    background: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .toggle-btn:hover {
    background: #0056b3;
  }

  .metrics-panel {
    margin-top: 10px;
    padding: 16px;
    background: rgba(255, 255, 255, 0.95);
    border-radius: 8px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
    min-width: 250px;
    max-width: 350px;
  }

  .metrics-panel h3 {
    margin: 0 0 16px 0;
    font-size: 16px;
    font-weight: 600;
    color: #333;
    border-bottom: 2px solid #007bff;
    padding-bottom: 8px;
  }

  .metric-group {
    margin-bottom: 16px;
  }

  .metric-group:last-child {
    margin-bottom: 0;
  }

  .metric-group h4 {
    margin: 0 0 8px 0;
    font-size: 13px;
    font-weight: 600;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .metric {
    display: flex;
    justify-content: space-between;
    padding: 4px 0;
    font-size: 13px;
  }

  .label {
    color: #666;
    font-weight: 500;
  }

  .value {
    color: #333;
    font-weight: 600;
    font-family: 'Courier New', monospace;
  }

  .value.warning {
    color: #ff6b6b;
  }

  .value.good {
    color: #51cf66;
  }
</style>
