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

export interface SkinAssets {
    [key: string]: number[];  // Asset name → byte array
}

export interface Region {
    x: number;
    y: number;
    width: number;
    height: number;
}

export interface RegionConfig {
    main: Region;
}

export interface ParsedSkin {
    name: string;
    assets: SkinAssets;
    regions: RegionConfig | null;
}

export type FarmerState = 'idle' | 'listening' | 'prompting' | 'celebrating' | 'error';

export interface FarmerExpression {
    eyes: 'neutral' | 'blink' | 'look-left' | 'look-right';
    mouth: 'neutral' | 'smile' | 'talk-1' | 'talk-2';
}

export interface FarmerStateData {
    currentState: FarmerState;
    message: string | null;
    expression: FarmerExpression;
}
