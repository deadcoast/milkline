<script lang="ts">
  // Props
  let { 
    duration, 
    startTime, 
    endTime,
    ontrimchange
  }: {
    duration: number;
    startTime: number;
    endTime: number;
    ontrimchange?: (trim: { startSec: number; endSec: number }) => void;
  } = $props();

  // State
  let isDraggingStart = $state(false);
  let isDraggingEnd = $state(false);
  let timelineWidth = $state(400); // Default width, will be updated by container
  let timelineElement: HTMLDivElement | null = $state(null);

  // Derived values
  const startPosition = $derived((startTime / duration) * timelineWidth);
  const endPosition = $derived((endTime / duration) * timelineWidth);
  const selectionWidth = $derived(endPosition - startPosition);

  /**
   * Convert pixel position to time in seconds
   */
  function positionToTime(position: number): number {
    if (timelineWidth === 0) return 0;
    const clampedPosition = Math.max(0, Math.min(position, timelineWidth));
    return (clampedPosition / timelineWidth) * duration;
  }

  /**
   * Handle mouse down on start handle
   */
  function handleStartHandleMouseDown(event: MouseEvent) {
    event.stopPropagation();
    isDraggingStart = true;
  }

  /**
   * Handle mouse down on end handle
   */
  function handleEndHandleMouseDown(event: MouseEvent) {
    event.stopPropagation();
    isDraggingEnd = true;
  }

  /**
   * Handle mouse move - update handle positions
   */
  function handleMouseMove(event: MouseEvent) {
    if (!isDraggingStart && !isDraggingEnd) return;
    if (!timelineElement) return;

    const rect = timelineElement.getBoundingClientRect();
    const mouseX = event.clientX - rect.left;
    const newTime = positionToTime(mouseX);

    if (isDraggingStart) {
      // Constrain start handle to not exceed end handle
      const constrainedStartTime = Math.min(newTime, endTime);
      ontrimchange?.({ startSec: constrainedStartTime, endSec: endTime });
    } else if (isDraggingEnd) {
      // Constrain end handle to not go before start handle
      const constrainedEndTime = Math.max(newTime, startTime);
      ontrimchange?.({ startSec: startTime, endSec: constrainedEndTime });
    }
  }

  /**
   * Handle mouse up - stop dragging
   */
  function handleMouseUp() {
    isDraggingStart = false;
    isDraggingEnd = false;
  }

  /**
   * Format time in seconds to MM:SS format
   */
  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }

  /**
   * Update timeline width when element is mounted or resized
   */
  $effect(() => {
    if (timelineElement) {
      const updateWidth = () => {
        timelineWidth = timelineElement!.clientWidth;
      };
      updateWidth();

      // Update on window resize
      const resizeObserver = new ResizeObserver(updateWidth);
      resizeObserver.observe(timelineElement);

      return () => {
        resizeObserver.disconnect();
      };
    }
  });
</script>

<svelte:window onmousemove={handleMouseMove} onmouseup={handleMouseUp} />

<div class="timeline-container">
  <div class="timeline-labels">
    <span class="time-label">Start: {formatTime(startTime)}</span>
    <span class="time-label">End: {formatTime(endTime)}</span>
    <span class="time-label">Duration: {formatTime(duration)}</span>
  </div>

  <div
    bind:this={timelineElement}
    class="timeline"
    role="slider"
    aria-label="Video timeline"
    aria-valuemin={0}
    aria-valuemax={duration}
    aria-valuenow={startTime}
    tabindex="0"
  >
    <!-- Full timeline bar -->
    <div class="timeline-bar">
      <!-- Selected region -->
      <div
        class="timeline-selection"
        style="left: {startPosition}px; width: {selectionWidth}px;"
      >
        <!-- Start handle -->
        <div
          class="timeline-handle timeline-handle-start"
          onmousedown={handleStartHandleMouseDown}
          role="button"
          tabindex="0"
          aria-label="Start time handle"
        >
          <div class="handle-line"></div>
        </div>

        <!-- End handle -->
        <div
          class="timeline-handle timeline-handle-end"
          onmousedown={handleEndHandleMouseDown}
          role="button"
          tabindex="0"
          aria-label="End time handle"
        >
          <div class="handle-line"></div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .timeline-container {
    width: 100%;
    padding: 16px;
    user-select: none;
  }

  .timeline-labels {
    display: flex;
    justify-content: space-between;
    margin-bottom: 8px;
    font-size: 12px;
    color: #666;
  }

  .time-label {
    font-family: monospace;
  }

  .timeline {
    position: relative;
    width: 100%;
    height: 40px;
    cursor: pointer;
  }

  .timeline-bar {
    position: absolute;
    top: 50%;
    left: 0;
    right: 0;
    height: 8px;
    background-color: #ddd;
    border-radius: 4px;
    transform: translateY(-50%);
  }

  .timeline-selection {
    position: absolute;
    top: 0;
    height: 100%;
    background-color: #00aaff;
    border-radius: 4px;
  }

  .timeline-handle {
    position: absolute;
    top: 50%;
    width: 16px;
    height: 32px;
    transform: translateY(-50%);
    cursor: ew-resize;
    z-index: 10;
  }

  .timeline-handle-start {
    left: -8px;
  }

  .timeline-handle-end {
    right: -8px;
  }

  .handle-line {
    width: 3px;
    height: 100%;
    background-color: #fff;
    border: 1px solid #00aaff;
    border-radius: 2px;
    margin: 0 auto;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  .timeline-handle:hover .handle-line {
    background-color: #00aaff;
    border-color: #0088cc;
  }

  .timeline-handle:active .handle-line {
    background-color: #0088cc;
  }
</style>
