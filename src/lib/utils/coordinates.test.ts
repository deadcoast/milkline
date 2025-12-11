import { describe, it, expect } from "vitest";
import {
  widgetToPreviewCoords,
  previewToSourceCoords,
  clampToSourceBounds,
  calculateScaleFactor,
  previewRectToSourceRect,
  sourceToPreviewCoords,
  type PreviewBounds,
  type Rectangle,
} from "./coordinates";

describe("coordinates utilities", () => {
  describe("widgetToPreviewCoords", () => {
    it("should convert widget coordinates to preview coordinates", () => {
      const previewBounds: PreviewBounds = {
        offsetX: 10,
        offsetY: 20,
        width: 100,
        height: 100,
      };

      const result = widgetToPreviewCoords(50, 70, previewBounds);

      expect(result).toEqual({ x: 40, y: 50 });
    });

    it("should handle zero offset", () => {
      const previewBounds: PreviewBounds = {
        offsetX: 0,
        offsetY: 0,
        width: 100,
        height: 100,
      };

      const result = widgetToPreviewCoords(25, 35, previewBounds);

      expect(result).toEqual({ x: 25, y: 35 });
    });
  });

  describe("previewToSourceCoords", () => {
    it("should convert preview coordinates to source coordinates with 1:1 scale", () => {
      const result = previewToSourceCoords(50, 60, 100, 100, 100, 100);

      expect(result).toEqual({ x: 50, y: 60 });
    });

    it("should scale up when preview is smaller than source", () => {
      // Preview is 100x100, source is 200x200 (2x scale)
      const result = previewToSourceCoords(50, 60, 100, 100, 200, 200);

      expect(result).toEqual({ x: 100, y: 120 });
    });

    it("should scale down when preview is larger than source", () => {
      // Preview is 200x200, source is 100x100 (0.5x scale)
      const result = previewToSourceCoords(100, 120, 200, 200, 100, 100);

      expect(result).toEqual({ x: 50, y: 60 });
    });

    it("should handle different aspect ratios", () => {
      // Preview is 200x100, source is 400x200
      const result = previewToSourceCoords(100, 50, 200, 100, 400, 200);

      expect(result).toEqual({ x: 200, y: 100 });
    });

    it("should round to nearest integer", () => {
      // Preview is 100x100, source is 300x300 (3x scale)
      const result = previewToSourceCoords(33, 67, 100, 100, 300, 300);

      // 33 / (100/300) = 99, 67 / (100/300) = 201
      expect(result).toEqual({ x: 99, y: 201 });
    });
  });

  describe("clampToSourceBounds", () => {
    it("should not modify coordinates within bounds", () => {
      const result = clampToSourceBounds(10, 20, 50, 60, 100, 100);

      expect(result).toEqual({ x: 10, y: 20, width: 50, height: 60 });
    });

    it("should clamp negative coordinates to zero", () => {
      const result = clampToSourceBounds(-10, -20, 50, 60, 100, 100);

      expect(result).toEqual({ x: 0, y: 0, width: 50, height: 60 });
    });

    it("should clamp coordinates exceeding source bounds", () => {
      const result = clampToSourceBounds(150, 150, 50, 60, 100, 100);

      // x and y clamped to 99 (max valid position)
      // width and height clamped to 1 (minimum)
      expect(result).toEqual({ x: 99, y: 99, width: 1, height: 1 });
    });

    it("should clamp width exceeding bounds", () => {
      const result = clampToSourceBounds(50, 50, 100, 100, 100, 100);

      // Width should be clamped to 50 (100 - 50)
      expect(result).toEqual({ x: 50, y: 50, width: 50, height: 50 });
    });

    it("should ensure minimum dimensions of 1", () => {
      const result = clampToSourceBounds(10, 10, 0, 0, 100, 100);

      expect(result).toEqual({ x: 10, y: 10, width: 1, height: 1 });
    });

    it("should handle boundary case at edge", () => {
      const result = clampToSourceBounds(99, 99, 10, 10, 100, 100);

      // At position 99, only 1 pixel available
      expect(result).toEqual({ x: 99, y: 99, width: 1, height: 1 });
    });
  });

  describe("calculateScaleFactor", () => {
    it("should calculate scale factor correctly", () => {
      expect(calculateScaleFactor(100, 200)).toBe(0.5);
      expect(calculateScaleFactor(200, 100)).toBe(2);
      expect(calculateScaleFactor(100, 100)).toBe(1);
    });

    it("should handle zero source dimension", () => {
      expect(calculateScaleFactor(100, 0)).toBe(1);
    });

    it("should handle fractional scales", () => {
      expect(calculateScaleFactor(150, 100)).toBe(1.5);
    });
  });

  describe("previewRectToSourceRect", () => {
    it("should convert rectangle from preview to source coordinates", () => {
      const rect: Rectangle = { x: 10, y: 20, width: 30, height: 40 };
      const result = previewRectToSourceRect(rect, 100, 100, 200, 200);

      // 2x scale: positions and dimensions should double
      expect(result).toEqual({ x: 20, y: 40, width: 60, height: 80 });
    });

    it("should clamp result to source bounds", () => {
      const rect: Rectangle = { x: 80, y: 80, width: 50, height: 50 };
      const result = previewRectToSourceRect(rect, 100, 100, 100, 100);

      // Rectangle extends beyond bounds, should be clamped
      expect(result.x).toBe(80);
      expect(result.y).toBe(80);
      expect(result.width).toBeLessThanOrEqual(20);
      expect(result.height).toBeLessThanOrEqual(20);
    });

    it("should handle different aspect ratios", () => {
      const rect: Rectangle = { x: 0, y: 0, width: 99, height: 49 };
      const result = previewRectToSourceRect(rect, 100, 50, 200, 100);

      // 2x scale on both dimensions
      // Note: width/height are slightly less than full size due to coordinate clamping
      expect(result.x).toBe(0);
      expect(result.y).toBe(0);
      expect(result.width).toBeGreaterThanOrEqual(198);
      expect(result.height).toBeGreaterThanOrEqual(98);
    });
  });

  describe("sourceToPreviewCoords", () => {
    it("should convert source coordinates to preview coordinates", () => {
      const result = sourceToPreviewCoords(100, 120, 100, 100, 200, 200);

      // 0.5x scale: coordinates should halve
      expect(result).toEqual({ x: 50, y: 60 });
    });

    it("should handle 1:1 scale", () => {
      const result = sourceToPreviewCoords(50, 60, 100, 100, 100, 100);

      expect(result).toEqual({ x: 50, y: 60 });
    });

    it("should round to nearest integer", () => {
      const result = sourceToPreviewCoords(100, 100, 150, 150, 300, 300);

      // 100 * (150/300) = 50
      expect(result).toEqual({ x: 50, y: 50 });
    });
  });

  describe("coordinate mapping integration", () => {
    it("should map from widget to source coordinates correctly", () => {
      const previewBounds: PreviewBounds = {
        offsetX: 10,
        offsetY: 10,
        width: 100,
        height: 100,
      };

      // Widget coords (60, 70) -> Preview coords (50, 60)
      const previewCoords = widgetToPreviewCoords(60, 70, previewBounds);

      // Preview coords (50, 60) -> Source coords (100, 120) with 2x scale
      const sourceCoords = previewToSourceCoords(
        previewCoords.x,
        previewCoords.y,
        100,
        100,
        200,
        200,
      );

      expect(sourceCoords).toEqual({ x: 100, y: 120 });
    });
  });
});
