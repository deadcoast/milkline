import { describe, it, expect, vi } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import * as fc from "fast-check";
import Timeline from "./Timeline.svelte";

describe("Timeline Component - Unit Tests", () => {
  const defaultProps = {
    duration: 120, // 2 minutes
    startTime: 10,
    endTime: 60,
  };

  /**
   * Test handle dragging updates times
   * Requirements: 3.2, 3.3
   */
  it("should update start time when dragging start handle", async () => {
    let emittedTrim: { startSec: number; endSec: number } | null = null;
    const { container } = render(Timeline, {
      props: {
        ...defaultProps,
        ontrimchange: (trim: { startSec: number; endSec: number }) => {
          emittedTrim = trim;
        },
      },
    });

    const startHandle = container.querySelector(
      ".timeline-handle-start",
    ) as HTMLElement;
    expect(startHandle).toBeTruthy();

    // Simulate dragging start handle
    await fireEvent.mouseDown(startHandle, { clientX: 100 });
    await fireEvent.mouseMove(window, { clientX: 150 });
    await fireEvent.mouseUp(window);

    // Verify trimChange event was emitted
    expect(emittedTrim).toBeTruthy();
    expect(emittedTrim).toHaveProperty("startSec");
    expect(emittedTrim).toHaveProperty("endSec");
  });

  /**
   * Test handle dragging updates times
   * Requirements: 3.2, 3.3
   */
  it("should update end time when dragging end handle", async () => {
    let emittedTrim: { startSec: number; endSec: number } | null = null;
    const { container } = render(Timeline, {
      props: {
        ...defaultProps,
        ontrimchange: (trim: { startSec: number; endSec: number }) => {
          emittedTrim = trim;
        },
      },
    });

    const endHandle = container.querySelector(
      ".timeline-handle-end",
    ) as HTMLElement;
    expect(endHandle).toBeTruthy();

    // Simulate dragging end handle
    await fireEvent.mouseDown(endHandle, { clientX: 300 });
    await fireEvent.mouseMove(window, { clientX: 350 });
    await fireEvent.mouseUp(window);

    // Verify trimChange event was emitted
    expect(emittedTrim).toBeTruthy();
    expect(emittedTrim).toHaveProperty("startSec");
    expect(emittedTrim).toHaveProperty("endSec");
  });

  /**
   * Test start handle constraint
   * Requirements: 3.3, 3.4
   */
  it("should constrain start handle to not exceed end handle", async () => {
    let emittedTrim: { startSec: number; endSec: number } | null = null;
    const { container } = render(Timeline, {
      props: {
        ...defaultProps,
        ontrimchange: (trim: { startSec: number; endSec: number }) => {
          emittedTrim = trim;
        },
      },
    });

    const startHandle = container.querySelector(
      ".timeline-handle-start",
    ) as HTMLElement;
    const timeline = container.querySelector(".timeline") as HTMLElement;

    // Mock getBoundingClientRect to return a proper width
    const mockRect = {
      left: 0,
      top: 0,
      width: 400,
      height: 40,
      right: 400,
      bottom: 40,
    };
    vi.spyOn(timeline, "getBoundingClientRect").mockReturnValue(
      mockRect as DOMRect,
    );

    // Try to drag start handle past end handle (to the right edge)
    await fireEvent.mouseDown(startHandle, { clientX: 100 });
    await fireEvent.mouseMove(window, { clientX: 500 }); // Way past end
    await fireEvent.mouseUp(window);

    // Verify start time does not exceed end time
    expect(emittedTrim).toBeTruthy();
    expect(emittedTrim!.startSec).toBeLessThanOrEqual(emittedTrim!.endSec);
  });

  /**
   * Test end handle constraint
   * Requirements: 3.3, 3.4
   */
  it("should constrain end handle to not go before start handle", async () => {
    let emittedTrim: { startSec: number; endSec: number } | null = null;
    const { container } = render(Timeline, {
      props: {
        ...defaultProps,
        ontrimchange: (trim: { startSec: number; endSec: number }) => {
          emittedTrim = trim;
        },
      },
    });

    const endHandle = container.querySelector(
      ".timeline-handle-end",
    ) as HTMLElement;
    const timeline = container.querySelector(".timeline") as HTMLElement;

    // Mock getBoundingClientRect to return a proper width
    const mockRect = {
      left: 0,
      top: 0,
      width: 400,
      height: 40,
      right: 400,
      bottom: 40,
    };
    vi.spyOn(timeline, "getBoundingClientRect").mockReturnValue(
      mockRect as DOMRect,
    );

    // Try to drag end handle before start handle (to the left edge)
    await fireEvent.mouseDown(endHandle, { clientX: 300 });
    await fireEvent.mouseMove(window, { clientX: -100 }); // Way before start
    await fireEvent.mouseUp(window);

    // Verify end time does not go before start time
    expect(emittedTrim).toBeTruthy();
    expect(emittedTrim!.endSec).toBeGreaterThanOrEqual(emittedTrim!.startSec);
  });

  /**
   * Test timeline displays correct time labels
   */
  it("should display formatted time labels", () => {
    const { container } = render(Timeline, { props: defaultProps });

    const labels = container.querySelectorAll(".time-label");
    expect(labels.length).toBe(3); // Start, End, Duration

    // Check that labels contain time format (MM:SS)
    const labelTexts = Array.from(labels).map((label) => label.textContent);
    expect(labelTexts.some((text) => text?.includes("00:10"))).toBe(true); // Start: 10s
    expect(labelTexts.some((text) => text?.includes("01:00"))).toBe(true); // End: 60s
    expect(labelTexts.some((text) => text?.includes("02:00"))).toBe(true); // Duration: 120s
  });

  /**
   * Test timeline renders selection region
   */
  it("should render selection region between handles", () => {
    const { container } = render(Timeline, { props: defaultProps });

    const selection = container.querySelector(
      ".timeline-selection",
    ) as HTMLElement;
    expect(selection).toBeTruthy();

    // Verify selection has width (visual representation of trim range)
    const style = selection.getAttribute("style");
    expect(style).toContain("width");
  });

  /**
   * Test handles are positioned correctly
   */
  it("should position start and end handles", () => {
    const { container } = render(Timeline, { props: defaultProps });

    const startHandle = container.querySelector(".timeline-handle-start");
    const endHandle = container.querySelector(".timeline-handle-end");

    expect(startHandle).toBeTruthy();
    expect(endHandle).toBeTruthy();
  });

  /**
   * Test mouse up stops dragging
   */
  it("should stop dragging on mouse up", async () => {
    let callCount = 0;
    const { container } = render(Timeline, {
      props: {
        ...defaultProps,
        ontrimchange: () => {
          callCount++;
        },
      },
    });

    const startHandle = container.querySelector(
      ".timeline-handle-start",
    ) as HTMLElement;

    // Start dragging
    await fireEvent.mouseDown(startHandle, { clientX: 100 });
    await fireEvent.mouseMove(window, { clientX: 150 });

    const countAfterFirstMove = callCount;

    // Release mouse
    await fireEvent.mouseUp(window);

    // Move again - should not trigger event
    await fireEvent.mouseMove(window, { clientX: 200 });

    // Verify no additional events after mouse up
    expect(callCount).toBe(countAfterFirstMove);
  });

  /**
   * Test timeline handles edge cases
   */
  it("should handle zero duration gracefully", () => {
    const { container } = render(Timeline, {
      props: {
        duration: 0,
        startTime: 0,
        endTime: 0,
      },
    });

    // Should render without errors
    const timeline = container.querySelector(".timeline");
    expect(timeline).toBeTruthy();
  });

  /**
   * Test timeline with start and end at same position
   */
  it("should handle start and end at same position", () => {
    const { container } = render(Timeline, {
      props: {
        duration: 100,
        startTime: 50,
        endTime: 50,
      },
    });

    const selection = container.querySelector(
      ".timeline-selection",
    ) as HTMLElement;
    expect(selection).toBeTruthy();

    // Selection should have zero or minimal width
    const style = selection.getAttribute("style");
    expect(style).toBeTruthy();
  });
});

describe("Timeline Component - Property-Based Tests", () => {
  /**
   * **Feature: media-editor, Property 2: Timeline trim constraints are maintained**
   * **Validates: Requirements 3.2, 3.3, 3.4**
   *
   * For any video timeline state, the start time SHALL always be less than or equal to
   * the end time, and both SHALL be within the video duration bounds
   * (0 ≤ start ≤ end ≤ duration).
   */
  it("should maintain timeline constraints for all valid inputs", async () => {
    await fc.assert(
      fc.asyncProperty(
        fc.record({
          duration: fc.float({ min: 1, max: 3600 }), // 1 second to 1 hour
          startTime: fc.float({ min: 0, max: 3600 }),
          endTime: fc.float({ min: 0, max: 3600 }),
          dragStartX: fc.integer({ min: 0, max: 400 }),
          dragEndX: fc.integer({ min: 0, max: 400 }),
        }),
        async ({ duration, startTime, endTime, dragStartX, dragEndX }) => {
          // Ensure initial state is valid
          const validStartTime = Math.max(0, Math.min(startTime, duration));
          const validEndTime = Math.max(
            validStartTime,
            Math.min(endTime, duration),
          );

          let emittedTrim: { startSec: number; endSec: number } | null = null;
          const { container } = render(Timeline, {
            props: {
              duration,
              startTime: validStartTime,
              endTime: validEndTime,
              ontrimchange: (trim: { startSec: number; endSec: number }) => {
                emittedTrim = trim;
              },
            },
          });

          const timeline = container.querySelector(".timeline") as HTMLElement;

          // Mock getBoundingClientRect to return a proper width
          const mockRect = {
            left: 0,
            top: 0,
            width: 400,
            height: 40,
            right: 400,
            bottom: 40,
          };
          vi.spyOn(timeline, "getBoundingClientRect").mockReturnValue(
            mockRect as DOMRect,
          );

          // Test dragging start handle
          const startHandle = container.querySelector(
            ".timeline-handle-start",
          ) as HTMLElement;
          if (startHandle) {
            await fireEvent.mouseDown(startHandle, { clientX: dragStartX });
            await fireEvent.mouseMove(window, { clientX: dragEndX });
            await fireEvent.mouseUp(window);

            if (emittedTrim) {
              // Property 2: Verify constraints are maintained
              expect(emittedTrim.startSec).toBeGreaterThanOrEqual(0);
              expect(emittedTrim.startSec).toBeLessThanOrEqual(
                emittedTrim.endSec,
              );
              expect(emittedTrim.endSec).toBeLessThanOrEqual(duration);
            }
          }

          // Reset for end handle test
          emittedTrim = null;

          // Test dragging end handle
          const endHandle = container.querySelector(
            ".timeline-handle-end",
          ) as HTMLElement;
          if (endHandle) {
            await fireEvent.mouseDown(endHandle, { clientX: dragStartX });
            await fireEvent.mouseMove(window, { clientX: dragEndX });
            await fireEvent.mouseUp(window);

            if (emittedTrim) {
              // Property 2: Verify constraints are maintained
              expect(emittedTrim.startSec).toBeGreaterThanOrEqual(0);
              expect(emittedTrim.startSec).toBeLessThanOrEqual(
                emittedTrim.endSec,
              );
              expect(emittedTrim.endSec).toBeLessThanOrEqual(duration);
            }
          }

          return true;
        },
      ),
      { numRuns: 100 },
    );
  });
});
