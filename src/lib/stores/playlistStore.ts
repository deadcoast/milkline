// Playlist state management store
import { writable } from 'svelte/store';
import type { PlaylistState, Playlist, Track } from '../types';

const initialPlaylistState: PlaylistState = {
    playlists: [],
    currentPlaylist: null
};

function createPlaylistStore() {
    const { subscribe, set, update } = writable<PlaylistState>(initialPlaylistState);

    return {
        subscribe,
        setPlaylists: (playlists: Playlist[]) => update(state => ({ ...state, playlists })),
        setCurrentPlaylist: (playlist: Playlist | null) => update(state => ({ ...state, currentPlaylist: playlist })),
        addPlaylist: (playlist: Playlist) => update(state => ({
            ...state,
            playlists: [...state.playlists, playlist]
        })),
        updatePlaylist: (playlistId: string, updates: Partial<Playlist>) => update(state => ({
            ...state,
            playlists: state.playlists.map(p =>
                p.id === playlistId ? { ...p, ...updates, modifiedAt: new Date() } : p
            ),
            currentPlaylist: state.currentPlaylist?.id === playlistId
                ? { ...state.currentPlaylist, ...updates, modifiedAt: new Date() }
                : state.currentPlaylist
        })),
        removePlaylist: (playlistId: string) => update(state => ({
            ...state,
            playlists: state.playlists.filter(p => p.id !== playlistId),
            currentPlaylist: state.currentPlaylist?.id === playlistId ? null : state.currentPlaylist
        })),
        addTrackToPlaylist: (playlistId: string, track: Track) => update(state => ({
            ...state,
            playlists: state.playlists.map(p =>
                p.id === playlistId
                    ? { ...p, tracks: [...p.tracks, track], modifiedAt: new Date() }
                    : p
            ),
            currentPlaylist: state.currentPlaylist?.id === playlistId
                ? { ...state.currentPlaylist, tracks: [...state.currentPlaylist.tracks, track], modifiedAt: new Date() }
                : state.currentPlaylist
        })),
        removeTrackFromPlaylist: (playlistId: string, trackId: string) => update(state => ({
            ...state,
            playlists: state.playlists.map(p =>
                p.id === playlistId
                    ? { ...p, tracks: p.tracks.filter(t => t.id !== trackId), modifiedAt: new Date() }
                    : p
            ),
            currentPlaylist: state.currentPlaylist?.id === playlistId
                ? { ...state.currentPlaylist, tracks: state.currentPlaylist.tracks.filter(t => t.id !== trackId), modifiedAt: new Date() }
                : state.currentPlaylist
        })),
        reorderTracks: (playlistId: string, newOrder: Track[]) => update(state => ({
            ...state,
            playlists: state.playlists.map(p =>
                p.id === playlistId
                    ? { ...p, tracks: newOrder, modifiedAt: new Date() }
                    : p
            ),
            currentPlaylist: state.currentPlaylist?.id === playlistId
                ? { ...state.currentPlaylist, tracks: newOrder, modifiedAt: new Date() }
                : state.currentPlaylist
        })),
        reset: () => set(initialPlaylistState)
    };
}

export const playlistStore = createPlaylistStore();
