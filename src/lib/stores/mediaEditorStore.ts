// Media Editor state management store
import { writable } from "svelte/store";
import type { MediaEditorState, CropRect, TrimState } from "../types";

const initialMediaEditorState: MediaEditorState = {
  filePath: null,
  mediaType: null,
  crop: null,
  trim: null,
  isLoading: false,
  error: null,
};

function createMediaEditorStore() {
  const { subscribe, set, update } = writable<MediaEditorState>(
    initialMediaEditorState,
  );

  return {
    subscribe,
    loadMedia: (filePath: string, mediaType: "image" | "video") =>
      update((state) => ({
        ...state,
        filePath,
        mediaType,
        isLoading: false,
        error: null,
      })),
    setCrop: (crop: CropRect) => update((state) => ({ ...state, crop })),
    setTrim: (trim: TrimState) => update((state) => ({ ...state, trim })),
    clearCrop: () => update((state) => ({ ...state, crop: null })),
    reset: () => set(initialMediaEditorState),
  };
}

export const mediaEditorStore = createMediaEditorStore();
