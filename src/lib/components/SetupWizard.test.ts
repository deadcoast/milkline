import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import SetupWizard from "./SetupWizard.svelte";
import * as tauri from "../tauri";

// Mock Tauri IPC functions
vi.mock("../tauri", () => ({
  saveConfig: vi.fn().mockResolvedValue(undefined),
  validateDirectoryPath: vi.fn().mockResolvedValue(true),
}));

describe("SetupWizard", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should render welcome step initially", () => {
    render(SetupWizard);
    expect(screen.getByText("Welcome to milk!")).toBeTruthy();
    expect(screen.getByText("Let's Go!")).toBeTruthy();
  });

  it("should progress to library step when clicking next", async () => {
    render(SetupWizard);
    const nextButton = screen.getByText("Let's Go!");
    await fireEvent.click(nextButton);

    expect(screen.getByText("Music Library")).toBeTruthy();
    expect(screen.getByLabelText("Library Path:")).toBeTruthy();
  });

  it("should validate library path before proceeding", async () => {
    const validateSpy = vi.spyOn(tauri, "validateDirectoryPath");
    render(SetupWizard);

    // Go to library step
    const welcomeNext = screen.getByText("Let's Go!");
    await fireEvent.click(welcomeNext);

    // Enter a path
    const input = screen.getByLabelText("Library Path:") as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "/test/path" } });

    // Click next
    const nextButton = screen.getByText("Next");
    await fireEvent.click(nextButton);

    expect(validateSpy).toHaveBeenCalledWith("/test/path");
  });

  it("should show error for invalid library path", async () => {
    vi.spyOn(tauri, "validateDirectoryPath").mockResolvedValue(false);
    render(SetupWizard);

    // Go to library step
    const welcomeNext = screen.getByText("Let's Go!");
    await fireEvent.click(welcomeNext);

    // Enter a path
    const input = screen.getByLabelText("Library Path:") as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "/invalid/path" } });

    // Click next
    const nextButton = screen.getByText("Next");
    await fireEvent.click(nextButton);

    // Wait for validation
    await new Promise((resolve) => setTimeout(resolve, 100));

    expect(
      screen.getByText("Directory does not exist or is not accessible"),
    ).toBeTruthy();
  });

  it("should allow skipping library setup", async () => {
    render(SetupWizard);

    // Go to library step
    const welcomeNext = screen.getByText("Let's Go!");
    await fireEvent.click(welcomeNext);

    // Click skip
    const skipButton = screen.getByText("Skip for Now");
    await fireEvent.click(skipButton);

    expect(screen.getByText("Streaming Services")).toBeTruthy();
  });

  it("should save configuration on completion", async () => {
    const saveSpy = vi.spyOn(tauri, "saveConfig");
    const onComplete = vi.fn();

    render(SetupWizard, { props: { onComplete } });

    // Go through all steps
    const welcomeNext = screen.getByText("Let's Go!");
    await fireEvent.click(welcomeNext);

    const skipLibrary = screen.getByText("Skip for Now");
    await fireEvent.click(skipLibrary);

    const finishButton = screen.getByText("Finish Setup");
    await fireEvent.click(finishButton);

    // Wait for save
    await new Promise((resolve) => setTimeout(resolve, 100));

    expect(saveSpy).toHaveBeenCalled();
  });

  it("should enable streaming services when checkboxes are checked", async () => {
    const saveSpy = vi.spyOn(tauri, "saveConfig");

    render(SetupWizard);

    // Go to streaming step
    const welcomeNext = screen.getByText("Let's Go!");
    await fireEvent.click(welcomeNext);

    const skipLibrary = screen.getByText("Skip for Now");
    await fireEvent.click(skipLibrary);

    // Enable Spotify
    const spotifyCheckbox = screen.getByLabelText(
      "Enable Spotify integration",
    ) as HTMLInputElement;
    await fireEvent.click(spotifyCheckbox);

    const finishButton = screen.getByText("Finish Setup");
    await fireEvent.click(finishButton);

    // Wait for save
    await new Promise((resolve) => setTimeout(resolve, 100));

    expect(saveSpy).toHaveBeenCalledWith(
      expect.objectContaining({
        spotifyEnabled: true,
        youtubeEnabled: false,
      }),
    );
  });
});
