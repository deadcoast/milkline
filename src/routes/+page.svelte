<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { playerStore, configStore, farmerStore } from '$lib/stores';
  import { loadConfig, isFirstRun, applySkin } from '$lib/tauri';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { startSpotifyAuthMonitor, startYoutubeAuthMonitor, stopAllAuthMonitors } from '$lib/utils/authMonitor';
  import Player from '$lib/components/Player.svelte';
  import Playlist from '$lib/components/Playlist.svelte';
  import FarmerBuddy from '$lib/components/FarmerBuddy.svelte';
  import SkinRenderer from '$lib/components/SkinRenderer.svelte';
  import Visualizer from '$lib/components/Visualizer.svelte';
  import SetupWizard from '$lib/components/SetupWizard.svelte';
  import StreamingAuthDialog from '$lib/components/StreamingAuthDialog.svelte';
  import '$lib/styles/global.css';

  let initialized = $state(false);
  let error = $state<string | null>(null);
  let showSetup = $state(false);
  let showAuthDialog = $state(false);
  let authService = $state<'spotify' | 'youtube'>('spotify');
  let visualizerComponent = $state<any>(null);
  let audioElement = $state<HTMLAudioElement | null>(null);
  let analyzerNode = $state<AnalyserNode | null>(null);
  let isDraggingOver = $state(false);
  let unlistenLoadSkin: UnlistenFn | null = null;

  // Subscribe to player state to sync visualizer
  let currentTrack = $derived($playerStore.currentTrack);
  let isPlaying = $derived($playerStore.isPlaying);

  onMount(async () => {
    try {
      // Check if this is the first run
      const firstRun = await isFirstRun();
      
      if (firstRun) {
        showSetup = true;
        initialized = true;
        return;
      }

      // Load configuration on app startup
      const config = await loadConfig();
      configStore.setConfig(config);
      
      // Sync volume from config to player
      playerStore.setVolume(config.volume);
      
      initialized = true;
      
      // Listen for load-skin-file events from backend (file associations)
      unlistenLoadSkin = await listen<string>('load-skin-file', async (event) => {
        console.log('Received load-skin-file event:', event.payload);
        await handleSkinLoad(event.payload);
      });

      // Start authentication monitors if streaming services are enabled
      if (config.spotifyEnabled) {
        startSpotifyAuthMonitor({
          onSpotifyAuthRequired: () => {
            authService = 'spotify';
            showAuthDialog = true;
          }
        });
      }

      if (config.youtubeEnabled) {
        startYoutubeAuthMonitor({
          onYoutubeAuthRequired: () => {
            authService = 'youtube';
            showAuthDialog = true;
          }
        });
      }
    } catch (err) {
      console.error('Failed to initialize app:', err);
      error = err instanceof Error ? err.message : 'Unknown error';
      // Continue with default config
      initialized = true;
    }
  });

  onDestroy(() => {
    // Clean up event listener
    if (unlistenLoadSkin) {
      unlistenLoadSkin();
    }

    // Stop all authentication monitors
    stopAllAuthMonitors();
  });

  function handleSetupComplete() {
    showSetup = false;
    // Reload config after setup
    loadConfig().then(config => {
      configStore.setConfig(config);
      playerStore.setVolume(config.volume);
    }).catch(err => {
      console.error('Failed to load config after setup:', err);
    });
  }

  // Handle skin file loading
  async function handleSkinLoad(skinPath: string) {
    try {
      console.log('Loading skin:', skinPath);
      farmerStore.transition('prompting', 'Loading new skin...');
      
      const skin = await applySkin(skinPath);
      
      // Update config with new skin
      const config = await loadConfig();
      configStore.setConfig(config);
      
      farmerStore.transition('celebrating', 'Skin loaded successfully!');
      setTimeout(() => {
        farmerStore.transition('idle');
      }, 2000);
    } catch (err) {
      console.error('Failed to load skin:', err);
      farmerStore.transition('error', 'Failed to load skin. Using default.');
      setTimeout(() => {
        farmerStore.transition('idle');
      }, 3000);
    }
  }

  // Drag and drop handlers
  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    if (event.dataTransfer) {
      event.dataTransfer.dropEffect = 'copy';
    }
    isDraggingOver = true;
  }

  function handleDragLeave(event: DragEvent) {
    event.preventDefault();
    isDraggingOver = false;
  }

  async function handleDrop(event: DragEvent) {
    event.preventDefault();
    isDraggingOver = false;

    if (!event.dataTransfer) return;

    const files = Array.from(event.dataTransfer.files);
    
    // Find the first skin file (.wsz or .wal)
    const skinFile = files.find(file => 
      file.name.toLowerCase().endsWith('.wsz') || 
      file.name.toLowerCase().endsWith('.wal')
    );

    if (skinFile) {
      // Get the file path - in Tauri, we need to use the file path
      // The file object from drag-and-drop contains the path
      const filePath = (skinFile as any).path;
      
      if (filePath) {
        await handleSkinLoad(filePath);
      } else {
        console.error('Could not get file path from dropped file');
        farmerStore.transition('error', 'Could not load skin file.');
        setTimeout(() => {
          farmerStore.transition('idle');
        }, 3000);
      }
    }
  }

  // Watch for playback state changes to sync visualizer
  $effect(() => {
    if (visualizerComponent) {
      if (isPlaying) {
        visualizerComponent.start();
      } else {
        visualizerComponent.stop();
      }
      // Update analyzer node for farmer
      analyzerNode = visualizerComponent.getAnalyzerNode();
    }
  });

  function openSpotifyAuth() {
    authService = 'spotify';
    showAuthDialog = true;
  }

  function openYoutubeAuth() {
    authService = 'youtube';
    showAuthDialog = true;
  }

  function handleAuthSuccess() {
    // Reload config to reflect updated streaming service settings
    loadConfig().then(config => {
      configStore.setConfig(config);

      // Restart auth monitors for the authenticated service
      if (authService === 'spotify' && config.spotifyEnabled) {
        startSpotifyAuthMonitor({
          onSpotifyAuthRequired: () => {
            authService = 'spotify';
            showAuthDialog = true;
          }
        });
      } else if (authService === 'youtube' && config.youtubeEnabled) {
        startYoutubeAuthMonitor({
          onYoutubeAuthRequired: () => {
            authService = 'youtube';
            showAuthDialog = true;
          }
        });
      }
    }).catch(err => {
      console.error('Failed to reload config after auth:', err);
    });
  }
</script>

<div 
  class="app-container"
  class:dragging-over={isDraggingOver}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  role="application"
>
  {#if showSetup}
    <SetupWizard onComplete={handleSetupComplete} />
  {:else if !initialized}
    <div class="loading">
      <p>Loading milk player...</p>
    </div>
  {:else if error}
    <div class="error">
      <p>Failed to load configuration: {error}</p>
      <p>Using default settings...</p>
    </div>
  {:else}
    <!-- Skin renderer (applies skin globally) -->
    <SkinRenderer skinPath={$configStore.lastSkin} />
    
    <!-- Settings menu -->
    <div class="settings-menu">
      <button class="settings-button" title="Settings">⚙️</button>
      <div class="settings-dropdown">
        <button onclick={openSpotifyAuth}>
          <span class="icon">🎵</span>
          Connect Spotify
        </button>
        <button onclick={openYoutubeAuth}>
          <span class="icon">📺</span>
          Connect YouTube
        </button>
      </div>
    </div>
    
    <main class="main-layout">
      <div class="player-section">
        <Player bind:audioElement />
      </div>
      
      <div class="playlist-section">
        <Playlist />
      </div>
      
      <div class="visualizer-section">
        <Visualizer 
          bind:this={visualizerComponent}
          {audioElement}
          style={$configStore.visualizerStyle || 'bars'}
          width={800}
          height={400}
          useSystemAudio={currentTrack?.source === 'spotify' || currentTrack?.source === 'youtube'}
        />
      </div>
      
      <div class="farmer-section">
        <FarmerBuddy {analyzerNode} />
      </div>
    </main>
    
    {#if isDraggingOver}
      <div class="drop-overlay">
        <div class="drop-message">
          <p>Drop skin file here</p>
          <p class="drop-hint">(.wsz or .wal files)</p>
        </div>
      </div>
    {/if}

    {#if showAuthDialog}
      <StreamingAuthDialog 
        service={authService}
        onClose={() => showAuthDialog = false}
        onSuccess={handleAuthSuccess}
      />
    {/if}
  {/if}
</div>

<style>
  .app-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--color-background);
    position: relative;
  }

  .app-container.dragging-over {
    outline: 3px dashed var(--color-accent);
    outline-offset: -10px;
  }

  .loading,
  .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-lg);
    color: var(--color-text);
    font-size: var(--font-size-md);
  }

  .error {
    background-color: var(--color-surface);
    border: var(--border-width) solid var(--color-accent);
    margin: var(--spacing-md);
    border-radius: var(--border-radius-md);
  }

  .main-layout {
    flex: 1;
    display: grid;
    grid-template-columns: auto 1fr;
    grid-template-rows: auto 1fr;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm);
    overflow: hidden;
  }

  .player-section {
    grid-column: 1;
    grid-row: 1;
    background-color: var(--color-player-bg);
    border: var(--border-width) solid var(--color-border);
    border-radius: var(--border-radius-sm);
  }

  .playlist-section {
    grid-column: 1;
    grid-row: 2;
    background-color: var(--color-surface);
    border: var(--border-width) solid var(--color-border);
    border-radius: var(--border-radius-sm);
    overflow: hidden;
  }

  .visualizer-section {
    grid-column: 2;
    grid-row: 1 / 3;
    background-color: var(--color-visualizer-bg);
    border: var(--border-width) solid var(--color-border);
    border-radius: var(--border-radius-sm);
  }

  .farmer-section {
    position: fixed;
    bottom: var(--spacing-md);
    right: var(--spacing-md);
    width: 150px;
    height: 150px;
    z-index: var(--z-tooltip);
  }

  .settings-menu {
    position: fixed;
    top: var(--spacing-md);
    right: var(--spacing-md);
    z-index: var(--z-tooltip);
  }

  .settings-button {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background: var(--color-surface);
    border: var(--border-width) solid var(--color-border);
    font-size: 24px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .settings-button:hover {
    background: var(--color-accent);
    transform: rotate(90deg);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }

  .settings-dropdown {
    position: absolute;
    top: 56px;
    right: 0;
    background: var(--color-surface);
    border: var(--border-width) solid var(--color-border);
    border-radius: var(--border-radius-md);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    min-width: 200px;
    opacity: 0;
    visibility: hidden;
    transform: translateY(-10px);
    transition: all 0.2s;
  }

  .settings-menu:hover .settings-dropdown {
    opacity: 1;
    visibility: visible;
    transform: translateY(0);
  }

  .settings-dropdown button {
    width: 100%;
    padding: 12px 16px;
    background: none;
    border: none;
    text-align: left;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 12px;
    color: var(--color-text);
    font-size: 14px;
    transition: background 0.2s;
  }

  .settings-dropdown button:hover {
    background: var(--color-player-bg);
  }

  .settings-dropdown button:first-child {
    border-radius: var(--border-radius-md) var(--border-radius-md) 0 0;
  }

  .settings-dropdown button:last-child {
    border-radius: 0 0 var(--border-radius-md) var(--border-radius-md);
  }

  .settings-dropdown button .icon {
    font-size: 18px;
  }

  .drop-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    pointer-events: none;
  }

  .drop-message {
    background-color: var(--color-surface);
    border: 3px dashed var(--color-accent);
    border-radius: var(--border-radius-lg);
    padding: var(--spacing-xl);
    text-align: center;
  }

  .drop-message p {
    margin: 0;
    color: var(--color-text);
    font-size: var(--font-size-lg);
    font-weight: bold;
  }

  .drop-hint {
    margin-top: var(--spacing-sm) !important;
    font-size: var(--font-size-sm) !important;
    font-weight: normal !important;
    opacity: 0.7;
  }
</style>
