// Tauri IPC client wrapper functions
import { invoke } from '@tauri-apps/api/core';
import type { Track, Playlist, AppConfig } from '../types';

// Configuration commands
export async function loadConfig(): Promise<AppConfig> {
    return await invoke<AppConfig>('load_config');
}

export async function saveConfig(config: AppConfig): Promise<void> {
    await invoke('save_config', { config });
}

// Library commands
export async function scanLibrary(path: string): Promise<Track[]> {
    return await invoke<Track[]>('scan_library', { path });
}

// Metadata commands
export async function extractMetadata(filePath: string): Promise<Track> {
    return await invoke<Track>('extract_metadata', { filePath });
}

export async function extractArtwork(filePath: string): Promise<string | null> {
    return await invoke<string | null>('extract_artwork', { filePath });
}

// Playlist commands
export async function createPlaylist(name: string): Promise<Playlist> {
    return await invoke<Playlist>('create_playlist', { name });
}

export async function loadPlaylists(): Promise<Playlist[]> {
    return await invoke<Playlist[]>('load_playlists');
}

export async function savePlaylist(playlist: Playlist): Promise<void> {
    await invoke('save_playlist', { playlist });
}

export async function deletePlaylist(playlistId: string): Promise<void> {
    await invoke('delete_playlist', { playlistId });
}

// Skin commands
export async function loadSkin(skinPath: string): Promise<void> {
    await invoke('load_skin', { skinPath });
}

export async function applySkin(skinPath: string): Promise<void> {
    await invoke('apply_skin', { skinPath });
}

// Streaming service commands
export async function authenticateSpotify(credentials: any): Promise<void> {
    await invoke('authenticate_spotify', { credentials });
}

export async function getSpotifyNowPlaying(): Promise<Track | null> {
    return await invoke<Track | null>('get_spotify_now_playing');
}

export async function authenticateYoutube(credentials: any): Promise<void> {
    await invoke('authenticate_youtube', { credentials });
}

export async function getYoutubeNowPlaying(): Promise<Track | null> {
    return await invoke<Track | null>('get_youtube_now_playing');
}

// Secure storage commands
export async function storeCredential(key: string, value: string): Promise<void> {
    await invoke('store_credential', { key, value });
}

export async function retrieveCredential(key: string): Promise<string | null> {
    return await invoke<string | null>('retrieve_credential', { key });
}

export async function deleteCredential(key: string): Promise<void> {
    await invoke('delete_credential', { key });
}
