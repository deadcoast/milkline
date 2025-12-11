import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import VideoEditor from "./VideoEditor.svelte";

// Mock Tauri API
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
  convertFileSrc: vi.fn((path: string) => `asset://localhost/${path}`),
}));

describe("VideoEditor Component - Unit Tests", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  /**
   * Test component renders with loading state
   * Requirements: 2.1
   */
  it("should render loading state initially", () => {
    const { container } = render(VideoEditor, {
      props: {
        filePath: "/path/to/video.mp4",
      },
    });

    const loadingElement = container.querySelector(".loading");
    expect(loadingElement).toBeTruthy();
    expect(loadingElement?.textContent).toContain("Loading video");
  });

  /**
   * Test component handles missing file path
   */
  it("should handle empty file path gracefully", () => {
    const { container } = render(VideoEditor, {
      props: {
        filePath: "",
      },
    });

    // Should render without crashing
    expect(container).toBeTruthy();
  });

  /**
   * Test export method exists and is callable
   * Requirements: 2.4, 2.5, 3.5, 5.4
   */
  it("should expose exportVideo method", async () => {
    const { component } = render(VideoEditor, {
      props: {
        filePath: "/path/to/video.mp4",
      },
    });

    // Verify exportVideo method exists
    expect(component.exportVideo).toBeDefined();
    expect(typeof component.exportVideo).toBe("function");
  });

  /**
   * Test clearCrop method exists and is callable
   * Requirements: 2.3
   */
  it("should expose clearCrop method", () => {
    const { component } = render(VideoEditor, {
      props: {
        filePath: "/path/to/video.mp4",
      },
    });

    // Verify clearCrop method exists
    expect(component.clearCrop).toBeDefined();
    expect(typeof component.clearCrop).toBe("function");
  });

  /**
   * Test component structure
   */
  it("should have correct component structure", () => {
    const { container } = render(VideoEditor, {
      props: {
        filePath: "/path/to/video.mp4",
      },
    });

    const editor = container.querySelector(".video-editor");
    expect(editor).toBeTruthy();
  });
});
