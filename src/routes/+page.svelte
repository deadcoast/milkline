<script lang="ts">
  import { onMount } from 'svelte';
  import { playerStore, playlistStore, configStore } from '$lib/stores';
  import { loadConfig, isFirstRun } from '$lib/tauri';
  import Player from '$lib/components/Player.svelte';
  import SetupWizard from '$lib/components/SetupWizard.svelte';
  import '$lib/styles/global.css';

  let initialized = $state(false);
  let error = $state<string | null>(null);
  let showSetup = $state(false);

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
    } catch (err) {
      console.error('Failed to initialize app:', err);
      error = err instanceof Error ? err.message : 'Unknown error';
      // Continue with default config
      initialized = true;
    }
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
</script>

<div class="app-container">
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
    <main class="main-layout">
      <div class="player-section">
        <Player />
      </div>
      
      <div class="playlist-section">
        <!-- Playlist component will go here -->
        <div class="placeholder">Playlist</div>
      </div>
      
      <div class="visualizer-section">
        <!-- Visualizer component will go here -->
        <div class="placeholder">Visualizer</div>
      </div>
      
      <div class="farmer-section">
        <!-- Farmer buddy component will go here -->
        <div class="placeholder">Farmer</div>
      </div>
    </main>
  {/if}
</div>

<style>
  .app-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--color-background);
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

  .placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    color: var(--color-text-secondary);
    font-size: var(--font-size-sm);
    opacity: 0.5;
  }
</style>
