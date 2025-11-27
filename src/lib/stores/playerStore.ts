// Player state management store
import { writable } from 'svelte/store';
import type { PlayerState, Track } from '../types';

const initialPlayerState: PlayerState = {
    currentTrack: null,
    isPlaying: false,
    position: 0,
    duration: 0,
    volume: 0.7,
    queue: []
};

function createPlayerStore() {
    const { subscribe, set, update } = writable<PlayerState>(initialPlayerState);

    return {
        subscribe,
        setCurrentTrack: (track: Track | null) => update(state => ({ ...state, currentTrack: track })),
        setPlaying: (isPlaying: boolean) => update(state => ({ ...state, isPlaying })),
        setPosition: (position: number) => update(state => ({ ...state, position })),
        setDuration: (duration: number) => update(state => ({ ...state, duration })),
        setVolume: (volume: number) => update(state => ({ ...state, volume })),
        setQueue: (queue: Track[]) => update(state => ({ ...state, queue })),
        addToQueue: (track: Track) => update(state => ({ ...state, queue: [...state.queue, track] })),
        removeFromQueue: (trackId: string) => update(state => ({
            ...state,
            queue: state.queue.filter(t => t.id !== trackId)
        })),
        clearQueue: () => update(state => ({ ...state, queue: [] })),
        reset: () => set(initialPlayerState)
    };
}

export const playerStore = createPlayerStore();
