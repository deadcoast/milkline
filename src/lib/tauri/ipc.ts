// Tauri IPC client wrapper functions
import { invoke } from '@tauri-apps/api/core';
import type { Track, Playlist, AppConfig } from '../types';
import { handleError } from '../utils/errorHandler';

// Configuration commands
export async function loadConfig(): Promise<AppConfig> {
    return await invoke<AppConfig>('load_config');
}

export async function saveConfig(config: AppConfig): Promise<void> {
    await invoke('save_config', { config });
}

export async function isFirstRun(): Promise<boolean> {
    return await invoke<boolean>('is_first_run');
}

export async function validateDirectoryPath(path: string): Promise<boolean> {
    return await invoke<boolean>('validate_directory_path', { path });
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

export async function listPlaylists(): Promise<Playlist[]> {
    return await invoke<Playlist[]>('list_playlists');
}

export async function loadPlaylist(playlistId: string): Promise<Playlist> {
    return await invoke<Playlist>('load_playlist', { playlistId });
}

export async function deletePlaylist(playlistId: string): Promise<void> {
    await invoke('delete_playlist', { playlistId });
}

export async function addTrackToPlaylist(playlistId: string, track: Track): Promise<Playlist> {
    return await invoke<Playlist>('add_track_to_playlist', { playlistId, track });
}

export async function removeTrackFromPlaylist(playlistId: string, trackId: string): Promise<Playlist> {
    return await invoke<Playlist>('remove_track_from_playlist', { playlistId, trackId });
}

export async function reorderPlaylistTracks(playlistId: string, trackIds: string[]): Promise<Playlist> {
    return await invoke<Playlist>('reorder_playlist_tracks', { playlistId, trackIds });
}

export async function updatePlaylist(playlistId: string, name?: string): Promise<Playlist> {
    return await invoke<Playlist>('update_playlist', { playlistId, name });
}

// Skin commands
export async function loadSkin(skinPath: string): Promise<import('../types').ParsedSkin> {
    return await invoke('load_skin', { skinPath });
}

export async function applySkin(skinPath: string): Promise<import('../types').ParsedSkin> {
    return await invoke('apply_skin', { skinPath });
}

// Spotify streaming service commands
export interface SpotifyCredentials {
    client_id: string;
    client_secret: string;
    redirect_uri: string;
}

export interface SpotifyToken {
    access_token: string;
    token_type: string;
    expires_in: number;
    refresh_token?: string;
    scope?: string;
}

export interface SpotifyTrackMetadata {
    title: string;
    artist: string;
    album: string;
    duration_ms: number;
    is_playing: boolean;
    progress_ms?: number;
}

export async function spotifyAuthenticate(credentials: SpotifyCredentials, authCode: string): Promise<SpotifyToken> {
    return await invoke<SpotifyToken>('spotify_authenticate', { credentials, authCode });
}

export async function spotifyGetNowPlaying(): Promise<SpotifyTrackMetadata | null> {
    return await invoke<SpotifyTrackMetadata | null>('spotify_get_now_playing');
}

export async function youtubeGetNowPlaying(): Promise<SpotifyTrackMetadata | null> {
    return await invoke<SpotifyTrackMetadata | null>('youtube_get_now_playing');
}

export async function spotifyRefreshToken(credentials: SpotifyCredentials): Promise<SpotifyToken> {
    return await invoke<SpotifyToken>('spotify_refresh_token', { credentials });
}

// YouTube streaming service commands (placeholder for future implementation)
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


// Error-handling wrappers for common operations

/**
 * Load config with automatic error handling
 */
export async function loadConfigSafe(): Promise<AppConfig | null> {
    try {
        return await loadConfig();
    } catch (error) {
        handleError(error, 'Failed to load configuration');
        return null;
    }
}

/**
 * Save config with automatic error handling
 */
export async function saveConfigSafe(config: AppConfig): Promise<boolean> {
    try {
        await saveConfig(config);
        return true;
    } catch (error) {
        handleError(error, 'Failed to save configuration');
        return false;
    }
}

/**
 * Scan library with automatic error handling
 */
export async function scanLibrarySafe(path: string): Promise<Track[]> {
    try {
        return await scanLibrary(path);
    } catch (error) {
        handleError(error, 'Failed to scan library');
        return [];
    }
}

/**
 * Load skin with automatic error handling and fallback
 */
export async function loadSkinSafe(skinPath: string): Promise<import('../types').ParsedSkin | null> {
    try {
        return await loadSkin(skinPath);
    } catch (error) {
        handleError(error, 'Failed to load skin');
        return null;
    }
}

/**
 * Apply skin with automatic error handling
 */
export async function applySkinSafe(skinPath: string): Promise<import('../types').ParsedSkin | null> {
    try {
        return await applySkin(skinPath);
    } catch (error) {
        handleError(error, 'Failed to apply skin');
        return null;
    }
}

/**
 * Create playlist with automatic error handling
 */
export async function createPlaylistSafe(name: string): Promise<Playlist | null> {
    try {
        return await createPlaylist(name);
    } catch (error) {
        handleError(error, 'Failed to create playlist');
        return null;
    }
}

/**
 * Spotify authenticate with automatic error handling
 */
export async function spotifyAuthenticateSafe(credentials: SpotifyCredentials, authCode: string): Promise<SpotifyToken | null> {
    try {
        return await spotifyAuthenticate(credentials, authCode);
    } catch (error) {
        handleError(error, 'Spotify authentication failed');
        return null;
    }
}

/**
 * Get Spotify now playing with automatic error handling (silent for no playback)
 */
export async function spotifyGetNowPlayingSafe(): Promise<SpotifyTrackMetadata | null> {
    try {
        return await spotifyGetNowPlaying();
    } catch (error) {
        // Don't show error for "no active playback" - this is expected
        const errorMsg = String(error).toLowerCase();
        if (!errorMsg.includes('no active playback')) {
            handleError(error, 'Failed to get Spotify playback info');
        }
        return null;
    }
}
