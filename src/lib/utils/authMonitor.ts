// Authentication monitoring and re-authentication handling
import { farmerStore } from '../stores/farmerStore';
import { spotifyGetNowPlaying, youtubeGetNowPlaying } from '../tauri/ipc';

let spotifyMonitorInterval: number | undefined;
let youtubeMonitorInterval: number | undefined;

export interface AuthMonitorCallbacks {
    onSpotifyAuthRequired?: () => void;
    onYoutubeAuthRequired?: () => void;
}

/**
 * Start monitoring Spotify authentication status
 */
export function startSpotifyAuthMonitor(callbacks: AuthMonitorCallbacks) {
    // Stop existing monitor if any
    stopSpotifyAuthMonitor();

    // Poll every 30 seconds to check if auth is still valid
    spotifyMonitorInterval = window.setInterval(async () => {
        try {
            await spotifyGetNowPlaying();
        } catch (error) {
            const errorMsg = String(error).toLowerCase();
            
            // Check if it's an authentication error
            if (errorMsg.includes('token expired') || 
                errorMsg.includes('authentication') || 
                errorMsg.includes('unauthorized') ||
                errorMsg.includes('401')) {
                
                // Show farmer error and trigger re-auth callback
                farmerStore.showError('Spotify authentication expired. Please reconnect.');
                
                if (callbacks.onSpotifyAuthRequired) {
                    callbacks.onSpotifyAuthRequired();
                }
                
                // Stop monitoring until re-authenticated
                stopSpotifyAuthMonitor();
            }
        }
    }, 30000); // Check every 30 seconds
}

/**
 * Stop monitoring Spotify authentication
 */
export function stopSpotifyAuthMonitor() {
    if (spotifyMonitorInterval !== undefined) {
        clearInterval(spotifyMonitorInterval);
        spotifyMonitorInterval = undefined;
    }
}

/**
 * Start monitoring YouTube authentication status
 */
export function startYoutubeAuthMonitor(callbacks: AuthMonitorCallbacks) {
    // Stop existing monitor if any
    stopYoutubeAuthMonitor();

    // Poll every 30 seconds to check if auth is still valid
    youtubeMonitorInterval = window.setInterval(async () => {
        try {
            await youtubeGetNowPlaying();
        } catch (error) {
            const errorMsg = String(error).toLowerCase();
            
            // Check if it's an authentication error
            if (errorMsg.includes('token expired') || 
                errorMsg.includes('authentication') || 
                errorMsg.includes('unauthorized') ||
                errorMsg.includes('401') ||
                errorMsg.includes('403')) {
                
                // Show farmer error and trigger re-auth callback
                farmerStore.showError('YouTube authentication expired. Please reconnect.');
                
                if (callbacks.onYoutubeAuthRequired) {
                    callbacks.onYoutubeAuthRequired();
                }
                
                // Stop monitoring until re-authenticated
                stopYoutubeAuthMonitor();
            }
        }
    }, 30000); // Check every 30 seconds
}

/**
 * Stop monitoring YouTube authentication
 */
export function stopYoutubeAuthMonitor() {
    if (youtubeMonitorInterval !== undefined) {
        clearInterval(youtubeMonitorInterval);
        youtubeMonitorInterval = undefined;
    }
}

/**
 * Stop all authentication monitors
 */
export function stopAllAuthMonitors() {
    stopSpotifyAuthMonitor();
    stopYoutubeAuthMonitor();
}
