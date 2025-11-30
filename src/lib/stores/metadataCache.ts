// Metadata cache store with LRU eviction
import { writable } from 'svelte/store';
import type { TrackMetadata } from '../types';

interface CacheEntry {
    metadata: TrackMetadata;
    timestamp: number;
}

interface MetadataCacheState {
    cache: Map<string, CacheEntry>;
    maxSize: number;
}

const MAX_CACHE_SIZE = 1000;
const CACHE_TTL = 3600000; // 1 hour in milliseconds

function createMetadataCacheStore() {
    const initialState: MetadataCacheState = {
        cache: new Map(),
        maxSize: MAX_CACHE_SIZE
    };

    const { subscribe, update } = writable<MetadataCacheState>(initialState);

    return {
        subscribe,
        
        // Get metadata from cache
        get: (filePath: string): TrackMetadata | null => {
            let result: TrackMetadata | null = null;
            
            update(state => {
                const entry = state.cache.get(filePath);
                
                if (entry) {
                    // Check if entry is still valid (not expired)
                    const now = Date.now();
                    if (now - entry.timestamp < CACHE_TTL) {
                        result = entry.metadata;
                        
                        // Update access time (LRU)
                        state.cache.delete(filePath);
                        state.cache.set(filePath, { ...entry, timestamp: now });
                    } else {
                        // Entry expired, remove it
                        state.cache.delete(filePath);
                    }
                }
                
                return state;
            });
            
            return result;
        },
        
        // Set metadata in cache
        set: (filePath: string, metadata: TrackMetadata) => {
            update(state => {
                // If cache is full, remove oldest entry (first in Map)
                if (state.cache.size >= state.maxSize) {
                    const firstKey = state.cache.keys().next().value;
                    if (firstKey) {
                        state.cache.delete(firstKey);
                    }
                }
                
                // Add new entry
                state.cache.set(filePath, {
                    metadata,
                    timestamp: Date.now()
                });
                
                return state;
            });
        },
        
        // Check if metadata is cached
        has: (filePath: string): boolean => {
            let exists = false;
            
            update(state => {
                const entry = state.cache.get(filePath);
                if (entry) {
                    const now = Date.now();
                    if (now - entry.timestamp < CACHE_TTL) {
                        exists = true;
                    } else {
                        // Entry expired, remove it
                        state.cache.delete(filePath);
                    }
                }
                return state;
            });
            
            return exists;
        },
        
        // Clear cache
        clear: () => {
            update(state => {
                state.cache.clear();
                return state;
            });
        },
        
        // Get cache size
        size: (): number => {
            let cacheSize = 0;
            update(state => {
                cacheSize = state.cache.size;
                return state;
            });
            return cacheSize;
        }
    };
}

export const metadataCache = createMetadataCacheStore();
