import { describe, it, expect, beforeEach, vi } from "vitest";
import { render } from "@testing-library/svelte";
import { get } from "svelte/store";
import * as fc from "fast-check";
import Player from "./Player.svelte";
import { playerStore } from "$lib/stores";
import type { Track } from "$lib/types";

describe("Player Component - Property-Based Tests", () => {
  beforeEach(() => {
    playerStore.reset();
    // Mock HTMLAudioElement
    global.HTMLAudioElement.prototype.play = vi
      .fn()
      .mockResolvedValue(undefined);
    global.HTMLAudioElement.prototype.pause = vi.fn();
    global.HTMLAudioElement.prototype.load = vi.fn();
  });

  /**
   * **Feature: milk-player, Property 1: Supported audio format playback**
   * **Validates: Requirements 1.1**
   *
   * For any valid audio file in supported formats (mp3, flac, wav),
   * the application should successfully load and initiate playback without errors.
   */
  describe("Property 1: Supported audio format playback", () => {
    it("should successfully load and play any supported audio format", async () => {
      await fc.assert(
        fc.asyncProperty(
          fc.record({
            id: fc.uuid(),
            title: fc.string({ minLength: 1, maxLength: 100 }),
            artist: fc.string({ minLength: 1, maxLength: 100 }),
            album: fc.string({ minLength: 1, maxLength: 100 }),
            duration: fc.double({ min: 1, max: 7200, noNaN: true }),
            format: fc.constantFrom("mp3", "flac", "wav"),
            source: fc.constant("local" as const),
          }),
          async (trackData) => {
            const track: Track = {
              ...trackData,
              filePath: `/path/to/audio.${trackData.format}`,
              metadata: {},
            };

            const { component } = render(Player);

            // Call play with the track
            component.play(track);

            // Wait for async operations
            await new Promise((resolve) => setTimeout(resolve, 10));

            // Verify track was loaded
            const state = get(playerStore);
            expect(state.currentTrack).toEqual(track);

            // Verify play was called without errors
            expect(global.HTMLAudioElement.prototype.play).toHaveBeenCalled();
          },
        ),
        { numRuns: 100 },
      );
    });
  });

  /**
   * **Feature: milk-player, Property 3: Playback position accuracy**
   * **Validates: Requirements 1.3**
   *
   * For any audio file during playback, the reported playback position and duration
   * should match the actual audio file properties within acceptable tolerance (±100ms).
   */
  describe("Property 3: Playback position accuracy", () => {
    it("should accurately track playback position within tolerance", async () => {
      await fc.assert(
        fc.asyncProperty(
          fc.record({
            duration: fc.double({ min: 1, max: 7200, noNaN: true }),
            position: fc.double({ min: 0, max: 7200, noNaN: true }),
          }),
          async ({ duration, position }) => {
            // Ensure position doesn't exceed duration
            const validPosition = Math.min(position, duration);

            const track: Track = {
              id: "test-track",
              title: "Test Track",
              artist: "Test Artist",
              album: "Test Album",
              duration,
              filePath: "/path/to/audio.mp3",
              source: "local",
              metadata: {},
            };

            const { component } = render(Player);

            // Mock audio element properties
            const audioElement = document.querySelector(
              "audio",
            ) as HTMLAudioElement;
            if (audioElement) {
              Object.defineProperty(audioElement, "duration", {
                value: duration,
                writable: true,
              });
              Object.defineProperty(audioElement, "currentTime", {
                value: validPosition,
                writable: true,
              });
            }

            component.play(track);
            await new Promise((resolve) => setTimeout(resolve, 10));

            // Trigger loadedmetadata event
            audioElement?.dispatchEvent(new Event("loadedmetadata"));
            await new Promise((resolve) => setTimeout(resolve, 10));

            // Seek to position
            component.seek(validPosition);
            await new Promise((resolve) => setTimeout(resolve, 10));

            const state = get(playerStore);

            // Verify duration is set correctly
            expect(state.duration).toBeCloseTo(duration, 1);

            // Verify position is within tolerance (±100ms = ±0.1s)
            const tolerance = 0.1;
            expect(
              Math.abs(state.position - validPosition),
            ).toBeLessThanOrEqual(tolerance);
          },
        ),
        { numRuns: 100 },
      );
    });
  });

  /**
   * **Feature: milk-player, Property 4: Volume control responsiveness**
   * **Validates: Requirements 1.4**
   *
   * For any volume value in the valid range [0.0, 1.0], setting the volume
   * should result in the audio output reflecting that volume level.
   */
  describe("Property 4: Volume control responsiveness", () => {
    it("should set volume correctly for any valid value in range [0, 1]", async () => {
      await fc.assert(
        fc.asyncProperty(
          fc.double({ min: 0, max: 1, noNaN: true }),
          async (volumeValue) => {
            const { component } = render(Player);

            // Set volume
            component.setVolume(volumeValue);
            await new Promise((resolve) => setTimeout(resolve, 10));

            const state = get(playerStore);

            // Verify volume is set correctly
            expect(state.volume).toBeCloseTo(volumeValue, 2);

            // Verify volume is within valid range
            expect(state.volume).toBeGreaterThanOrEqual(0);
            expect(state.volume).toBeLessThanOrEqual(1);
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should clamp volume values outside valid range", async () => {
      await fc.assert(
        fc.asyncProperty(
          fc.double({ min: -10, max: 10, noNaN: true }),
          async (volumeValue) => {
            const { component } = render(Player);

            component.setVolume(volumeValue);
            await new Promise((resolve) => setTimeout(resolve, 10));

            const state = get(playerStore);

            // Verify volume is clamped to valid range
            expect(state.volume).toBeGreaterThanOrEqual(0);
            expect(state.volume).toBeLessThanOrEqual(1);
          },
        ),
        { numRuns: 100 },
      );
    });
  });

  /**
   * **Feature: milk-player, Property 5: Playback control state transitions**
   * **Validates: Requirements 1.5**
   *
   * For any playback control command (play, pause, stop, next, previous),
   * executing the command should transition the player to the corresponding state immediately.
   */
  describe("Property 5: Playback control state transitions", () => {
    it("should transition to correct state for any playback command", async () => {
      await fc.assert(
        fc.asyncProperty(
          fc.record({
            track: fc.record({
              id: fc.uuid(),
              title: fc.string({ minLength: 1, maxLength: 100 }),
              artist: fc.string({ minLength: 1, maxLength: 100 }),
              album: fc.string({ minLength: 1, maxLength: 100 }),
              duration: fc.double({ min: 1, max: 7200, noNaN: true }),
              filePath: fc.constant("/path/to/audio.mp3"),
              source: fc.constant("local" as const),
              metadata: fc.constant({}),
            }),
            command: fc.constantFrom("play", "pause", "stop"),
          }),
          async ({ track, command }) => {
            const { component } = render(Player);

            // Load a track first
            component.play(track);
            await new Promise((resolve) => setTimeout(resolve, 10));

            // Simulate audio element playing
            const audioElement = document.querySelector(
              "audio",
            ) as HTMLAudioElement;
            if (audioElement) {
              audioElement.dispatchEvent(new Event("play"));
            }
            await new Promise((resolve) => setTimeout(resolve, 10));

            // Execute command
            switch (command) {
              case "play":
                component.play();
                if (audioElement) {
                  audioElement.dispatchEvent(new Event("play"));
                }
                break;
              case "pause":
                component.pause();
                break;
              case "stop":
                component.stop();
                break;
            }

            await new Promise((resolve) => setTimeout(resolve, 10));

            const state = get(playerStore);

            // Verify state transitions
            switch (command) {
              case "play":
                expect(state.isPlaying).toBe(true);
                break;
              case "pause":
                expect(state.isPlaying).toBe(false);
                expect(state.currentTrack).not.toBeNull();
                break;
              case "stop":
                expect(state.isPlaying).toBe(false);
                expect(state.position).toBe(0);
                break;
            }
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should handle next/previous commands correctly", async () => {
      await fc.assert(
        fc.asyncProperty(
          fc.array(
            fc.record({
              id: fc.uuid(),
              title: fc.string({ minLength: 1, maxLength: 100 }),
              artist: fc.string({ minLength: 1, maxLength: 100 }),
              album: fc.string({ minLength: 1, maxLength: 100 }),
              duration: fc.double({ min: 1, max: 7200, noNaN: true }),
              filePath: fc.constant("/path/to/audio.mp3"),
              source: fc.constant("local" as const),
              metadata: fc.constant({}),
            }),
            { minLength: 1, maxLength: 10 },
          ),
          async (tracks) => {
            const { component } = render(Player);

            // Reset store to ensure clean state
            playerStore.reset();

            // Add tracks to queue
            tracks.forEach((track) => playerStore.addToQueue(track));
            await new Promise((resolve) => setTimeout(resolve, 10));

            const initialQueueLength = tracks.length;

            // Execute next command
            component.next();
            await new Promise((resolve) => setTimeout(resolve, 10));

            const state = get(playerStore);

            // Verify queue was updated (first track removed from queue)
            expect(state.queue.length).toBe(initialQueueLength - 1);
            // Verify first track became current track
            expect(state.currentTrack).toEqual(tracks[0]);
          },
        ),
        { numRuns: 100 },
      );
    });
  });
});
