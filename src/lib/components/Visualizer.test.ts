import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, cleanup } from "@testing-library/svelte";
import Visualizer from "./Visualizer.svelte";
import * as fc from "fast-check";

describe("Visualizer Component", () => {
  beforeEach(() => {
    // Mock Web Audio API
    global.AudioContext = class MockAudioContext {
      state = "running";
      destination = {};

      createAnalyser() {
        return {
          fftSize: 2048,
          smoothingTimeConstant: 0.8,
          frequencyBinCount: 1024,
          connect: vi.fn(),
          disconnect: vi.fn(),
          getByteFrequencyData: vi.fn((array: Uint8Array) => {
            for (let i = 0; i < array.length; i++) {
              array[i] = Math.floor(Math.random() * 255);
            }
          }),
          getByteTimeDomainData: vi.fn((array: Uint8Array) => {
            for (let i = 0; i < array.length; i++) {
              array[i] = 128 + Math.floor(Math.random() * 50 - 25);
            }
          }),
        };
      }

      createMediaElementSource() {
        return {
          connect: vi.fn(),
          disconnect: vi.fn(),
        };
      }

      resume() {
        return Promise.resolve();
      }

      close() {
        return Promise.resolve();
      }
    } as any;

    // Mock Canvas getContext
    HTMLCanvasElement.prototype.getContext = vi.fn((contextType: string) => {
      if (contextType === "2d") {
        return {
          clearRect: vi.fn(),
          fillRect: vi.fn(),
          beginPath: vi.fn(),
          moveTo: vi.fn(),
          lineTo: vi.fn(),
          stroke: vi.fn(),
          fillStyle: "",
          strokeStyle: "",
          lineWidth: 1,
        } as any;
      }
      return null;
    }) as any;

    // Mock requestAnimationFrame
    global.requestAnimationFrame = vi.fn((cb) => {
      setTimeout(cb, 16);
      return 1;
    }) as any;

    global.cancelAnimationFrame = vi.fn();
  });

  afterEach(() => {
    cleanup();
    vi.restoreAllMocks();
  });

  /**
   * **Feature: milk-player, Property 12: Visualizer activation**
   * **Validates: Requirements 5.1, 5.4**
   */
  describe("Property 12: Visualizer activation", () => {
    it("should activate visualizer for any audio element when start is called", () => {
      fc.assert(
        fc.property(
          fc.record({
            src: fc.oneof(
              fc.constant("file:///path/to/local.mp3"),
              fc.constant("https://spotify.com/track"),
              fc.constant("https://youtube.com/watch"),
            ),
            paused: fc.boolean(),
          }),
          (audioConfig) => {
            const audioElement = document.createElement("audio");
            audioElement.src = audioConfig.src;
            Object.defineProperty(audioElement, "paused", {
              value: audioConfig.paused,
              writable: true,
            });

            const { component } = render(Visualizer, {
              props: {
                audioElement,
                style: "bars",
                width: 800,
                height: 200,
              },
            });

            component.start();
            expect(component.isVisualizerActive()).toBe(true);

            const analyzerNode = component.getAnalyzerNode();
            expect(analyzerNode).not.toBeNull();

            component.stop();
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should display real-time data when audio is playing", () => {
      fc.assert(
        fc.property(
          fc.constantFrom("bars", "waveform", "spectrum"),
          (visualizationStyle) => {
            const audioElement = document.createElement("audio");
            audioElement.src = "file:///test.mp3";

            const { component } = render(Visualizer, {
              props: {
                audioElement,
                style: visualizationStyle as "bars" | "waveform" | "spectrum",
                width: 800,
                height: 200,
              },
            });

            component.start();
            expect(component.isVisualizerActive()).toBe(true);

            const analyzerNode = component.getAnalyzerNode();
            expect(analyzerNode).not.toBeNull();
            if (analyzerNode) {
              expect(analyzerNode.fftSize).toBe(2048);
              expect(analyzerNode.smoothingTimeConstant).toBe(0.8);
            }

            component.stop();
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should stop displaying data when stop is called", () => {
      fc.assert(
        fc.property(
          fc.constantFrom("bars", "waveform", "spectrum"),
          (visualizationStyle) => {
            const audioElement = document.createElement("audio");
            audioElement.src = "file:///test.mp3";

            const { component } = render(Visualizer, {
              props: {
                audioElement,
                style: visualizationStyle as "bars" | "waveform" | "spectrum",
                width: 800,
                height: 200,
              },
            });

            component.start();
            expect(component.isVisualizerActive()).toBe(true);

            component.stop();
            expect(component.isVisualizerActive()).toBe(false);
          },
        ),
        { numRuns: 100 },
      );
    });
  });

  /**
   * **Feature: milk-player, Property 13: Visualizer frame rate**
   * **Validates: Requirements 5.3**
   */
  describe("Property 13: Visualizer frame rate", () => {
    it("should use requestAnimationFrame for rendering loop", async () => {
      await fc.assert(
        fc.asyncProperty(
          fc.constantFrom("bars", "waveform", "spectrum"),
          async (visualizationStyle) => {
            const audioElement = document.createElement("audio");
            audioElement.src = "file:///test.mp3";

            // Create a spy that tracks RAF calls but doesn't execute callbacks
            const rafSpy = vi.fn(() => 1);
            global.requestAnimationFrame = rafSpy as any;

            const { component } = render(Visualizer, {
              props: {
                audioElement,
                style: visualizationStyle as "bars" | "waveform" | "spectrum",
                width: 800,
                height: 200,
              },
            });

            await component.start();

            // Wait a tick for async operations
            await new Promise((resolve) => setTimeout(resolve, 10));

            // Verify RAF was called (the render loop uses RAF)
            expect(rafSpy).toHaveBeenCalled();

            await component.stop();
          },
        ),
        { numRuns: 50 },
      );
    });

    it("should cancel animation frame when stopped", async () => {
      await fc.assert(
        fc.asyncProperty(
          fc.constantFrom("bars", "waveform", "spectrum"),
          async (visualizationStyle) => {
            const audioElement = document.createElement("audio");
            audioElement.src = "file:///test.mp3";

            // Create spies
            const rafSpy = vi.fn(() => 1);
            const cancelSpy = vi.fn();
            global.requestAnimationFrame = rafSpy as any;
            global.cancelAnimationFrame = cancelSpy as any;

            const { component } = render(Visualizer, {
              props: {
                audioElement,
                style: visualizationStyle as "bars" | "waveform" | "spectrum",
                width: 800,
                height: 200,
              },
            });

            await component.start();

            // Wait a tick for async operations
            await new Promise((resolve) => setTimeout(resolve, 10));

            await component.stop();

            // Verify cancel was called when stopping
            expect(cancelSpy).toHaveBeenCalled();
          },
        ),
        { numRuns: 50 },
      );
    });
  });

  /**
   * **Feature: milk-player, Property 14: Visualizer style switching**
   * **Validates: Requirements 5.5**
   */
  describe("Property 14: Visualizer style switching", () => {
    it("should switch visualization styles without stopping the visualizer", () => {
      fc.assert(
        fc.property(
          fc.constantFrom("bars", "waveform", "spectrum"),
          fc.constantFrom("bars", "waveform", "spectrum"),
          (initialStyle, newStyle) => {
            const audioElement = document.createElement("audio");
            audioElement.src = "file:///test.mp3";

            const { component } = render(Visualizer, {
              props: {
                audioElement,
                style: initialStyle as "bars" | "waveform" | "spectrum",
                width: 800,
                height: 200,
              },
            });

            component.start();
            expect(component.isVisualizerActive()).toBe(true);

            component.setStyle(newStyle as "bars" | "waveform" | "spectrum");
            expect(component.isVisualizerActive()).toBe(true);

            component.stop();
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should accept all valid visualization styles", () => {
      fc.assert(
        fc.property(
          fc.constantFrom("bars", "waveform", "spectrum"),
          (visualizationStyle) => {
            const audioElement = document.createElement("audio");
            audioElement.src = "file:///test.mp3";

            const { component } = render(Visualizer, {
              props: {
                audioElement,
                style: visualizationStyle as "bars" | "waveform" | "spectrum",
                width: 800,
                height: 200,
              },
            });

            component.start();
            expect(component.isVisualizerActive()).toBe(true);

            const styles: Array<"bars" | "waveform" | "spectrum"> = [
              "bars",
              "waveform",
              "spectrum",
            ];
            for (const style of styles) {
              component.setStyle(style);
              expect(component.isVisualizerActive()).toBe(true);
            }

            component.stop();
          },
        ),
        { numRuns: 100 },
      );
    });
  });
});
