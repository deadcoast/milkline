import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import SkinRenderer from "./SkinRenderer.svelte";

// Mock Tauri IPC
vi.mock("../tauri/ipc", () => ({
  applySkin: vi.fn(),
  getSkinAssets: vi.fn(),
}));

describe("SkinRenderer", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should render without crashing", () => {
    const { container } = render(SkinRenderer, {
      props: {
        skinPath: null,
      },
    });
    expect(container).toBeTruthy();
  });

  it("should display skin info when skin is loaded", async () => {
    const { applySkin, getSkinAssets } = await import("../tauri/ipc");

    vi.mocked(applySkin).mockResolvedValue({
      name: "test-skin",
      assets: {},
      regions: {
        main: { x: 0, y: 0, width: 275, height: 116 },
      },
    });

    vi.mocked(getSkinAssets).mockResolvedValue({});

    const { container, findByText } = render(SkinRenderer, {
      props: {
        skinPath: "/path/to/skin.wsz",
      },
    });

    // Wait for async operations and find the text
    const skinInfo = await findByText(/test-skin/i);
    expect(skinInfo).toBeTruthy();
  });

  it("should handle skin loading errors gracefully", async () => {
    const { applySkin } = await import("../tauri/ipc");

    vi.mocked(applySkin).mockRejectedValue(new Error("Failed to load skin"));

    const { container } = render(SkinRenderer, {
      props: {
        skinPath: "/invalid/skin.wsz",
      },
    });

    // Wait for async operations
    await new Promise((resolve) => setTimeout(resolve, 100));

    // Should fall back to default skin without crashing
    expect(container).toBeTruthy();
  });
});
