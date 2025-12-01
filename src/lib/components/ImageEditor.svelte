<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import CropOverlay from './CropOverlay.svelte';
  import { mediaEditorStore } from '$lib/stores/mediaEditorStore';
  import type { CropRect } from '$lib/types';

  // Props
  let { filePath }: { filePath: string } = $props();

  // State
  let imageElement: HTMLImageElement | null = $state(null);
  let containerElement: HTMLDivElement | null = $state(null);
  let cropOverlayRef: any = $state(null);
  let sourceWidth = $state(0);
  let sourceHeight = $state(0);
  let previewWidth = $state(0);
  let previewHeight = $state(0);
  let imageLoaded = $state(false);
  let imageSrc = $state('');

  /**
   * Load and display the image
   */
  onMount(() => {
    if (filePath) {
      // Convert file path to a URL that can be loaded in the browser
      imageSrc = convertFileSrc(filePath);
    }
  });

  /**
   * Handle image load - calculate dimensions
   */
  function handleImageLoad() {
    if (!imageElement || !containerElement) return;

    // Get source dimensions from the loaded image
    sourceWidth = imageElement.naturalWidth;
    sourceHeight = imageElement.naturalHeight;

    // Calculate preview dimensions to fit container while maintaining aspect ratio
    const containerWidth = containerElement.clientWidth;
    const containerHeight = containerElement.clientHeight;
    
    const sourceAspect = sourceWidth / sourceHeight;
    const containerAspect = containerWidth / containerHeight;

    if (sourceAspect > containerAspect) {
      // Image is wider than container
      previewWidth = containerWidth;
      previewHeight = containerWidth / sourceAspect;
    } else {
      // Image is taller than container
      previewHeight = containerHeight;
      previewWidth = containerHeight * sourceAspect;
    }

    imageLoaded = true;
  }

  /**
   * Handle crop change from overlay
   */
  function handleCropChange(crop: CropRect) {
    mediaEditorStore.setCrop(crop);
  }

  /**
   * Export the image with crop applied
   */
  export async function exportImage(outputPath: string): Promise<void> {
    // Get current store state
    let currentCrop: CropRect | null = null;
    const unsubscribe = mediaEditorStore.subscribe(state => {
      currentCrop = state.crop;
    });
    unsubscribe();
    
    if (!currentCrop) {
      // No crop defined - use full image dimensions
      // Create a crop rect that covers the entire image
      currentCrop = {
        x: 0,
        y: 0,
        width: sourceWidth,
        height: sourceHeight
      };
    }

    // Call the crop_image Tauri command
    try {
      await invoke('crop_image_command', {
        inputPath: filePath,
        outputPath: outputPath,
        cropRect: currentCrop
      });
    } catch (error) {
      // Re-throw with more context
      throw new Error(`Failed to export image: ${error}`);
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

<div class="image-editor" bind:this={containerElement}>
  {#if imageSrc}
    <div class="image-container">
      <img
        bind:this={imageElement}
        src={imageSrc}
        alt="Image to edit"
        class="preview-image"
        style="width: {previewWidth}px; height: {previewHeight}px;"
        onload={handleImageLoad}
      />
      
      {#if imageLoaded}
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
  {:else}
    <div class="no-image">
      <p>No image loaded</p>
    </div>
  {/if}
</div>

<style>
  .image-editor {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #1a1a1a;
    overflow: hidden;
  }

  .image-container {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .preview-image {
    display: block;
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
  }

  .no-image {
    color: #888;
    font-size: 1.2rem;
  }
</style>
