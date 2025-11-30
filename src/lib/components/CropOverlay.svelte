<script lang="ts">
  import { previewRectToSourceRect } from '$lib/utils/coordinates';
  import type { CropRect } from '$lib/types';

  // Props
  let { 
    sourceWidth, 
    sourceHeight, 
    previewWidth, 
    previewHeight,
    oncropchange
  }: {
    sourceWidth: number;
    sourceHeight: number;
    previewWidth: number;
    previewHeight: number;
    oncropchange?: (crop: CropRect) => void;
  } = $props();

  // State
  let isDrawing = $state(false);
  let startX = $state(0);
  let startY = $state(0);
  let currentX = $state(0);
  let currentY = $state(0);
  let cropRect = $state<{ x: number; y: number; width: number; height: number } | null>(null);

  /**
   * Normalize rectangle to handle drawing in any direction
   */
  function normalizeRect(x1: number, y1: number, x2: number, y2: number) {
    return {
      x: Math.min(x1, x2),
      y: Math.min(y1, y2),
      width: Math.abs(x2 - x1),
      height: Math.abs(y2 - y1),
    };
  }

  /**
   * Handle mouse down - start drawing
   */
  function handleMouseDown(event: MouseEvent) {
    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    startX = event.clientX - rect.left;
    startY = event.clientY - rect.top;
    currentX = startX;
    currentY = startY;
    isDrawing = true;
    cropRect = null; // Clear existing crop when starting new one
  }

  /**
   * Handle mouse move - update rectangle
   */
  function handleMouseMove(event: MouseEvent) {
    if (!isDrawing) return;

    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    currentX = event.clientX - rect.left;
    currentY = event.clientY - rect.top;

    // Clamp to preview bounds
    currentX = Math.max(0, Math.min(currentX, previewWidth));
    currentY = Math.max(0, Math.min(currentY, previewHeight));

    // Update crop rectangle
    cropRect = normalizeRect(startX, startY, currentX, currentY);
  }

  /**
   * Handle mouse up - finish drawing and emit event
   */
  function handleMouseUp(event: MouseEvent) {
    if (!isDrawing) return;

    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    currentX = event.clientX - rect.left;
    currentY = event.clientY - rect.top;

    // Clamp to preview bounds
    currentX = Math.max(0, Math.min(currentX, previewWidth));
    currentY = Math.max(0, Math.min(currentY, previewHeight));

    isDrawing = false;

    // Finalize crop rectangle
    const previewRect = normalizeRect(startX, startY, currentX, currentY);
    
    // Only emit if rectangle has meaningful size (at least 1x1 pixel)
    if (previewRect.width > 0 && previewRect.height > 0) {
      cropRect = previewRect;

      // Convert to source coordinates
      const sourceRect = previewRectToSourceRect(
        previewRect,
        previewWidth,
        previewHeight,
        sourceWidth,
        sourceHeight
      );

      // Call event callback with source coordinates
      oncropchange?.(sourceRect);
    }
  }

  /**
   * Clear the crop rectangle
   */
  export function clear() {
    cropRect = null;
    isDrawing = false;
  }
</script>

<div
  class="crop-overlay"
  style="width: {previewWidth}px; height: {previewHeight}px;"
  onmousedown={handleMouseDown}
  onmousemove={handleMouseMove}
  onmouseup={handleMouseUp}
  onmouseleave={handleMouseUp}
  role="button"
  tabindex="0"
  aria-label="Crop selection area"
>
  {#if cropRect}
    <svg
      class="crop-svg"
      width={previewWidth}
      height={previewHeight}
      xmlns="http://www.w3.org/2000/svg"
    >
      <!-- Darken everything outside the crop rectangle -->
      <defs>
        <mask id="crop-mask">
          <rect x="0" y="0" width={previewWidth} height={previewHeight} fill="white" />
          <rect
            x={cropRect.x}
            y={cropRect.y}
            width={cropRect.width}
            height={cropRect.height}
            fill="black"
          />
        </mask>
      </defs>
      <rect
        x="0"
        y="0"
        width={previewWidth}
        height={previewHeight}
        fill="rgba(0, 0, 0, 0.5)"
        mask="url(#crop-mask)"
      />

      <!-- Draw crop rectangle border -->
      <rect
        x={cropRect.x}
        y={cropRect.y}
        width={cropRect.width}
        height={cropRect.height}
        fill="none"
        stroke="#00aaff"
        stroke-width="2"
        stroke-dasharray="5,5"
      />

      <!-- Corner handles -->
      <circle cx={cropRect.x} cy={cropRect.y} r="4" fill="#00aaff" />
      <circle cx={cropRect.x + cropRect.width} cy={cropRect.y} r="4" fill="#00aaff" />
      <circle cx={cropRect.x} cy={cropRect.y + cropRect.height} r="4" fill="#00aaff" />
      <circle
        cx={cropRect.x + cropRect.width}
        cy={cropRect.y + cropRect.height}
        r="4"
        fill="#00aaff"
      />
    </svg>
  {/if}
</div>

<style>
  .crop-overlay {
    position: absolute;
    top: 0;
    left: 0;
    cursor: crosshair;
    user-select: none;
  }

  .crop-svg {
    position: absolute;
    top: 0;
    left: 0;
    pointer-events: none;
  }
</style>
