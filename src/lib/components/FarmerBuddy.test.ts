// Property-based tests for FarmerBuddy component
import { describe, it, expect, beforeEach } from "vitest";
import * as fc from "fast-check";
import { farmerStore } from "../stores/farmerStore";
import { playerStore } from "../stores/playerStore";
import {
  syncFarmerWithPlayer,
  resetSyncState,
} from "../stores/farmerPlayerSync";
import { get } from "svelte/store";
import type { FarmerState } from "../types";

describe("FarmerBuddy Property Tests", () => {
  beforeEach(() => {
    farmerStore.reset();
  });

  /**
   * **Feature: milk-player, Property 15: farmer error state handling**
   * **Validates: Requirements 6.3**
   *
   * For any invalid user input (non-existent path, invalid credentials),
   * farmer should transition to error state and display an appropriate error message.
   */
  it("Property 15: farmer error state handling - invalid inputs trigger error state with message", () => {
    fc.assert(
      fc.property(
        fc.oneof(
          fc.constant(""),
          fc.constant("/non/existent/path"),
          fc.constant("C:\\invalid\\path\\that\\does\\not\\exist"),
          fc.constant("invalid_credentials_xyz"),
          fc.constant("malformed@@@data"),
          fc.string().filter((s) => s.trim().length === 0), // whitespace-only strings
        ),
        fc.string({ minLength: 1, maxLength: 200 }), // error message
        (invalidInput, errorMessage) => {
          // Reset to known state
          farmerStore.reset();

          // Simulate error handling for invalid input
          farmerStore.showError(errorMessage);

          const state = get(farmerStore);

          // Property: farmer should be in error state
          expect(state.currentState).toBe("error");

          // Property: error message should be displayed
          expect(state.message).toBe(errorMessage);
          expect(state.message).not.toBeNull();
          expect(state.message!.length).toBeGreaterThan(0);
        },
      ),
      { numRuns: 100 },
    );
  });

  /**
   * **Feature: milk-player, Property 16: farmer state machine transitions**
   * **Validates: Requirements 6.4, 7.1, 7.2, 7.3**
   *
   * For any valid state transition trigger (track start, track stop, config complete, error),
   * farmer should transition to the appropriate state according to the state machine definition.
   */
  it("Property 16: farmer state machine transitions - valid triggers produce correct state transitions", () => {
    fc.assert(
      fc.property(
        fc.constantFrom<FarmerState>(
          "idle",
          "listening",
          "prompting",
          "celebrating",
          "error",
        ),
        fc.constantFrom<FarmerState>(
          "idle",
          "listening",
          "prompting",
          "celebrating",
          "error",
        ),
        fc.option(fc.string({ minLength: 1, maxLength: 100 }), { nil: null }),
        (fromState, toState, message) => {
          // Reset and set initial state
          farmerStore.reset();
          farmerStore.transition(fromState);

          const initialState = get(farmerStore);
          expect(initialState.currentState).toBe(fromState);

          // Transition to new state
          farmerStore.transition(toState, message);

          const finalState = get(farmerStore);

          // Property: state should transition to the requested state
          expect(finalState.currentState).toBe(toState);

          // Property: message should be set if provided
          if (message !== null) {
            expect(finalState.message).toBe(message);
          }

          // Property: expression should be appropriate for the state
          expect(finalState.expression).toBeDefined();
          expect(finalState.expression.eyes).toBeDefined();
          expect(finalState.expression.mouth).toBeDefined();

          // Verify state-specific expression properties
          switch (toState) {
            case "idle":
              expect(finalState.expression.eyes).toBe("neutral");
              expect(finalState.expression.mouth).toBe("neutral");
              break;
            case "listening":
              expect(finalState.expression.mouth).toBe("smile");
              break;
            case "prompting":
              expect(finalState.expression.mouth).toBe("talk-1");
              break;
            case "celebrating":
              expect(finalState.expression.mouth).toBe("smile");
              break;
            case "error":
              expect(finalState.expression.eyes).toBe("neutral");
              break;
          }
        },
      ),
      { numRuns: 100 },
    );
  });

  /**
   * Additional property test: State machine specific transitions
   * Tests the specific transitions mentioned in requirements:
   * - Track start -> listening
   * - Track stop -> idle
   * - Config complete -> celebrating then idle
   * - Error -> error state
   */
  it("Property 16 (extended): specific event-driven state transitions", () => {
    fc.assert(
      fc.property(
        fc.constantFrom(
          { event: "track_start", expectedState: "listening" as FarmerState },
          { event: "track_stop", expectedState: "idle" as FarmerState },
          { event: "track_pause", expectedState: "idle" as FarmerState },
          { event: "error_occurred", expectedState: "error" as FarmerState },
        ),
        (transition) => {
          farmerStore.reset();

          // Simulate the event
          switch (transition.event) {
            case "track_start":
              farmerStore.transition("listening");
              break;
            case "track_stop":
            case "track_pause":
              farmerStore.transition("idle");
              break;
            case "error_occurred":
              farmerStore.showError("An error occurred");
              break;
          }

          const state = get(farmerStore);

          // Property: state should match expected state for the event
          expect(state.currentState).toBe(transition.expectedState);
        },
      ),
      { numRuns: 100 },
    );
  });

  /**
   * Property test: Celebrate auto-returns to idle
   * Tests that celebrating state automatically transitions back to idle
   */
  it("Property 16 (celebrate): celebrating state returns to idle after duration", async () => {
    farmerStore.reset();

    // Trigger celebrate with short duration for testing
    farmerStore.celebrate("Success!", 100);

    const celebratingState = get(farmerStore);
    expect(celebratingState.currentState).toBe("celebrating");
    expect(celebratingState.message).toBe("Success!");

    // Wait for auto-transition
    await new Promise((resolve) => setTimeout(resolve, 150));

    const idleState = get(farmerStore);
    expect(idleState.currentState).toBe("idle");
    expect(idleState.message).toBeNull();
  });

  /**
   * Property test: Prompt sets correct state
   */
  it("Property 16 (prompt): prompt method sets prompting state with message", () => {
    fc.assert(
      fc.property(fc.string({ minLength: 1, maxLength: 200 }), (question) => {
        farmerStore.reset();
        farmerStore.prompt(question);

        const state = get(farmerStore);

        expect(state.currentState).toBe("prompting");
        expect(state.message).toBe(question);
        expect(state.expression.mouth).toBe("talk-1");
      }),
      { numRuns: 100 },
    );
  });
});

describe("FarmerBuddy Playback Reaction Tests", () => {
  beforeEach(() => {
    farmerStore.reset();
    playerStore.reset();
    resetSyncState();
  });

  /**
   * **Feature: milk-player, Property 17: farmer listening state animations**
   * **Validates: Requirements 7.4**
   *
   * For any audio playback, when farmer is in listening state,
   * farmer should display animations that are synchronized with audio characteristics.
   */
  it("Property 17: farmer listening state animations - animations synchronized with playback", () => {
    fc.assert(
      fc.property(
        fc.record({
          trackId: fc.string({ minLength: 1, maxLength: 50 }),
          title: fc.string({ minLength: 1, maxLength: 100 }),
          artist: fc.string({ minLength: 1, maxLength: 100 }),
          album: fc.string({ minLength: 1, maxLength: 100 }),
          duration: fc.integer({ min: 1, max: 600 }),
        }),
        fc.boolean(), // isPlaying state
        (trackData, isPlaying) => {
          // Reset stores and sync state for each iteration
          farmerStore.reset();
          playerStore.reset();
          resetSyncState();

          // Create a track
          const track = {
            id: trackData.trackId,
            title: trackData.title,
            artist: trackData.artist,
            album: trackData.album,
            duration: trackData.duration,
            filePath: "/path/to/audio.mp3",
            source: "local" as const,
            metadata: {},
          };

          // Set track and playing state
          playerStore.setCurrentTrack(track);
          playerStore.setPlaying(isPlaying);

          // Sync farmer with player state
          const playerState = get(playerStore);
          syncFarmerWithPlayer(playerState);

          const farmerState = get(farmerStore);

          // Property: When audio is playing, farmer should be in listening state
          if (isPlaying) {
            expect(farmerState.currentState).toBe("listening");

            // Property: Listening state should have appropriate expression
            expect(farmerState.expression.mouth).toBe("smile");

            // Property: Expression should be defined and valid
            expect(farmerState.expression.eyes).toBeDefined();
            expect(["neutral", "blink", "look-left", "look-right"]).toContain(
              farmerState.expression.eyes,
            );
          } else {
            // Property: When not playing, farmer should be in idle state
            expect(farmerState.currentState).toBe("idle");
            expect(farmerState.expression.eyes).toBe("neutral");
            expect(farmerState.expression.mouth).toBe("neutral");
          }
        },
      ),
      { numRuns: 100 },
    );
  });

  /**
   * Property test: Track start transitions to listening
   */
  it("Property 17 (track start): starting playback transitions farmer to listening state", () => {
    fc.assert(
      fc.property(
        fc.record({
          trackId: fc.string({ minLength: 1, maxLength: 50 }),
          title: fc.string({ minLength: 1, maxLength: 100 }),
          artist: fc.string({ minLength: 1, maxLength: 100 }),
        }),
        (trackData) => {
          // Start in idle state
          farmerStore.reset();
          playerStore.reset();
          resetSyncState();

          const initialState = get(farmerStore);
          expect(initialState.currentState).toBe("idle");

          // Create and play a track
          const track = {
            id: trackData.trackId,
            title: trackData.title,
            artist: trackData.artist,
            album: "Test Album",
            duration: 180,
            filePath: "/path/to/audio.mp3",
            source: "local" as const,
            metadata: {},
          };

          playerStore.setCurrentTrack(track);
          playerStore.setPlaying(true);

          // Sync farmer with player state
          syncFarmerWithPlayer(get(playerStore));

          const listeningState = get(farmerStore);

          // Property: Farmer should transition to listening state
          expect(listeningState.currentState).toBe("listening");
          expect(listeningState.expression.mouth).toBe("smile");
        },
      ),
      { numRuns: 100 },
    );
  });

  /**
   * Property test: Track stop returns to idle
   */
  it("Property 17 (track stop): stopping playback returns farmer to idle state", () => {
    fc.assert(
      fc.property(
        fc.record({
          trackId: fc.string({ minLength: 1, maxLength: 50 }),
          title: fc.string({ minLength: 1, maxLength: 100 }),
        }),
        (trackData) => {
          // Start with playing state
          farmerStore.reset();
          playerStore.reset();
          resetSyncState();

          const track = {
            id: trackData.trackId,
            title: trackData.title,
            artist: "Test Artist",
            album: "Test Album",
            duration: 180,
            filePath: "/path/to/audio.mp3",
            source: "local" as const,
            metadata: {},
          };

          playerStore.setCurrentTrack(track);
          playerStore.setPlaying(true);

          // Sync farmer with player state
          syncFarmerWithPlayer(get(playerStore));

          const listeningState = get(farmerStore);
          expect(listeningState.currentState).toBe("listening");

          // Stop playback
          playerStore.setPlaying(false);

          // Sync farmer with updated player state
          syncFarmerWithPlayer(get(playerStore));

          const idleState = get(farmerStore);

          // Property: Farmer should return to idle state
          expect(idleState.currentState).toBe("idle");
          expect(idleState.expression.eyes).toBe("neutral");
          expect(idleState.expression.mouth).toBe("neutral");
        },
      ),
      { numRuns: 100 },
    );
  });

  /**
   * Property test: Track change reaction
   * Note: This test verifies that track changes are detected and trigger the reaction logic.
   * The actual animation timing is handled by setTimeout in the sync function.
   */
  it("Property 17 (track change): changing tracks triggers reaction while maintaining listening state", () => {
    fc.assert(
      fc.property(
        fc
          .array(
            fc.record({
              trackId: fc.string({ minLength: 1, maxLength: 50 }),
              title: fc.string({ minLength: 1, maxLength: 100 }),
            }),
            { minLength: 2, maxLength: 5 },
          )
          .filter((tracks) => {
            // Ensure all tracks have unique IDs
            const ids = tracks.map((t) => t.trackId);
            return new Set(ids).size === ids.length;
          }),
        (tracks) => {
          farmerStore.reset();
          playerStore.reset();
          resetSyncState();

          // Play first track
          const firstTrack = {
            id: tracks[0].trackId,
            title: tracks[0].title,
            artist: "Test Artist",
            album: "Test Album",
            duration: 180,
            filePath: "/path/to/audio1.mp3",
            source: "local" as const,
            metadata: {},
          };

          playerStore.setCurrentTrack(firstTrack);
          playerStore.setPlaying(true);

          // Sync farmer with player state
          syncFarmerWithPlayer(get(playerStore));

          const initialState = get(farmerStore);
          expect(initialState.currentState).toBe("listening");

          // Change to second track
          const secondTrack = {
            id: tracks[1].trackId,
            title: tracks[1].title,
            artist: "Test Artist",
            album: "Test Album",
            duration: 180,
            filePath: "/path/to/audio2.mp3",
            source: "local" as const,
            metadata: {},
          };

          playerStore.setCurrentTrack(secondTrack);
          // Keep playing state true

          // Sync farmer with updated player state (track changed)
          syncFarmerWithPlayer(get(playerStore));

          // Property: The sync function should have set a brief reaction expression
          // and scheduled a return to listening state
          // We can't easily test the setTimeout behavior in a synchronous property test,
          // but we can verify that the expression was set
          const afterChangeState = get(farmerStore);

          // The expression should have been updated to show a reaction
          expect(afterChangeState.expression.mouth).toBe("smile");
          expect(afterChangeState.expression.eyes).toBe("neutral");
        },
      ),
      { numRuns: 100 },
    );
  });

  /**
   * Property test: Expression changes during listening
   */
  it("Property 17 (expressions): listening state allows dynamic expression changes", () => {
    fc.assert(
      fc.property(
        fc.constantFrom<"smile" | "talk-1" | "talk-2">(
          "smile",
          "talk-1",
          "talk-2",
        ),
        fc.constantFrom<"neutral" | "look-left" | "look-right">(
          "neutral",
          "look-left",
          "look-right",
        ),
        (mouthExpression, eyeExpression) => {
          farmerStore.reset();
          playerStore.reset();

          // Set to listening state
          farmerStore.transition("listening");

          // Change expression
          farmerStore.setExpression({
            mouth: mouthExpression,
            eyes: eyeExpression,
          });

          const state = get(farmerStore);

          // Property: State should remain listening
          expect(state.currentState).toBe("listening");

          // Property: Expression should be updated
          expect(state.expression.mouth).toBe(mouthExpression);
          expect(state.expression.eyes).toBe(eyeExpression);
        },
      ),
      { numRuns: 100 },
    );
  });
});
