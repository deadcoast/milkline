<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { playerStore } from '$lib/stores';
  import type { Track } from '$lib/types';

  let audioElement: HTMLAudioElement | null = null;
  let positionUpdateInterval: number | null = null;

  // Subscribe to player state
  let currentTrack = $derived($playerStore.currentTrack);
  let isPlaying = $derived($playerStore.isPlaying);
  let position = $derived($playerStore.position);
  let duration = $derived($playerStore.duration);
  let volume = $derived($playerStore.volume);
  let queue = $derived($playerStore.queue);

  // Format time in MM:SS
  function formatTime(seconds: number): string {
    if (!isFinite(seconds) || seconds < 0) return '00:00';
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }

  // Play handler
  export function play(track?: Track) {
    if (track) {
      // Load new track
      playerStore.setCurrentTrack(track);
      if (audioElement && track.filePath) {
        audioElement.src = `file://${track.filePath}`;
        audioElement.load();
      }
    }
    
    if (audioElement) {
      audioElement.play().catch(err => {
        console.error('Playback failed:', err);
        playerStore.setPlaying(false);
      });
    }
  }

  // Pause handler
  export function pause() {
    if (audioElement) {
      audioElement.pause();
    }
    playerStore.setPlaying(false);
  }

  // Stop handler
  export function stop() {
    if (audioElement) {
      audioElement.pause();
      audioElement.currentTime = 0;
    }
    playerStore.setPlaying(false);
    playerStore.setPosition(0);
  }

  // Next track handler
  export function next() {
    if (queue.length > 0) {
      const nextTrack = queue[0];
      playerStore.removeFromQueue(nextTrack.id);
      play(nextTrack);
    } else {
      stop();
    }
  }

  // Previous track handler
  export function previous() {
    if (audioElement && audioElement.currentTime > 3) {
      // If more than 3 seconds into track, restart current track
      audioElement.currentTime = 0;
    } else {
      // Otherwise, go to previous track (not implemented yet - would need history)
      stop();
    }
  }

  // Seek handler
  export function seek(newPosition: number) {
    if (audioElement && isFinite(newPosition)) {
      audioElement.currentTime = newPosition;
      playerStore.setPosition(newPosition);
    }
  }

  // Volume handler
  export function setVolume(newVolume: number) {
    const clampedVolume = Math.max(0, Math.min(1, newVolume));
    if (audioElement) {
      audioElement.volume = clampedVolume;
    }
    playerStore.setVolume(clampedVolume);
  }

  // Handle audio element events
  function handleLoadedMetadata() {
    if (audioElement) {
      playerStore.setDuration(audioElement.duration);
    }
  }

  function handlePlay() {
    playerStore.setPlaying(true);
    startPositionTracking();
  }

  function handlePause() {
    playerStore.setPlaying(false);
    stopPositionTracking();
  }

  function handleEnded() {
    playerStore.setPlaying(false);
    stopPositionTracking();
    next(); // Auto-play next track
  }

  function handleTimeUpdate() {
    if (audioElement) {
      playerStore.setPosition(audioElement.currentTime);
    }
  }

  function handleError(event: Event) {
    console.error('Audio playback error:', event);
    playerStore.setPlaying(false);
    stopPositionTracking();
  }

  // Position tracking
  function startPositionTracking() {
    if (positionUpdateInterval === null) {
      positionUpdateInterval = window.setInterval(() => {
        if (audioElement && !audioElement.paused) {
          playerStore.setPosition(audioElement.currentTime);
        }
      }, 100); // Update every 100ms
    }
  }

  function stopPositionTracking() {
    if (positionUpdateInterval !== null) {
      clearInterval(positionUpdateInterval);
      positionUpdateInterval = null;
    }
  }

  // Seek bar interaction
  let seekBarElement: HTMLDivElement | null = null;
  function handleSeekBarClick(event: MouseEvent) {
    if (seekBarElement && duration > 0) {
      const rect = seekBarElement.getBoundingClientRect();
      const clickX = event.clientX - rect.left;
      const percentage = clickX / rect.width;
      const newPosition = percentage * duration;
      seek(newPosition);
    }
  }

  onMount(() => {
    // Initialize audio element
    if (audioElement) {
      audioElement.volume = volume;
    }
  });

  onDestroy(() => {
    stopPositionTracking();
  });
</script>

<div class="player">
  <!-- Hidden audio element -->
  <audio
    bind:this={audioElement}
    on:loadedmetadata={handleLoadedMetadata}
    on:play={handlePlay}
    on:pause={handlePause}
    on:ended={handleEnded}
    on:timeupdate={handleTimeUpdate}
    on:error={handleError}
  />

  <!-- Track info display -->
  <div class="track-info">
    {#if currentTrack}
      <div class="track-title">{currentTrack.title}</div>
      <div class="track-artist">{currentTrack.artist}</div>
      <div class="track-album">{currentTrack.album}</div>
    {:else}
      <div class="no-track">No track loaded</div>
    {/if}
  </div>

  <!-- Playback controls -->
  <div class="controls">
    <button class="control-btn" on:click={previous} title="Previous">
      <span class="icon">⏮</span>
    </button>
    
    {#if isPlaying}
      <button class="control-btn play-pause" on:click={pause} title="Pause">
        <span class="icon">⏸</span>
      </button>
    {:else}
      <button class="control-btn play-pause" on:click={() => play()} title="Play">
        <span class="icon">▶</span>
      </button>
    {/if}
    
    <button class="control-btn" on:click={stop} title="Stop">
      <span class="icon">⏹</span>
    </button>
    
    <button class="control-btn" on:click={next} title="Next">
      <span class="icon">⏭</span>
    </button>
  </div>

  <!-- Position bar -->
  <div class="position-section">
    <span class="time-display">{formatTime(position)}</span>
    <div 
      class="seek-bar" 
      bind:this={seekBarElement}
      on:click={handleSeekBarClick}
      role="slider"
      tabindex="0"
      aria-label="Seek position"
      aria-valuemin="0"
      aria-valuemax={duration}
      aria-valuenow={position}
    >
      <div class="seek-progress" style="width: {duration > 0 ? (position / duration) * 100 : 0}%"></div>
    </div>
    <span class="time-display">{formatTime(duration)}</span>
  </div>

  <!-- Volume control -->
  <div class="volume-section">
    <span class="volume-icon">🔊</span>
    <input
      type="range"
      min="0"
      max="1"
      step="0.01"
      value={volume}
      on:input={(e) => setVolume(parseFloat(e.currentTarget.value))}
      class="volume-slider"
      aria-label="Volume"
    />
    <span class="volume-value">{Math.round(volume * 100)}%</span>
  </div>

  <!-- Queue info -->
  <div class="queue-info">
    <span>Queue: {queue.length} track{queue.length !== 1 ? 's' : ''}</span>
  </div>
</div>

<style>
  .player {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 16px;
    background-color: var(--color-player-bg, #1a1a1a);
    color: var(--color-text, #ffffff);
    min-width: 300px;
  }

  .track-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-height: 60px;
  }

  .track-title {
    font-size: 16px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .track-artist,
  .track-album {
    font-size: 14px;
    color: var(--color-text-secondary, #aaaaaa);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .no-track {
    font-size: 14px;
    color: var(--color-text-secondary, #aaaaaa);
    font-style: italic;
  }

  .controls {
    display: flex;
    gap: 8px;
    justify-content: center;
    align-items: center;
  }

  .control-btn {
    background-color: var(--color-surface, #2a2a2a);
    border: 1px solid var(--color-border, #444444);
    border-radius: 4px;
    padding: 8px 12px;
    cursor: pointer;
    transition: background-color 0.2s;
    color: var(--color-text, #ffffff);
  }

  .control-btn:hover {
    background-color: var(--color-accent, #3a3a3a);
  }

  .control-btn:active {
    background-color: var(--color-accent-dark, #4a4a4a);
  }

  .control-btn.play-pause {
    padding: 8px 16px;
  }

  .icon {
    font-size: 18px;
    display: inline-block;
  }

  .position-section {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .time-display {
    font-size: 12px;
    font-family: monospace;
    min-width: 40px;
    text-align: center;
  }

  .seek-bar {
    flex: 1;
    height: 8px;
    background-color: var(--color-surface, #2a2a2a);
    border-radius: 4px;
    cursor: pointer;
    position: relative;
    overflow: hidden;
  }

  .seek-progress {
    height: 100%;
    background-color: var(--color-accent, #00aaff);
    transition: width 0.1s linear;
  }

  .volume-section {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .volume-icon {
    font-size: 16px;
  }

  .volume-slider {
    flex: 1;
    height: 6px;
    -webkit-appearance: none;
    appearance: none;
    background: var(--color-surface, #2a2a2a);
    border-radius: 3px;
    outline: none;
  }

  .volume-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 14px;
    height: 14px;
    background: var(--color-accent, #00aaff);
    border-radius: 50%;
    cursor: pointer;
  }

  .volume-slider::-moz-range-thumb {
    width: 14px;
    height: 14px;
    background: var(--color-accent, #00aaff);
    border-radius: 50%;
    cursor: pointer;
    border: none;
  }

  .volume-value {
    font-size: 12px;
    min-width: 40px;
    text-align: right;
  }

  .queue-info {
    font-size: 12px;
    color: var(--color-text-secondary, #aaaaaa);
    text-align: center;
  }
</style>
