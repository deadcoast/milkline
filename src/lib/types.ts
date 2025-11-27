// Type definitions for milk player

export interface Track {
    id: string;
    title: string;
    artist: string;
    album: string;
    duration: number;
    filePath?: string;
    source: 'local' | 'spotify' | 'youtube';
    metadata: {
        year?: number;
        genre?: string;
        trackNumber?: number;
        albumArt?: string;
    };
}

export interface Playlist {
    id: string;
    name: string;
    tracks: Track[];
    createdAt: Date;
    modifiedAt: Date;
}

export interface AppConfig {
    libraryPath: string | null;
    lastSkin: string | null;
    volume: number;
    visualizerStyle: 'bars' | 'waveform' | 'spectrum';
    spotifyEnabled: boolean;
    youtubeEnabled: boolean;
    windowPosition: { x: number; y: number };
    windowSize: { width: number; height: number };
}

export interface PlayerState {
    currentTrack: Track | null;
    isPlaying: boolean;
    position: number;
    duration: number;
    volume: number;
    queue: Track[];
}

export interface PlaylistState {
    playlists: Playlist[];
    currentPlaylist: Playlist | null;
}
