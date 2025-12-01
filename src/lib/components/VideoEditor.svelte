<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import CropOverlay from './CropOverlay.svelte';
  import Timeline from './Timeline.svelte';
  import { mediaEditorStore } from '$lib/stores/mediaEditorStore';
  import type { CropRect, TrimState } from '$lib/types';

  // Props
  let { filePath }: { filePath: string } = $props();

  // State
  let containerElement: HTMLDivElement | null = $state(null);
  let cropOverlayRef: any = $state(null);
  let sourceWidth = $state(0);
  let sourceHeight = $state(0);
  let previewWidth = $state(0);
  let previewHeight = $state(0);
  let videoLoaded = $state(false);
  let videoSrc = $state('');
  let duration = $state(0);
  let startTime = $state(0);
  let endTime = $state(0);
  let error = $state<string | null>(null);

  /**
   * Load video metadata and display preview
   */
  onMount(async () => {
    if (filePath) {
      try {
        // Convert file path to a URL that can be loaded in the browser
        videoSrc = convertFileSrc(filePath);

        // Probe video metadata using Tauri command
        const metadata = await invoke<{ duration_sec: number; width: number; height: number }>(
          'probe_video_metadata_command',
          { path: filePath }
        );

        sourceWidth = metadata.width;
        sourceHeight = metadata.height;
        duration = metadata.duration_sec;
        startTime = 0;
        endTime = metadata.duration_sec;

        // Update store with initial trim state
        mediaEditorStore.setTrim({
          startSec: startTime,
          endSec: endTime,
          durationSec: duration
        });

        // Calculate preview dimensions
        calculatePreviewDimensions();

        videoLoaded = true;
      } catch (err) {
        error = `Failed to load video: ${err}`;
        console.error(error);
      }
    }
  });

  /**
   * Calculate preview dimensions to fit container while maintaining aspect ratio
   */
  function calculatePreviewDimensions() {
    if (!containerElement || sourceWidth === 0 || sourceHeight === 0) return;

    const containerWidth = containerElement.clientWidth;
    const containerHeight = containerElement.clientHeight - 100; // Reserve space for timeline

    const sourceAspect = sourceWidth / sourceHeight;
    const containerAspect = containerWidth / containerHeight;

    if (sourceAspect > containerAspect) {
      // Video is wider than container
      previewWidth = containerWidth;
      previewHeight = containerWidth / sourceAspect;
    } else {
      // Video is taller than container
      previewHeight = containerHeight;
      previewWidth = containerHeight * sourceAspect;
    }
  }

  /**
   * Handle crop change from overlay
   */
  function handleCropChange(crop: CropRect) {
    mediaEditorStore.setCrop(crop);
  }

  /**
   * Handle trim change from timeline
   */
  function handleTrimChange(trim: { startSec: number; endSec: number }) {
    startTime = trim.startSec;
    endTime = trim.endSec;
    
    mediaEditorStore.setTrim({
      startSec: trim.startSec,
      endSec: trim.endSec,
      durationSec: duration
    });
  }

  /**
   * Export the video with trim and crop applied
   */
  export async function exportVideo(outputPath: string): Promise<void> {
    // Get current store state
    let currentCrop: CropRect | null = null;
    let currentTrim: TrimState | null = null;
    const unsubscribe = mediaEditorStore.subscribe(state => {
      currentCrop = state.crop;
      currentTrim = state.trim;
    });
    unsubscribe();

    // Use default trim if not set
    if (!currentTrim) {
      currentTrim = {
        startSec: 0,
        endSec: duration,
        durationSec: duration
      };
    }

    // Create export config with defaults
    const config = {
      video_codec: 'libx264',
      audio_codec: 'aac',
      quality: '23'
    };

    // Call the trim_and_crop_video Tauri command
    try {
      await invoke('trim_and_crop_video_command', {
        inputPath: filePath,
        outputPath: outputPath,
        startSec: currentTrim.startSec,
        endSec: currentTrim.endSec,
        cropRect: currentCrop,
        config: config
      });
    } catch (error) {
      // Re-throw with more context
      throw new Error(`Failed to export video: ${error}`);
    }
  }

  /**
   * Clear the crop selection
   */
  export function clearCrop() {
    cropOverlayRef?.clear();
    mediaEditorStore.clearCrop();
  }
</script>

<div class="video-editor" bind:this={containerElement}>
  {#if error}
    <div class="error-message">
      <p>{error}</p>
    </div>
  {:else if videoLoaded}
    <div class="video-container">
      <!-- Video preview placeholder -->
      <div class="video-preview" style="width: {previewWidth}px; height: {previewHeight}px;">
        <div class="preview-placeholder">
          <p>Video Preview</p>
          <p class="video-info">{sourceWidth}x{sourceHeight}</p>
          <p class="video-info">{duration.toFixed(2)}s</p>
        </div>
      </div>

      {#if videoLoaded}
        <CropOverlay
          bind:this={cropOverlayRef}
          {sourceWidth}
          {sourceHeight}
          {previewWidth}
          {previewHeight}
          oncropchange={handleCropChange}
        />
      {/if}
    </div>

    <div class="timeline-container">
      <Timeline
        {duration}
        startTime={startTime}
        endTime={endTime}
        ontrimchange={handleTrimChange}
      />
    </div>
  {:else}
    <div class="loading">
      <p>Loading video...</p>
    </div>
  {/if}
</div>

<style>
  .video-editor {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: #1a1a1a;
    overflow: hidden;
  }

  .video-container {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    min-height: 0;
  }

  .video-preview {
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #2a2a2a;
    border: 2px solid #444;
    border-radius: 4px;
  }

  .preview-placeholder {
    text-align: center;
    color: #888;
  }

  .preview-placeholder p {
    margin: 8px 0;
  }

  .video-info {
    font-size: 0.9rem;
    color: #666;
  }

  .timeline-container {
    background-color: #252525;
    border-top: 1px solid #444;
  }

  .loading,
  .error-message {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #888;
    font-size: 1.2rem;
  }

  .error-message {
    color: #ff6b6b;
  }
</style>
