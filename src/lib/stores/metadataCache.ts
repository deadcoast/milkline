// Metadata cache store with LRU eviction and memory optimization
import { writable } from 'svelte/store';
import type { Track } from '../types';

interface CacheEntry {
    metadata: Track['metadata'];
    timestamp: number;
    accessCount: number;
}

interface MetadataCacheState {
    cache: Map<string, CacheEntry>;
    maxSize: number;
}

// Optimized cache size based on memory constraints
// Target: ~100MB RAM usage, each entry ~100 bytes = ~500 entries max
const MAX_CACHE_SIZE = 500;
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
        get: (filePath: string): Track['metadata'] | null => {
            let result: TrackMetadata | null = null;
            
            update(state => {
                const entry = state.cache.get(filePath);
                
                if (entry) {
                    // Check if entry is still valid (not expired)
                    const now = Date.now();
                    if (now - entry.timestamp < CACHE_TTL) {
                        result = entry.metadata;
                        
                        // Update access time and count (LRU with frequency)
                        state.cache.delete(filePath);
                        state.cache.set(filePath, { 
                            ...entry, 
                            timestamp: now,
                            accessCount: entry.accessCount + 1
                        });
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
        set: (filePath: string, metadata: Track['metadata']) => {
            update(state => {
                // If cache is full, evict least recently used entry
                if (state.cache.size >= state.maxSize) {
                    // Find entry with oldest timestamp and lowest access count
                    let oldestKey: string | null = null;
                    let oldestTime = Date.now();
                    let lowestAccess = Infinity;
                    
                    for (const [key, entry] of state.cache.entries()) {
                        if (entry.timestamp < oldestTime || 
                            (entry.timestamp === oldestTime && entry.accessCount < lowestAccess)) {
                            oldestKey = key;
                            oldestTime = entry.timestamp;
                            lowestAccess = entry.accessCount;
                        }
                    }
                    
                    if (oldestKey) {
                        state.cache.delete(oldestKey);
                    }
                }
                
                // Add new entry
                state.cache.set(filePath, {
                    metadata,
                    timestamp: Date.now(),
                    accessCount: 0
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
        },
        
        // Get cache statistics
        getStats: () => {
            let stats = {
                size: 0,
                maxSize: MAX_CACHE_SIZE,
                totalAccesses: 0,
                averageAccessCount: 0
            };
            
            update(state => {
                stats.size = state.cache.size;
                let totalAccess = 0;
                
                for (const entry of state.cache.values()) {
                    totalAccess += entry.accessCount;
                }
                
                stats.totalAccesses = totalAccess;
                stats.averageAccessCount = state.cache.size > 0 
                    ? totalAccess / state.cache.size 
                    : 0;
                
                return state;
            });
            
            return stats;
        }
    };
}

export const metadataCache = createMetadataCacheStore();
