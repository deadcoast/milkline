<script lang="ts">
    import { playlistStore } from '../stores/playlistStore';
    import { playerStore } from '../stores/playerStore';
    import type { Track, Playlist as PlaylistType } from '../types';
    import { 
        createPlaylist, 
        listPlaylists, 
        loadPlaylist,
        deletePlaylist,
        addTrackToPlaylist,
        removeTrackFromPlaylist,
        reorderPlaylistTracks 
    } from '../tauri/ipc';
    import { onMount } from 'svelte';

    let newPlaylistName = '';
    let draggedTrackIndex: number | null = null;

    onMount(async () => {
        await loadAllPlaylists();
    });

    async function loadAllPlaylists() {
        try {
            const playlists = await listPlaylists();
            playlistStore.setPlaylists(playlists);
        } catch (error) {
            console.error('Failed to load playlists:', error);
        }
    }

    async function handleCreatePlaylist() {
        if (!newPlaylistName.trim()) return;
        
        try {
            const playlist = await createPlaylist(newPlaylistName);
            playlistStore.addPlaylist(playlist);
            newPlaylistName = '';
        } catch (error) {
            console.error('Failed to create playlist:', error);
        }
    }

    async function handleSelectPlaylist(playlist: PlaylistType) {
        try {
            const loadedPlaylist = await loadPlaylist(playlist.id);
            playlistStore.setCurrentPlaylist(loadedPlaylist);
        } catch (error) {
            console.error('Failed to load playlist:', error);
        }
    }

    async function handleDeletePlaylist(playlistId: string) {
        try {
            await deletePlaylist(playlistId);
            playlistStore.removePlaylist(playlistId);
        } catch (error) {
            console.error('Failed to delete playlist:', error);
        }
    }

    async function handleRemoveTrack(playlistId: string, trackId: string) {
        try {
            const updatedPlaylist = await removeTrackFromPlaylist(playlistId, trackId);
            playlistStore.updatePlaylist(playlistId, updatedPlaylist);
        } catch (error) {
            console.error('Failed to remove track:', error);
        }
    }

    async function handleLoadToQueue(playlist: PlaylistType) {
        // Load playlist tracks into player queue
        playerStore.setQueue(playlist.tracks);
        if (playlist.tracks.length > 0) {
            playerStore.setCurrentTrack(playlist.tracks[0]);
        }
    }

    function handleDragStart(event: DragEvent, index: number) {
        draggedTrackIndex = index;
        if (event.dataTransfer) {
            event.dataTransfer.effectAllowed = 'move';
        }
    }

    function handleDragOver(event: DragEvent) {
        event.preventDefault();
        if (event.dataTransfer) {
            event.dataTransfer.dropEffect = 'move';
        }
    }

    async function handleDrop(event: DragEvent, dropIndex: number, playlist: PlaylistType) {
        event.preventDefault();
        
        if (draggedTrackIndex === null || draggedTrackIndex === dropIndex) {
            draggedTrackIndex = null;
            return;
        }

        const tracks = [...playlist.tracks];
        const [draggedTrack] = tracks.splice(draggedTrackIndex, 1);
        tracks.splice(dropIndex, 0, draggedTrack);

        try {
            const trackIds = tracks.map(t => t.id);
            const updatedPlaylist = await reorderPlaylistTracks(playlist.id, trackIds);
            playlistStore.updatePlaylist(playlist.id, updatedPlaylist);
        } catch (error) {
            console.error('Failed to reorder tracks:', error);
        }

        draggedTrackIndex = null;
    }

    $: currentPlaylist = $playlistStore.currentPlaylist;
    $: playlists = $playlistStore.playlists;
</script>

<div class="playlist-container">
    <div class="playlist-sidebar">
        <h2>Playlists</h2>
        
        <div class="create-playlist">
            <input 
                type="text" 
                bind:value={newPlaylistName} 
                placeholder="New playlist name"
                on:keydown={(e) => e.key === 'Enter' && handleCreatePlaylist()}
            />
            <button on:click={handleCreatePlaylist}>Create</button>
        </div>

        <div class="playlist-list">
            {#each playlists as playlist}
                <div 
                    class="playlist-item"
                    class:active={currentPlaylist?.id === playlist.id}
                    on:click={() => handleSelectPlaylist(playlist)}
                >
                    <span class="playlist-name">{playlist.name}</span>
                    <span class="track-count">({playlist.tracks.length})</span>
                    <button 
                        class="delete-btn"
                        on:click|stopPropagation={() => handleDeletePlaylist(playlist.id)}
                    >
                        ×
                    </button>
                </div>
            {/each}
        </div>
    </div>

    <div class="playlist-content">
        {#if currentPlaylist}
            <div class="playlist-header">
                <h2>{currentPlaylist.name}</h2>
                <button on:click={() => handleLoadToQueue(currentPlaylist)}>
                    Load to Queue
                </button>
            </div>

            <div class="track-list">
                {#each currentPlaylist.tracks as track, index}
                    <div 
                        class="track-item"
                        draggable="true"
                        on:dragstart={(e) => handleDragStart(e, index)}
                        on:dragover={handleDragOver}
                        on:drop={(e) => handleDrop(e, index, currentPlaylist)}
                    >
                        <span class="track-number">{index + 1}</span>
                        <div class="track-info">
                            <div class="track-title">{track.title}</div>
                            <div class="track-artist">{track.artist} - {track.album}</div>
                        </div>
                        <span class="track-duration">
                            {Math.floor(track.duration / 60)}:{String(Math.floor(track.duration % 60)).padStart(2, '0')}
                        </span>
                        <button 
                            class="remove-btn"
                            on:click={() => handleRemoveTrack(currentPlaylist.id, track.id)}
                        >
                            Remove
                        </button>
                    </div>
                {/each}
            </div>
        {:else}
            <div class="empty-state">
                <p>Select a playlist to view tracks</p>
            </div>
        {/if}
    </div>
</div>

<style>
    .playlist-container {
        display: flex;
        height: 100%;
        gap: 1rem;
    }

    .playlist-sidebar {
        width: 250px;
        border-right: 1px solid #ccc;
        padding: 1rem;
    }

    .playlist-sidebar h2 {
        margin: 0 0 1rem 0;
        font-size: 1.2rem;
    }

    .create-playlist {
        display: flex;
        gap: 0.5rem;
        margin-bottom: 1rem;
    }

    .create-playlist input {
        flex: 1;
        padding: 0.5rem;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    .create-playlist button {
        padding: 0.5rem 1rem;
        background: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .create-playlist button:hover {
        background: #0056b3;
    }

    .playlist-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .playlist-item {
        display: flex;
        align-items: center;
        padding: 0.75rem;
        background: #f5f5f5;
        border-radius: 4px;
        cursor: pointer;
        transition: background 0.2s;
    }

    .playlist-item:hover {
        background: #e0e0e0;
    }

    .playlist-item.active {
        background: #007bff;
        color: white;
    }

    .playlist-name {
        flex: 1;
        font-weight: 500;
    }

    .track-count {
        font-size: 0.9rem;
        opacity: 0.7;
        margin-right: 0.5rem;
    }

    .delete-btn {
        background: transparent;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 0 0.5rem;
        opacity: 0.6;
    }

    .delete-btn:hover {
        opacity: 1;
    }

    .playlist-content {
        flex: 1;
        padding: 1rem;
    }

    .playlist-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .playlist-header h2 {
        margin: 0;
        font-size: 1.5rem;
    }

    .playlist-header button {
        padding: 0.5rem 1rem;
        background: #28a745;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .playlist-header button:hover {
        background: #218838;
    }

    .track-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .track-item {
        display: flex;
        align-items: center;
        padding: 0.75rem;
        background: #f9f9f9;
        border-radius: 4px;
        cursor: move;
        transition: background 0.2s;
    }

    .track-item:hover {
        background: #f0f0f0;
    }

    .track-number {
        width: 30px;
        text-align: center;
        font-weight: 500;
        opacity: 0.6;
    }

    .track-info {
        flex: 1;
        margin-left: 1rem;
    }

    .track-title {
        font-weight: 500;
        margin-bottom: 0.25rem;
    }

    .track-artist {
        font-size: 0.9rem;
        opacity: 0.7;
    }

    .track-duration {
        margin-right: 1rem;
        font-size: 0.9rem;
        opacity: 0.7;
    }

    .remove-btn {
        padding: 0.25rem 0.75rem;
        background: #dc3545;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.9rem;
    }

    .remove-btn:hover {
        background: #c82333;
    }

    .empty-state {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
        color: #999;
    }
</style>
