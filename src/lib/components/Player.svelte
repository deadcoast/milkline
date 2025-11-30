<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { playerStore } from '$lib/stores';
  import type { Track } from '$lib/types';
  import { spotifyGetNowPlaying, youtubeGetNowPlaying } from '$lib/tauri/ipc';
  import Visualizer from './Visualizer.svelte';

  let audioElement: HTMLAudioElement | null = null;
  let positionUpdateInterval: number | null = null;
  let visualizerComponent: any = null;
  let streamingMetadataInterval: number | null = null;

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
    // Start visualizer when playback starts
    if (visualizerComponent) {
      visualizerComponent.start();
    }
  }

  function handlePause() {
    playerStore.setPlaying(false);
    stopPositionTracking();
    // Stop visualizer when playback pauses
    if (visualizerComponent) {
      visualizerComponent.stop();
    }
  }

  function handleEnded() {
    playerStore.setPlaying(false);
    stopPositionTracking();
    // Stop visualizer when track ends
    if (visualizerComponent) {
      visualizerComponent.stop();
    }
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

  // Streaming metadata polling
  async function pollStreamingMetadata() {
    if (!currentTrack) return;

    try {
      if (currentTrack.source === 'spotify') {
        const metadata = await spotifyGetNowPlaying();
        if (metadata) {
          updateTrackFromStreamingMetadata(metadata, 'spotify');
        }
      } else if (currentTrack.source === 'youtube') {
        const metadata = await youtubeGetNowPlaying();
        if (metadata) {
          updateTrackFromStreamingMetadata(metadata, 'youtube');
        }
      }
    } catch (error) {
      console.error('Failed to fetch streaming metadata:', error);
    }
  }

  function updateTrackFromStreamingMetadata(
    metadata: any,
    source: 'spotify' | 'youtube'
  ) {
    // Update current track with streaming metadata
    const updatedTrack: Track = {
      id: currentTrack?.id || `${source}-${Date.now()}`,
      title: metadata.title,
      artist: metadata.artist,
      album: metadata.album,
      duration: metadata.duration_ms / 1000, // Convert ms to seconds
      source: source,
      metadata: {
        ...currentTrack?.metadata
      }
    };

    playerStore.setCurrentTrack(updatedTrack);
    playerStore.setDuration(updatedTrack.duration);
    playerStore.setPlaying(metadata.is_playing);

    // Update position if available
    if (metadata.progress_ms !== undefined && metadata.progress_ms !== null) {
      playerStore.setPosition(metadata.progress_ms / 1000);
    }
  }

  function startStreamingMetadataPolling() {
    if (streamingMetadataInterval === null && currentTrack && 
        (currentTrack.source === 'spotify' || currentTrack.source === 'youtube')) {
      // Poll every 2 seconds as per requirements
      streamingMetadataInterval = window.setInterval(pollStreamingMetadata, 2000);
      // Also poll immediately
      pollStreamingMetadata();
    }
  }

  function stopStreamingMetadataPolling() {
    if (streamingMetadataInterval !== null) {
      clearInterval(streamingMetadataInterval);
      streamingMetadataInterval = null;
    }
  }

  // Watch for track changes to start/stop streaming metadata polling
  $effect(() => {
    if (currentTrack && (currentTrack.source === 'spotify' || currentTrack.source === 'youtube')) {
      startStreamingMetadataPolling();
    } else {
      stopStreamingMetadataPolling();
    }
  });

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
    stopStreamingMetadataPolling();
  });
</script>

<div class="player">
  <!-- Hidden audio element -->
  <audio
    bind:this={audioElement}
    onloadedmetadata={handleLoadedMetadata}
    onplay={handlePlay}
    onpause={handlePause}
    onended={handleEnded}
    ontimeupdate={handleTimeUpdate}
    onerror={handleError}
  ></audio>

  <!-- Track info display -->
  <div class="track-info">
    {#if currentTrack}
      <div class="track-header">
        <div class="track-title">{currentTrack.title}</div>
        <div class="source-indicator source-{currentTrack.source}">
          {#if currentTrack.source === 'spotify'}
            <span class="source-icon">♫</span>
            <span class="source-label">Spotify</span>
          {:else if currentTrack.source === 'youtube'}
            <span class="source-icon">▶</span>
            <span class="source-label">YouTube</span>
          {:else}
            <span class="source-icon">🎵</span>
            <span class="source-label">Local</span>
          {/if}
        </div>
      </div>
      <div class="track-artist">{currentTrack.artist}</div>
      <div class="track-album">{currentTrack.album}</div>
    {:else}
      <div class="no-track">No track loaded</div>
    {/if}
  </div>

  <!-- Playback controls -->
  <div class="controls">
    <button class="control-btn" onclick={previous} title="Previous">
      <span class="icon">⏮</span>
    </button>

    {#if isPlaying}
      <button class="control-btn play-pause" onclick={pause} title="Pause">
        <span class="icon">⏸</span>
      </button>
    {:else}
      <button class="control-btn play-pause" onclick={() => play()} title="Play">
        <span class="icon">▶</span>
      </button>
    {/if}

    <button class="control-btn" onclick={stop} title="Stop">
      <span class="icon">⏹</span>
    </button>

    <button class="control-btn" onclick={next} title="Next">
      <span class="icon">⏭</span>
    </button>
  </div>

  <!-- Position bar -->
  <div class="position-section">
    <span class="time-display">{formatTime(position)}</span>
    <div
      class="seek-bar"
      bind:this={seekBarElement}
      onclick={handleSeekBarClick}
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
      oninput={(e) => setVolume(parseFloat(e.currentTarget.value))}
      class="volume-slider"
      aria-label="Volume"
    />
    <span class="volume-value">{Math.round(volume * 100)}%</span>
  </div>

  <!-- Queue info -->
  <div class="queue-info">
    <span>Queue: {queue.length} track{queue.length !== 1 ? 's' : ''}</span>
  </div>

  <!-- Visualizer -->
  <Visualizer 
    bind:this={visualizerComponent}
    {audioElement}
    style="bars"
    width={600}
    height={150}
    useSystemAudio={currentTrack?.source === 'spotify' || currentTrack?.source === 'youtube'}
  />
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

  .track-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 8px;
  }

  .track-title {
    font-size: 16px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .source-indicator {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 2px 8px;
    border-radius: 12px;
    font-size: 11px;
    font-weight: 500;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .source-indicator.source-local {
    background-color: rgba(100, 100, 255, 0.2);
    color: #8888ff;
    border: 1px solid rgba(100, 100, 255, 0.4);
  }

  .source-indicator.source-spotify {
    background-color: rgba(30, 215, 96, 0.2);
    color: #1ed760;
    border: 1px solid rgba(30, 215, 96, 0.4);
  }

  .source-indicator.source-youtube {
    background-color: rgba(255, 0, 0, 0.2);
    color: #ff4444;
    border: 1px solid rgba(255, 0, 0, 0.4);
  }

  .source-icon {
    font-size: 12px;
  }

  .source-label {
    text-transform: uppercase;
    letter-spacing: 0.5px;
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
