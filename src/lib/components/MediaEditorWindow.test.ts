import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import MediaEditorWindow from './MediaEditorWindow.svelte';
import { mediaEditorStore } from '$lib/stores/mediaEditorStore';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn(),
    convertFileSrc: vi.fn((path: string) => path),
}));

// Import mocked functions
import { invoke } from '@tauri-apps/api/core';

describe('MediaEditorWindow Component - Unit Tests', () => {
    beforeEach(() => {
        // Reset store before each test
        mediaEditorStore.reset();
        
        // Clear all mocks
        vi.clearAllMocks();
    });

    afterEach(() => {
        // Reset store after each test
        mediaEditorStore.reset();
    });

    /**
     * Test save without loaded media shows info message
     * Requirements: 5.2, 5.5
     */
    it('should show info message when saving without loaded media', async () => {
        const { container } = render(MediaEditorWindow);
        
        // Find and click the Save As button
        const saveButton = Array.from(container.querySelectorAll('button'))
            .find(btn => btn.textContent === 'Save As') as HTMLButtonElement;
        
        expect(saveButton).toBeTruthy();
        
        // Click save button
        await fireEvent.click(saveButton);
        
        // Wait for async operations
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Verify info message was shown
        expect(invoke).toHaveBeenCalledWith(
            'show_message_dialog',
            expect.objectContaining({
                title: 'Info',
                message: 'No media file is loaded. Please open a file first.',
                kind: 'info'
            })
        );
        
        // Verify save dialog was NOT shown
        const saveCalls = vi.mocked(invoke).mock.calls.filter(call => call[0] === 'show_save_dialog');
        expect(saveCalls.length).toBe(0);
    });

    /**
     * Test export errors display error dialog
     * Requirements: 8.2, 8.3
     */
    it('should display error dialog when image export fails', async () => {
        // Manually set the store state to simulate a loaded image
        mediaEditorStore.loadMedia('/path/to/test.png', 'image');
        
        const { container } = render(MediaEditorWindow);
        
        // Wait for component to update
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Mock the save dialog to return an output path
        vi.mocked(invoke).mockImplementation((cmd: string) => {
            if (cmd === 'show_save_dialog') {
                return Promise.resolve('/path/to/output.png');
            }
            if (cmd === 'crop_image_command') {
                return Promise.reject(new Error('Export failed'));
            }
            return Promise.resolve(null);
        });
        
        // Find and click the Save As button
        const saveButton = Array.from(container.querySelectorAll('button'))
            .find(btn => btn.textContent === 'Save As') as HTMLButtonElement;
        
        await fireEvent.click(saveButton);
        
        // Wait for async operations
        await new Promise(resolve => setTimeout(resolve, 200));
        
        // Verify error message was shown
        const messageCalls = vi.mocked(invoke).mock.calls.filter(call => 
            call[0] === 'show_message_dialog' && 
            call[1] && typeof call[1] === 'object' && 
            'kind' in call[1] && call[1].kind === 'error'
        );
        expect(messageCalls.length).toBeGreaterThan(0);
    });

    /**
     * Test export errors display error dialog for video
     * Requirements: 8.2, 8.3
     */
    it('should display error dialog when video export fails', async () => {
        // Manually set the store state to simulate a loaded video
        mediaEditorStore.loadMedia('/path/to/test.mp4', 'video');
        
        const { container } = render(MediaEditorWindow);
        
        // Wait for component to update
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Mock the save dialog to return an output path
        vi.mocked(invoke).mockImplementation((cmd: string) => {
            if (cmd === 'show_save_dialog') {
                return Promise.resolve('/path/to/output.mp4');
            }
            if (cmd === 'trim_and_crop_video_command') {
                return Promise.reject(new Error('Export failed'));
            }
            if (cmd === 'probe_video_metadata_command') {
                return Promise.resolve({ duration_sec: 10, width: 1920, height: 1080 });
            }
            return Promise.resolve(null);
        });
        
        // Find and click the Save As button
        const saveButton = Array.from(container.querySelectorAll('button'))
            .find(btn => btn.textContent === 'Save As') as HTMLButtonElement;
        
        await fireEvent.click(saveButton);
        
        // Wait for async operations
        await new Promise(resolve => setTimeout(resolve, 200));
        
        // Verify error message was shown
        const messageCalls = vi.mocked(invoke).mock.calls.filter(call => 
            call[0] === 'show_message_dialog' && 
            call[1] && typeof call[1] === 'object' && 
            'kind' in call[1] && call[1].kind === 'error'
        );
        expect(messageCalls.length).toBeGreaterThan(0);
    });

    /**
     * Test unsupported file extension shows error
     * Requirements: 4.4
     */
    it('should show error message for unsupported file extensions', async () => {
        // Mock the open dialog to return an unsupported file
        vi.mocked(invoke).mockImplementation((cmd: string) => {
            if (cmd === 'show_open_dialog') {
                return Promise.resolve('/path/to/test.txt');
            }
            return Promise.resolve(null);
        });
        
        const { container } = render(MediaEditorWindow);
        
        // Find and click the Open button
        const openButton = Array.from(container.querySelectorAll('button'))
            .find(btn => btn.textContent === 'Open') as HTMLButtonElement;
        
        await fireEvent.click(openButton);
        
        // Wait for the dialog to process
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Verify error message was shown
        expect(invoke).toHaveBeenCalledWith(
            'show_message_dialog',
            expect.objectContaining({
                title: 'Error',
                message: expect.stringContaining('Unsupported file format'),
                kind: 'error'
            })
        );
    });

    /**
     * Test file open dialog is displayed
     * Requirements: 4.1
     */
    it('should display file open dialog when Open button is clicked', async () => {
        // Mock the open dialog to return null (user cancelled)
        vi.mocked(invoke).mockImplementation((cmd: string) => {
            if (cmd === 'show_open_dialog') {
                return Promise.resolve(null);
            }
            return Promise.resolve(null);
        });
        
        const { container } = render(MediaEditorWindow);
        
        // Find and click the Open button
        const openButton = Array.from(container.querySelectorAll('button'))
            .find(btn => btn.textContent === 'Open') as HTMLButtonElement;
        
        expect(openButton).toBeTruthy();
        
        await fireEvent.click(openButton);
        
        // Wait for async operations
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Verify open dialog was called with correct filters
        expect(invoke).toHaveBeenCalledWith(
            'show_open_dialog',
            expect.objectContaining({
                filters: expect.arrayContaining([
                    expect.objectContaining({
                        name: 'Media Files',
                        extensions: expect.arrayContaining(['png', 'jpg', 'jpeg', 'bmp', 'gif', 'mp4', 'mov', 'mkv'])
                    })
                ])
            })
        );
    });

    /**
     * Test save dialog is displayed for image
     * Requirements: 5.1
     */
    it('should display save dialog with image filters when saving image', async () => {
        // Mock the save dialog to return null (user cancelled)
        vi.mocked(invoke).mockImplementation((cmd: string) => {
            if (cmd === 'show_save_dialog') {
                return Promise.resolve(null);
            }
            return Promise.resolve(null);
        });
        
        // Set up store with loaded image
        mediaEditorStore.loadMedia('/path/to/test.png', 'image');
        
        const { container } = render(MediaEditorWindow);
        
        // Wait for component to update
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Find and click the Save As button
        const saveButton = Array.from(container.querySelectorAll('button'))
            .find(btn => btn.textContent === 'Save As') as HTMLButtonElement;
        
        await fireEvent.click(saveButton);
        
        // Wait for the dialog to process
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Verify save dialog was called with image filters
        expect(invoke).toHaveBeenCalledWith(
            'show_save_dialog',
            expect.objectContaining({
                filters: expect.arrayContaining([
                    expect.objectContaining({
                        name: 'Image Files',
                        extensions: expect.arrayContaining(['png', 'jpg', 'jpeg', 'bmp', 'gif'])
                    })
                ])
            })
        );
    });

    /**
     * Test save dialog is displayed for video
     * Requirements: 5.1
     */
    it('should display save dialog with video filters when saving video', async () => {
        // Mock the save dialog to return null (user cancelled)
        vi.mocked(invoke).mockImplementation((cmd: string) => {
            if (cmd === 'show_save_dialog') {
                return Promise.resolve(null);
            }
            return Promise.resolve(null);
        });
        
        // Set up store with loaded video
        mediaEditorStore.loadMedia('/path/to/test.mp4', 'video');
        
        const { container } = render(MediaEditorWindow);
        
        // Wait for component to update
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Find and click the Save As button
        const saveButton = Array.from(container.querySelectorAll('button'))
            .find(btn => btn.textContent === 'Save As') as HTMLButtonElement;
        
        await fireEvent.click(saveButton);
        
        // Wait for the dialog to process
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Verify save dialog was called with video filters
        expect(invoke).toHaveBeenCalledWith(
            'show_save_dialog',
            expect.objectContaining({
                filters: expect.arrayContaining([
                    expect.objectContaining({
                        name: 'Video Files',
                        extensions: expect.arrayContaining(['mp4', 'mov', 'mkv'])
                    })
                ])
            })
        );
    });

    /**
     * Test user cancelling save dialog
     * Requirements: 5.1
     */
    it('should handle user cancelling save dialog gracefully', async () => {
        // Mock the save dialog to return null (user cancelled)
        vi.mocked(invoke).mockImplementation((cmd: string) => {
            if (cmd === 'show_save_dialog') {
                return Promise.resolve(null);
            }
            return Promise.resolve(null);
        });
        
        // Set up store with loaded image
        mediaEditorStore.loadMedia('/path/to/test.png', 'image');
        
        const { container } = render(MediaEditorWindow);
        
        // Wait for component to update
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Find and click the Save As button
        const saveButton = Array.from(container.querySelectorAll('button'))
            .find(btn => btn.textContent === 'Save As') as HTMLButtonElement;
        
        await fireEvent.click(saveButton);
        
        // Wait for the dialog to process
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Verify no error message was shown (user just cancelled)
        const messageCalls = vi.mocked(invoke).mock.calls.filter(call => 
            call[0] === 'show_message_dialog' && 
            call[1] && typeof call[1] === 'object' && 
            'kind' in call[1] && call[1].kind === 'error'
        );
        expect(messageCalls.length).toBe(0);
    });

    /**
     * Test error handling when open dialog fails
     * Requirements: 8.2, 8.3
     */
    it('should display error message when open dialog fails', async () => {
        // Mock the open dialog to throw an error
        vi.mocked(invoke).mockImplementation((cmd: string) => {
            if (cmd === 'show_open_dialog') {
                return Promise.reject(new Error('Failed to open dialog'));
            }
            return Promise.resolve(null);
        });
        
        const { container } = render(MediaEditorWindow);
        
        // Find and click the Open button
        const openButton = Array.from(container.querySelectorAll('button'))
            .find(btn => btn.textContent === 'Open') as HTMLButtonElement;
        
        await fireEvent.click(openButton);
        
        // Wait for the error to be processed
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Verify error message was shown
        expect(invoke).toHaveBeenCalledWith(
            'show_message_dialog',
            expect.objectContaining({
                title: 'Error',
                message: expect.stringContaining('Failed to open file'),
                kind: 'error'
            })
        );
    });
});
