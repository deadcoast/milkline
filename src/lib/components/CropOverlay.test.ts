import { describe, it, expect } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import * as fc from "fast-check";
import CropOverlay from "./CropOverlay.svelte";
import type { CropRect } from "$lib/types";

describe("CropOverlay Component - Unit Tests", () => {
  const defaultProps = {
    sourceWidth: 1920,
    sourceHeight: 1080,
    previewWidth: 640,
    previewHeight: 360,
  };

  /**
   * Test mouse drag creates rectangle
   * Requirements: 1.2, 1.3
   */
  it("should create a crop rectangle when user drags mouse", async () => {
    let emittedCrop: CropRect | null = null;
    const { container } = render(CropOverlay, {
      props: {
        ...defaultProps,
        oncropchange: (crop: CropRect) => {
          emittedCrop = crop;
        },
      },
    });
    const overlay = container.querySelector(".crop-overlay") as HTMLElement;
    const rect = overlay.getBoundingClientRect();

    // Simulate mouse drag from (50, 50) to (200, 150)
    await fireEvent.mouseDown(overlay, {
      clientX: rect.left + 50,
      clientY: rect.top + 50,
    });
    await fireEvent.mouseMove(overlay, {
      clientX: rect.left + 200,
      clientY: rect.top + 150,
    });
    await fireEvent.mouseUp(overlay, {
      clientX: rect.left + 200,
      clientY: rect.top + 150,
    });

    // Verify SVG rectangle is rendered
    const svg = container.querySelector(".crop-svg");
    expect(svg).toBeTruthy();

    // Verify cropChange event was emitted with source coordinates
    expect(emittedCrop).toBeTruthy();
    expect(emittedCrop).toHaveProperty("x");
    expect(emittedCrop).toHaveProperty("y");
    expect(emittedCrop).toHaveProperty("width");
    expect(emittedCrop).toHaveProperty("height");
  });

  /**
   * Test rectangle persists after mouse release
   * Requirements: 1.2, 1.3
   */
  it("should persist crop rectangle after mouse release", async () => {
    const { container } = render(CropOverlay, { props: defaultProps });
    const overlay = container.querySelector(".crop-overlay") as HTMLElement;
    const rect = overlay.getBoundingClientRect();

    // Simulate mouse drag
    await fireEvent.mouseDown(overlay, {
      clientX: rect.left + 100,
      clientY: rect.top + 100,
    });
    await fireEvent.mouseMove(overlay, {
      clientX: rect.left + 300,
      clientY: rect.top + 250,
    });
    await fireEvent.mouseUp(overlay, {
      clientX: rect.left + 300,
      clientY: rect.top + 250,
    });

    // Verify SVG is still present after mouse release
    let svg = container.querySelector(".crop-svg");
    expect(svg).toBeTruthy();

    // Verify rectangle elements exist
    const rects = container.querySelectorAll("rect");
    expect(rects.length).toBeGreaterThan(0);

    // Verify corner handles exist
    const circles = container.querySelectorAll("circle");
    expect(circles.length).toBe(4); // 4 corner handles
  });

  /**
   * Test drawing in reverse direction (bottom-right to top-left)
   */
  it("should normalize rectangle when drawn in reverse direction", async () => {
    let emittedCrop: CropRect | null = null;
    const { container } = render(CropOverlay, {
      props: {
        ...defaultProps,
        oncropchange: (crop: CropRect) => {
          emittedCrop = crop;
        },
      },
    });
    const overlay = container.querySelector(".crop-overlay") as HTMLElement;
    const rect = overlay.getBoundingClientRect();

    // Draw from bottom-right to top-left
    await fireEvent.mouseDown(overlay, {
      clientX: rect.left + 200,
      clientY: rect.top + 150,
    });
    await fireEvent.mouseMove(overlay, {
      clientX: rect.left + 50,
      clientY: rect.top + 50,
    });
    await fireEvent.mouseUp(overlay, {
      clientX: rect.left + 50,
      clientY: rect.top + 50,
    });

    // Verify crop was emitted
    expect(emittedCrop).toBeTruthy();

    // Verify coordinates are normalized (positive width/height)
    expect(emittedCrop!.width).toBeGreaterThan(0);
    expect(emittedCrop!.height).toBeGreaterThan(0);
  });

  /**
   * Test clamping to preview bounds
   */
  it("should clamp coordinates to preview bounds", async () => {
    let emittedCrop: CropRect | null = null;
    const { container } = render(CropOverlay, {
      props: {
        ...defaultProps,
        oncropchange: (crop: CropRect) => {
          emittedCrop = crop;
        },
      },
    });
    const overlay = container.querySelector(".crop-overlay") as HTMLElement;
    const rect = overlay.getBoundingClientRect();

    // Try to drag beyond preview bounds
    await fireEvent.mouseDown(overlay, {
      clientX: rect.left + 50,
      clientY: rect.top + 50,
    });
    await fireEvent.mouseMove(overlay, {
      clientX: rect.left + 10000,
      clientY: rect.top + 10000,
    }); // Way beyond bounds
    await fireEvent.mouseUp(overlay, {
      clientX: rect.left + 10000,
      clientY: rect.top + 10000,
    });

    // Verify crop was emitted
    expect(emittedCrop).toBeTruthy();

    // Verify coordinates are within source bounds
    expect(emittedCrop!.x).toBeGreaterThanOrEqual(0);
    expect(emittedCrop!.y).toBeGreaterThanOrEqual(0);
    expect(emittedCrop!.x + emittedCrop!.width).toBeLessThanOrEqual(
      defaultProps.sourceWidth,
    );
    expect(emittedCrop!.y + emittedCrop!.height).toBeLessThanOrEqual(
      defaultProps.sourceHeight,
    );
  });

  /**
   * Test starting new crop clears previous one
   */
  it("should clear previous crop when starting new drag", async () => {
    const { container } = render(CropOverlay, { props: defaultProps });
    const overlay = container.querySelector(".crop-overlay") as HTMLElement;
    const rect = overlay.getBoundingClientRect();

    // Create first crop
    await fireEvent.mouseDown(overlay, {
      clientX: rect.left + 50,
      clientY: rect.top + 50,
    });
    await fireEvent.mouseMove(overlay, {
      clientX: rect.left + 150,
      clientY: rect.top + 150,
    });
    await fireEvent.mouseUp(overlay, {
      clientX: rect.left + 150,
      clientY: rect.top + 150,
    });

    // Verify first crop exists
    let svg = container.querySelector(".crop-svg");
    expect(svg).toBeTruthy();

    // Start second crop (should clear first)
    await fireEvent.mouseDown(overlay, {
      clientX: rect.left + 200,
      clientY: rect.top + 200,
    });

    // Complete second crop
    await fireEvent.mouseMove(overlay, {
      clientX: rect.left + 400,
      clientY: rect.top + 300,
    });
    await fireEvent.mouseUp(overlay, {
      clientX: rect.left + 400,
      clientY: rect.top + 300,
    });

    // Verify only 4 corner handles (one crop)
    const circles = container.querySelectorAll("circle");
    expect(circles.length).toBe(4);
  });

  /**
   * Test mouse leave acts like mouse up
   */
  it("should finalize crop when mouse leaves overlay", async () => {
    let emittedCrop: CropRect | null = null;
    const { container } = render(CropOverlay, {
      props: {
        ...defaultProps,
        oncropchange: (crop: CropRect) => {
          emittedCrop = crop;
        },
      },
    });
    const overlay = container.querySelector(".crop-overlay") as HTMLElement;
    const rect = overlay.getBoundingClientRect();

    // Start drag
    await fireEvent.mouseDown(overlay, {
      clientX: rect.left + 50,
      clientY: rect.top + 50,
    });
    await fireEvent.mouseMove(overlay, {
      clientX: rect.left + 200,
      clientY: rect.top + 150,
    });

    // Mouse leaves instead of mouse up
    await fireEvent.mouseLeave(overlay);

    // Verify crop was emitted
    expect(emittedCrop).toBeTruthy();
  });

  /**
   * Test zero-size rectangles are not emitted
   */
  it("should not emit cropChange for zero-size rectangles", async () => {
    let emittedCrop: CropRect | null = null;
    const { container } = render(CropOverlay, {
      props: {
        ...defaultProps,
        oncropchange: (crop: CropRect) => {
          emittedCrop = crop;
        },
      },
    });
    const overlay = container.querySelector(".crop-overlay") as HTMLElement;
    const rect = overlay.getBoundingClientRect();

    // Click without dragging (zero-size rectangle)
    await fireEvent.mouseDown(overlay, {
      clientX: rect.left + 100,
      clientY: rect.top + 100,
    });
    await fireEvent.mouseUp(overlay, {
      clientX: rect.left + 100,
      clientY: rect.top + 100,
    });

    // Verify no crop was emitted
    expect(emittedCrop).toBeNull();
  });
});

describe("CropOverlay Component - Property-Based Tests", () => {
  const defaultProps = {
    sourceWidth: 1920,
    sourceHeight: 1080,
    previewWidth: 640,
    previewHeight: 360,
  };

  /**
   * **Feature: media-editor, Property 5: Crop rectangle normalization is consistent**
   * **Validates: Requirements 1.2, 2.2, 6.2**
   *
   * For any two crop rectangles with the same start and end points but drawn in opposite
   * directions (top-left to bottom-right vs bottom-right to top-left), the normalized
   * rectangles should be identical.
   */
  it("should normalize crop rectangles consistently regardless of draw direction", async () => {
    await fc.assert(
      fc.asyncProperty(
        fc.record({
          x1: fc.integer({ min: 0, max: 640 }),
          y1: fc.integer({ min: 0, max: 360 }),
          x2: fc.integer({ min: 0, max: 640 }),
          y2: fc.integer({ min: 0, max: 360 }),
        }),
        async ({ x1, y1, x2, y2 }) => {
          // Skip if points are the same (zero-size rectangle)
          if (x1 === x2 || y1 === y2) {
            return true;
          }

          // Test drawing from top-left to bottom-right
          let crop1: CropRect | null = null;
          const { container: container1 } = render(CropOverlay, {
            props: {
              ...defaultProps,
              oncropchange: (crop: CropRect) => {
                crop1 = crop;
              },
            },
          });
          const overlay1 = container1.querySelector(
            ".crop-overlay",
          ) as HTMLElement;
          const rect1 = overlay1.getBoundingClientRect();

          await fireEvent.mouseDown(overlay1, {
            clientX: rect1.left + x1,
            clientY: rect1.top + y1,
          });
          await fireEvent.mouseMove(overlay1, {
            clientX: rect1.left + x2,
            clientY: rect1.top + y2,
          });
          await fireEvent.mouseUp(overlay1, {
            clientX: rect1.left + x2,
            clientY: rect1.top + y2,
          });

          // Test drawing from bottom-right to top-left (opposite direction)
          let crop2: CropRect | null = null;
          const { container: container2 } = render(CropOverlay, {
            props: {
              ...defaultProps,
              oncropchange: (crop: CropRect) => {
                crop2 = crop;
              },
            },
          });
          const overlay2 = container2.querySelector(
            ".crop-overlay",
          ) as HTMLElement;
          const rect2 = overlay2.getBoundingClientRect();

          await fireEvent.mouseDown(overlay2, {
            clientX: rect2.left + x2,
            clientY: rect2.top + y2,
          });
          await fireEvent.mouseMove(overlay2, {
            clientX: rect2.left + x1,
            clientY: rect2.top + y1,
          });
          await fireEvent.mouseUp(overlay2, {
            clientX: rect2.left + x1,
            clientY: rect2.top + y1,
          });

          // Both crops should be emitted
          expect(crop1).toBeTruthy();
          expect(crop2).toBeTruthy();

          // Normalized rectangles should be identical
          expect(crop1!.x).toBe(crop2!.x);
          expect(crop1!.y).toBe(crop2!.y);
          expect(crop1!.width).toBe(crop2!.width);
          expect(crop1!.height).toBe(crop2!.height);

          // Both should have positive dimensions
          expect(crop1!.width).toBeGreaterThan(0);
          expect(crop1!.height).toBeGreaterThan(0);
          expect(crop2!.width).toBeGreaterThan(0);
          expect(crop2!.height).toBeGreaterThan(0);

          return true;
        },
      ),
      { numRuns: 100 },
    );
  });
});
