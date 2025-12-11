// Synchronization logic between player and farmer states
// This module handles the coordination between playback events and farmer reactions

import { get } from "svelte/store";
import { playerStore } from "./playerStore";
import { farmerStore } from "./farmerStore";
import type { PlayerState } from "../types";

let previousPlayerState: PlayerState | null = null;

/**
 * Synchronize farmer state with player state changes
 * This function should be called whenever player state changes
 */
export function syncFarmerWithPlayer(currentPlayerState: PlayerState) {
  const wasPlaying = previousPlayerState?.isPlaying ?? false;
  const trackChanged =
    previousPlayerState !== null &&
    previousPlayerState.currentTrack?.id !==
      currentPlayerState.currentTrack?.id;

  // React to playback state changes
  if (currentPlayerState.isPlaying && !wasPlaying) {
    // Track started playing - transition to listening state
    farmerStore.transition("listening");
  } else if (!currentPlayerState.isPlaying && wasPlaying) {
    // Track stopped or paused - return to idle state
    farmerStore.transition("idle");
  }

  // React to track changes while playing (only if we have a previous state)
  if (
    trackChanged &&
    currentPlayerState.currentTrack &&
    currentPlayerState.isPlaying
  ) {
    // Brief reaction to track change
    farmerStore.setExpression({ eyes: "neutral", mouth: "smile" });

    // Return to listening state after brief moment
    setTimeout(() => {
      const latestState = get(playerStore);
      if (latestState.isPlaying) {
        farmerStore.transition("listening");
      }
    }, 500);
  }

  // Update previous state for next comparison
  previousPlayerState = { ...currentPlayerState };
}

/**
 * Reset the sync state (useful for testing)
 */
export function resetSyncState() {
  previousPlayerState = null;
}

/**
 * Initialize the synchronization by subscribing to player store
 * Returns an unsubscribe function
 */
export function initializeFarmerPlayerSync() {
  return playerStore.subscribe(syncFarmerWithPlayer);
}
