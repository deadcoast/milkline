import { describe, it, expect, beforeEach } from "vitest";
import { get } from "svelte/store";
import { playlistStore } from "../stores/playlistStore";
import { playerStore } from "../stores/playerStore";
import type { Playlist, Track } from "../types";
import fc from "fast-check";

// Helper to create arbitrary tracks
const arbTrack = (): fc.Arbitrary<Track> => {
  return fc.record({
    id: fc.uuid(),
    title: fc.string({ minLength: 5, maxLength: 20 }),
    artist: fc.string({ minLength: 5, maxLength: 20 }),
    album: fc.string({ minLength: 5, maxLength: 20 }),
    duration: fc.double({ min: 1, max: 600 }),
    filePath: fc.option(fc.string({ minLength: 10, maxLength: 30 })),
    source: fc.constantFrom(
      "local" as const,
      "spotify" as const,
      "youtube" as const,
    ),
    metadata: fc.record({
      year: fc.option(fc.integer({ min: 1900, max: 2024 })),
      genre: fc.option(fc.string({ minLength: 3, maxLength: 15 })),
      trackNumber: fc.option(fc.integer({ min: 1, max: 99 })),
      albumArt: fc.option(fc.string({ minLength: 10, maxLength: 50 })),
    }),
  });
};

// Helper to create arbitrary playlists
const arbPlaylist = (): fc.Arbitrary<Playlist> => {
  return fc.record({
    id: fc.uuid(),
    name: fc.string({ minLength: 3, maxLength: 30 }),
    tracks: fc.array(arbTrack(), { minLength: 0, maxLength: 10 }),
    createdAt: fc.date(),
    modifiedAt: fc.date(),
  });
};

describe("Playlist Queue Synchronization", () => {
  beforeEach(() => {
    playlistStore.reset();
    playerStore.reset();
  });

  // **Feature: milk-player, Property 19: Playlist queue synchronization**
  // **Validates: Requirements 9.4**
  // For any playlist loaded into the player, the playback queue should contain
  // all tracks in the same order as the playlist.
  it("property: playlist queue synchronization", () => {
    fc.assert(
      fc.property(arbPlaylist(), (playlist) => {
        // Load playlist tracks into player queue
        playerStore.setQueue(playlist.tracks);

        // Get the current queue
        const playerState = get(playerStore);

        // Verify queue matches playlist tracks
        expect(playerState.queue.length).toBe(playlist.tracks.length);

        // Verify order is preserved
        for (let i = 0; i < playlist.tracks.length; i++) {
          expect(playerState.queue[i].id).toBe(playlist.tracks[i].id);
          expect(playerState.queue[i].title).toBe(playlist.tracks[i].title);
          expect(playerState.queue[i].artist).toBe(playlist.tracks[i].artist);
        }
      }),
      { numRuns: 100 },
    );
  });

  it("property: empty playlist creates empty queue", () => {
    fc.assert(
      fc.property(arbPlaylist(), (playlist) => {
        // Create empty playlist
        const emptyPlaylist = { ...playlist, tracks: [] };

        // Load into queue
        playerStore.setQueue(emptyPlaylist.tracks);

        // Verify queue is empty
        const playerState = get(playerStore);
        expect(playerState.queue.length).toBe(0);
      }),
      { numRuns: 100 },
    );
  });

  it("property: queue preserves track metadata", () => {
    fc.assert(
      fc.property(arbPlaylist(), (playlist) => {
        // Skip if no tracks
        if (playlist.tracks.length === 0) return true;

        // Load playlist into queue
        playerStore.setQueue(playlist.tracks);

        // Get the current queue
        const playerState = get(playerStore);

        // Verify all metadata is preserved
        for (let i = 0; i < playlist.tracks.length; i++) {
          const original = playlist.tracks[i];
          const queued = playerState.queue[i];

          expect(queued.duration).toBe(original.duration);
          expect(queued.source).toBe(original.source);
          expect(queued.filePath).toBe(original.filePath);
          expect(queued.metadata.year).toBe(original.metadata.year);
          expect(queued.metadata.genre).toBe(original.metadata.genre);
        }

        return true;
      }),
      { numRuns: 100 },
    );
  });
});

describe("Playlist Track Reordering", () => {
  beforeEach(() => {
    playlistStore.reset();
    playerStore.reset();
  });

  // **Feature: milk-player, Property 20: Playlist track reordering**
  // **Validates: Requirements 9.3**
  // For any track reordering operation in a playlist, the playback queue should reflect the new order.
  it("property: reordering tracks updates queue order", () => {
    fc.assert(
      fc.property(arbPlaylist(), (playlist) => {
        // Skip if less than 2 tracks
        if (playlist.tracks.length < 2) return true;

        // Load original order into queue
        playerStore.setQueue(playlist.tracks);

        // Reverse the track order
        const reversedTracks = [...playlist.tracks].reverse();

        // Update the queue with new order
        playerStore.setQueue(reversedTracks);

        // Get the current queue
        const playerState = get(playerStore);

        // Verify queue reflects new order
        expect(playerState.queue.length).toBe(reversedTracks.length);
        for (let i = 0; i < reversedTracks.length; i++) {
          expect(playerState.queue[i].id).toBe(reversedTracks[i].id);
        }

        return true;
      }),
      { numRuns: 100 },
    );
  });

  it("property: reordering preserves all tracks", () => {
    fc.assert(
      fc.property(arbPlaylist(), (playlist) => {
        // Skip if no tracks
        if (playlist.tracks.length === 0) return true;

        // Load original order
        playerStore.setQueue(playlist.tracks);

        // Create a shuffled order (simple shuffle)
        const shuffled = [...playlist.tracks];
        for (let i = shuffled.length - 1; i > 0; i--) {
          const j = Math.floor(Math.random() * (i + 1));
          [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
        }

        // Update queue
        playerStore.setQueue(shuffled);

        // Get the current queue
        const playerState = get(playerStore);

        // Verify all tracks are still present
        expect(playerState.queue.length).toBe(playlist.tracks.length);

        // Verify all original track IDs are present
        const originalIds = new Set(playlist.tracks.map((t) => t.id));
        const queueIds = new Set(playerState.queue.map((t) => t.id));

        for (const id of originalIds) {
          expect(queueIds.has(id)).toBe(true);
        }

        return true;
      }),
      { numRuns: 100 },
    );
  });

  it("property: reordering maintains track integrity", () => {
    fc.assert(
      fc.property(arbPlaylist(), (playlist) => {
        // Skip if no tracks
        if (playlist.tracks.length === 0) return true;

        // Create a reordered version
        const reordered = [...playlist.tracks].sort((a, b) =>
          a.id.localeCompare(b.id),
        );

        // Update queue
        playerStore.setQueue(reordered);

        // Get the current queue
        const playerState = get(playerStore);

        // Verify each track maintains its properties
        for (let i = 0; i < reordered.length; i++) {
          const original = reordered[i];
          const queued = playerState.queue[i];

          expect(queued.id).toBe(original.id);
          expect(queued.title).toBe(original.title);
          expect(queued.artist).toBe(original.artist);
          expect(queued.album).toBe(original.album);
          expect(queued.duration).toBe(original.duration);
        }

        return true;
      }),
      { numRuns: 100 },
    );
  });
});
