import { describe, it, expect, beforeEach } from "vitest";
import { get } from "svelte/store";
import { mediaEditorStore } from "./mediaEditorStore";
import type { CropRect, TrimState } from "../types";

describe("MediaEditor Store", () => {
  beforeEach(() => {
    mediaEditorStore.reset();
  });

  describe("loadMedia", () => {
    it("should set file path and media type for image", () => {
      mediaEditorStore.loadMedia("/path/to/image.png", "image");

      const state = get(mediaEditorStore);
      expect(state.filePath).toBe("/path/to/image.png");
      expect(state.mediaType).toBe("image");
      expect(state.isLoading).toBe(false);
      expect(state.error).toBeNull();
    });

    it("should set file path and media type for video", () => {
      mediaEditorStore.loadMedia("/path/to/video.mp4", "video");

      const state = get(mediaEditorStore);
      expect(state.filePath).toBe("/path/to/video.mp4");
      expect(state.mediaType).toBe("video");
      expect(state.isLoading).toBe(false);
      expect(state.error).toBeNull();
    });

    it("should clear error when loading new media", () => {
      // Set an error first
      const initialState = get(mediaEditorStore);
      mediaEditorStore.reset();

      // Load media
      mediaEditorStore.loadMedia("/path/to/image.png", "image");

      const state = get(mediaEditorStore);
      expect(state.error).toBeNull();
    });
  });

  describe("setCrop", () => {
    it("should update crop rectangle", () => {
      const cropRect: CropRect = { x: 10, y: 20, width: 100, height: 200 };

      mediaEditorStore.setCrop(cropRect);

      const state = get(mediaEditorStore);
      expect(state.crop).toEqual(cropRect);
    });

    it("should replace existing crop rectangle", () => {
      const firstCrop: CropRect = { x: 10, y: 20, width: 100, height: 200 };
      const secondCrop: CropRect = { x: 50, y: 60, width: 150, height: 250 };

      mediaEditorStore.setCrop(firstCrop);
      mediaEditorStore.setCrop(secondCrop);

      const state = get(mediaEditorStore);
      expect(state.crop).toEqual(secondCrop);
    });
  });

  describe("setTrim", () => {
    it("should update trim state", () => {
      const trimState: TrimState = { startSec: 5, endSec: 15, durationSec: 30 };

      mediaEditorStore.setTrim(trimState);

      const state = get(mediaEditorStore);
      expect(state.trim).toEqual(trimState);
    });

    it("should replace existing trim state", () => {
      const firstTrim: TrimState = { startSec: 5, endSec: 15, durationSec: 30 };
      const secondTrim: TrimState = {
        startSec: 10,
        endSec: 20,
        durationSec: 30,
      };

      mediaEditorStore.setTrim(firstTrim);
      mediaEditorStore.setTrim(secondTrim);

      const state = get(mediaEditorStore);
      expect(state.trim).toEqual(secondTrim);
    });
  });

  describe("clearCrop", () => {
    it("should remove crop rectangle", () => {
      const cropRect: CropRect = { x: 10, y: 20, width: 100, height: 200 };

      mediaEditorStore.setCrop(cropRect);
      mediaEditorStore.clearCrop();

      const state = get(mediaEditorStore);
      expect(state.crop).toBeNull();
    });

    it("should not affect other state properties", () => {
      mediaEditorStore.loadMedia("/path/to/video.mp4", "video");
      const trimState: TrimState = { startSec: 5, endSec: 15, durationSec: 30 };
      mediaEditorStore.setTrim(trimState);
      const cropRect: CropRect = { x: 10, y: 20, width: 100, height: 200 };
      mediaEditorStore.setCrop(cropRect);

      mediaEditorStore.clearCrop();

      const state = get(mediaEditorStore);
      expect(state.crop).toBeNull();
      expect(state.filePath).toBe("/path/to/video.mp4");
      expect(state.mediaType).toBe("video");
      expect(state.trim).toEqual(trimState);
    });
  });

  describe("reset", () => {
    it("should clear all state", () => {
      // Set up some state
      mediaEditorStore.loadMedia("/path/to/video.mp4", "video");
      const cropRect: CropRect = { x: 10, y: 20, width: 100, height: 200 };
      mediaEditorStore.setCrop(cropRect);
      const trimState: TrimState = { startSec: 5, endSec: 15, durationSec: 30 };
      mediaEditorStore.setTrim(trimState);

      // Reset
      mediaEditorStore.reset();

      const state = get(mediaEditorStore);
      expect(state.filePath).toBeNull();
      expect(state.mediaType).toBeNull();
      expect(state.crop).toBeNull();
      expect(state.trim).toBeNull();
      expect(state.isLoading).toBe(false);
      expect(state.error).toBeNull();
    });

    it("should return to initial state", () => {
      const initialState = get(mediaEditorStore);

      // Modify state
      mediaEditorStore.loadMedia("/path/to/image.png", "image");
      mediaEditorStore.setCrop({ x: 10, y: 20, width: 100, height: 200 });

      // Reset
      mediaEditorStore.reset();

      const resetState = get(mediaEditorStore);
      expect(resetState).toEqual(initialState);
    });
  });

  describe("state updates", () => {
    it("should handle complete workflow", () => {
      // Load media
      mediaEditorStore.loadMedia("/path/to/video.mp4", "video");
      let state = get(mediaEditorStore);
      expect(state.filePath).toBe("/path/to/video.mp4");
      expect(state.mediaType).toBe("video");

      // Set crop
      const cropRect: CropRect = { x: 10, y: 20, width: 100, height: 200 };
      mediaEditorStore.setCrop(cropRect);
      state = get(mediaEditorStore);
      expect(state.crop).toEqual(cropRect);

      // Set trim
      const trimState: TrimState = { startSec: 5, endSec: 15, durationSec: 30 };
      mediaEditorStore.setTrim(trimState);
      state = get(mediaEditorStore);
      expect(state.trim).toEqual(trimState);

      // Clear crop
      mediaEditorStore.clearCrop();
      state = get(mediaEditorStore);
      expect(state.crop).toBeNull();
      expect(state.trim).toEqual(trimState);

      // Reset all
      mediaEditorStore.reset();
      state = get(mediaEditorStore);
      expect(state.filePath).toBeNull();
      expect(state.mediaType).toBeNull();
      expect(state.crop).toBeNull();
      expect(state.trim).toBeNull();
    });
  });
});
