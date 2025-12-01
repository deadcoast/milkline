<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import ImageEditor from './ImageEditor.svelte';
  import VideoEditor from './VideoEditor.svelte';
  import { mediaEditorStore } from '$lib/stores/mediaEditorStore';

  // State
  let currentFilePath = $state<string | null>(null);
  let currentMediaType = $state<'image' | 'video' | null>(null);
  let imageEditorRef: any = $state(null);
  let videoEditorRef: any = $state(null);

  // Subscribe to store
  $effect(() => {
    const unsubscribe = mediaEditorStore.subscribe(state => {
      currentFilePath = state.filePath;
      currentMediaType = state.mediaType;
    });
    return unsubscribe;
  });

  /**
   * Determine media type from file extension
   */
  function getMediaTypeFromExtension(filePath: string): 'image' | 'video' | null {
    const extension = filePath.split('.').pop()?.toLowerCase();
    
    const imageExtensions = ['png', 'jpg', 'jpeg', 'bmp', 'gif'];
    const videoExtensions = ['mp4', 'mov', 'mkv'];
    
    if (extension && imageExtensions.includes(extension)) {
      return 'image';
    } else if (extension && videoExtensions.includes(extension)) {
      return 'video';
    }
    
    return null;
  }

  /**
   * Handle file open
   */
  async function handleOpen() {
    try {
      // Use Tauri dialog command
      const selected = await invoke<string | null>('show_open_dialog', {
        filters: [
          {
            name: 'Media Files',
            extensions: ['png', 'jpg', 'jpeg', 'bmp', 'gif', 'mp4', 'mov', 'mkv']
          }
        ]
      });

      if (selected) {
        const mediaType = getMediaTypeFromExtension(selected);
        
        if (!mediaType) {
          await invoke('show_message_dialog', {
            title: 'Error',
            message: 'Unsupported file format. Please select an image (png, jpg, jpeg, bmp, gif) or video (mp4, mov, mkv) file.',
            kind: 'error'
          });
          return;
        }

        // Load media into store
        mediaEditorStore.loadMedia(selected, mediaType);
      }
    } catch (error) {
      await invoke('show_message_dialog', {
        title: 'Error',
        message: `Failed to open file: ${error}`,
        kind: 'error'
      });
    }
  }

  /**
   * Handle file save
   */
  async function handleSave() {
    // Check if media is loaded
    if (!currentFilePath || !currentMediaType) {
      await invoke('show_message_dialog', {
        title: 'Info',
        message: 'No media file is loaded. Please open a file first.',
        kind: 'info'
      });
      return;
    }

    try {
      // Show save dialog
      const outputPath = await invoke<string | null>('show_save_dialog', {
        filters: [
          {
            name: currentMediaType === 'image' ? 'Image Files' : 'Video Files',
            extensions: currentMediaType === 'image' 
              ? ['png', 'jpg', 'jpeg', 'bmp', 'gif']
              : ['mp4', 'mov', 'mkv']
          }
        ]
      });

      if (!outputPath) {
        // User cancelled
        return;
      }

      // Delegate to appropriate editor export method
      if (currentMediaType === 'image' && imageEditorRef) {
        await imageEditorRef.exportImage(outputPath);
        await invoke('show_message_dialog', {
          title: 'Success',
          message: 'Image exported successfully!',
          kind: 'info'
        });
      } else if (currentMediaType === 'video' && videoEditorRef) {
        await videoEditorRef.exportVideo(outputPath);
        await invoke('show_message_dialog', {
          title: 'Success',
          message: 'Video exported successfully!',
          kind: 'info'
        });
      }
    } catch (error) {
      await invoke('show_message_dialog', {
        title: 'Error',
        message: `Export failed: ${error}`,
        kind: 'error'
      });
    }
  }
</script>

<div class="media-editor-window">
  <div class="menu-bar">
    <button class="menu-button" onclick={handleOpen}>Open</button>
    <button class="menu-button" onclick={handleSave}>Save As</button>
  </div>

  <div class="editor-content">
    {#if currentMediaType === 'image' && currentFilePath}
      <ImageEditor bind:this={imageEditorRef} filePath={currentFilePath} />
    {:else if currentMediaType === 'video' && currentFilePath}
      <VideoEditor bind:this={videoEditorRef} filePath={currentFilePath} />
    {:else}
      <div class="no-media">
        <p>No media loaded</p>
        <p class="hint">Click "Open" to load an image or video file</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .media-editor-window {
    width: 100%;
    height: 100vh;
    display: flex;
    flex-direction: column;
    background-color: #1a1a1a;
  }

  .menu-bar {
    display: flex;
    gap: 8px;
    padding: 8px;
    background-color: #252525;
    border-bottom: 1px solid #444;
  }

  .menu-button {
    padding: 8px 16px;
    background-color: #333;
    color: #fff;
    border: 1px solid #555;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: background-color 0.2s;
  }

  .menu-button:hover {
    background-color: #444;
  }

  .menu-button:active {
    background-color: #555;
  }

  .editor-content {
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  .no-media {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: #888;
  }

  .no-media p {
    margin: 8px 0;
    font-size: 1.2rem;
  }

  .hint {
    font-size: 1rem;
    color: #666;
  }
</style>
