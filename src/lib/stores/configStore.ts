// Configuration state management store
import { writable } from 'svelte/store';
import type { AppConfig } from '../types';

const initialConfigState: AppConfig = {
    libraryPath: null,
    lastSkin: null,
    volume: 0.7,
    visualizerStyle: 'bars',
    spotifyEnabled: false,
    youtubeEnabled: false,
    windowPosition: { x: 100, y: 100 },
    windowSize: { width: 800, height: 600 }
};

function createConfigStore() {
    const { subscribe, set, update } = writable<AppConfig>(initialConfigState);

    return {
        subscribe,
        setConfig: (config: AppConfig) => set(config),
        updateConfig: (updates: Partial<AppConfig>) => update(state => ({ ...state, ...updates })),
        setLibraryPath: (path: string | null) => update(state => ({ ...state, libraryPath: path })),
        setLastSkin: (skin: string | null) => update(state => ({ ...state, lastSkin: skin })),
        setVolume: (volume: number) => update(state => ({ ...state, volume })),
        setVisualizerStyle: (style: 'bars' | 'waveform' | 'spectrum') => update(state => ({ ...state, visualizerStyle: style })),
        setSpotifyEnabled: (enabled: boolean) => update(state => ({ ...state, spotifyEnabled: enabled })),
        setYoutubeEnabled: (enabled: boolean) => update(state => ({ ...state, youtubeEnabled: enabled })),
        setWindowPosition: (position: { x: number; y: number }) => update(state => ({ ...state, windowPosition: position })),
        setWindowSize: (size: { width: number; height: number }) => update(state => ({ ...state, windowSize: size })),
        reset: () => set(initialConfigState)
    };
}

export const configStore = createConfigStore();
