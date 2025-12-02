<script lang="ts">
    import { onMount } from 'svelte';
    import { farmerStore } from '../stores';
    import { 
        spotifyAuthenticate, 
        spotifyRefreshToken,
        storeCredential,
        retrieveCredential,
        type SpotifyCredentials,
        type SpotifyToken
    } from '../tauri';
    import { invoke } from '@tauri-apps/api/core';

    // Props
    let {
        service = 'spotify',
        onClose = () => {},
        onSuccess = () => {}
    }: {
        service?: 'spotify' | 'youtube';
        onClose?: () => void;
        onSuccess?: () => void;
    } = $props();

    // State
    let step = $state<'credentials' | 'oauth' | 'complete'>('credentials');
    let isLoading = $state(false);
    let error = $state<string | null>(null);

    // Spotify OAuth credentials
    let spotifyClientId = $state('');
    let spotifyClientSecret = $state('');
    let spotifyAuthCode = $state('');

    // YouTube credentials
    let youtubeApiKey = $state('');
    let youtubeClientId = $state('');
    let youtubeClientSecret = $state('');
    let youtubeAuthCode = $state('');

    const SPOTIFY_REDIRECT_URI = 'http://localhost:8888/callback';
    const YOUTUBE_REDIRECT_URI = 'http://localhost:8888/callback';

    onMount(async () => {
        // Try to load existing credentials
        if (service === 'spotify') {
            const clientId = await retrieveCredential('spotify_client_id');
            const clientSecret = await retrieveCredential('spotify_client_secret');
            if (clientId) spotifyClientId = clientId;
            if (clientSecret) spotifyClientSecret = clientSecret;
        } else if (service === 'youtube') {
            const apiKey = await retrieveCredential('youtube_api_key');
            const clientId = await retrieveCredential('youtube_client_id');
            const clientSecret = await retrieveCredential('youtube_client_secret');
            if (apiKey) youtubeApiKey = apiKey;
            if (clientId) youtubeClientId = clientId;
            if (clientSecret) youtubeClientSecret = clientSecret;
        }
    });

    async function handleSpotifyCredentialsSubmit() {
        if (!spotifyClientId || !spotifyClientSecret) {
            error = 'Please enter both Client ID and Client Secret';
            return;
        }

        isLoading = true;
        error = null;

        try {
            // Store credentials securely
            await storeCredential('spotify_client_id', spotifyClientId);
            await storeCredential('spotify_client_secret', spotifyClientSecret);

            // Move to OAuth step
            step = 'oauth';
            farmerStore.prompt('Now authorize milk to access your Spotify account');
        } catch (err) {
            console.error('Failed to store Spotify credentials:', err);
            error = 'Failed to store credentials. Please try again.';
            farmerStore.showError('Failed to store credentials');
        } finally {
            isLoading = false;
        }
    }

    async function handleSpotifyOAuthSubmit() {
        if (!spotifyAuthCode) {
            error = 'Please enter the authorization code';
            return;
        }

        isLoading = true;
        error = null;

        try {
            const credentials: SpotifyCredentials = {
                client_id: spotifyClientId,
                client_secret: spotifyClientSecret,
                redirect_uri: SPOTIFY_REDIRECT_URI
            };

            const token = await spotifyAuthenticate(credentials, spotifyAuthCode);
            
            step = 'complete';
            farmerStore.celebrate('Spotify connected successfully!');
            
            setTimeout(() => {
                onSuccess();
                onClose();
            }, 2000);
        } catch (err) {
            console.error('Spotify authentication failed:', err);
            error = String(err);
            farmerStore.showError('Authentication failed. Check your authorization code.');
        } finally {
            isLoading = false;
        }
    }

    async function handleYoutubeApiKeySubmit() {
        if (!youtubeApiKey) {
            error = 'Please enter your YouTube API key';
            return;
        }

        isLoading = true;
        error = null;

        try {
            // Validate API key
            const isValid = await invoke<boolean>('youtube_validate_api_key', { apiKey: youtubeApiKey });
            
            if (!isValid) {
                error = 'Invalid API key. Please check and try again.';
                farmerStore.showError('Invalid YouTube API key');
                isLoading = false;
                return;
            }

            // Store API key
            await invoke('youtube_store_api_key', { apiKey: youtubeApiKey });
            await storeCredential('youtube_api_key', youtubeApiKey);

            // If OAuth is also needed, move to OAuth step
            if (youtubeClientId && youtubeClientSecret) {
                step = 'oauth';
                farmerStore.prompt('Now authorize milk to access your YouTube account');
            } else {
                step = 'complete';
                farmerStore.celebrate('YouTube API key saved successfully!');
                
                setTimeout(() => {
                    onSuccess();
                    onClose();
                }, 2000);
            }
        } catch (err) {
            console.error('Failed to validate YouTube API key:', err);
            error = 'Failed to validate API key. Please try again.';
            farmerStore.showError('Failed to validate API key');
        } finally {
            isLoading = false;
        }
    }

    async function handleYoutubeOAuthSubmit() {
        if (!youtubeAuthCode) {
            error = 'Please enter the authorization code';
            return;
        }

        isLoading = true;
        error = null;

        try {
            const credentials = {
                client_id: youtubeClientId,
                client_secret: youtubeClientSecret,
                redirect_uri: YOUTUBE_REDIRECT_URI
            };

            await invoke('youtube_authenticate', { credentials, authCode: youtubeAuthCode });
            
            // Store OAuth credentials
            await storeCredential('youtube_client_id', youtubeClientId);
            await storeCredential('youtube_client_secret', youtubeClientSecret);

            step = 'complete';
            farmerStore.celebrate('YouTube connected successfully!');
            
            setTimeout(() => {
                onSuccess();
                onClose();
            }, 2000);
        } catch (err) {
            console.error('YouTube authentication failed:', err);
            error = String(err);
            farmerStore.showError('Authentication failed. Check your authorization code.');
        } finally {
            isLoading = false;
        }
    }

    function openSpotifyAuthUrl() {
        const scopes = 'user-read-currently-playing user-read-playback-state';
        const authUrl = `https://accounts.spotify.com/authorize?client_id=${encodeURIComponent(spotifyClientId)}&response_type=code&redirect_uri=${encodeURIComponent(SPOTIFY_REDIRECT_URI)}&scope=${encodeURIComponent(scopes)}`;
        
        // Open in browser
        window.open(authUrl, '_blank');
    }

    function openYoutubeAuthUrl() {
        const scopes = 'https://www.googleapis.com/auth/youtube.readonly';
        const authUrl = `https://accounts.google.com/o/oauth2/v2/auth?client_id=${encodeURIComponent(youtubeClientId)}&response_type=code&redirect_uri=${encodeURIComponent(YOUTUBE_REDIRECT_URI)}&scope=${encodeURIComponent(scopes)}&access_type=offline`;
        
        // Open in browser
        window.open(authUrl, '_blank');
    }

    function handleClose() {
        farmerStore.transition('idle');
        onClose();
    }
</script>

<div class="dialog-overlay" onclick={handleClose} role="button" tabindex="0" onkeydown={(e) => e.key === 'Escape' && handleClose()}>
    <div class="dialog-content" onclick={(e) => e.stopPropagation()} role="dialog" aria-modal="true">
        <div class="dialog-header">
            <h2>{service === 'spotify' ? 'Connect Spotify' : 'Connect YouTube'}</h2>
            <button class="close-button" onclick={handleClose}>×</button>
        </div>

        <div class="dialog-body">
            {#if service === 'spotify'}
                {#if step === 'credentials'}
                    <div class="step-content">
                        <p class="instructions">
                            To connect Spotify, you'll need to create a Spotify Developer application:
                        </p>
                        <ol class="instructions-list">
                            <li>Go to <a href="https://developer.spotify.com/dashboard" target="_blank">Spotify Developer Dashboard</a></li>
                            <li>Create a new app</li>
                            <li>Add <code>{SPOTIFY_REDIRECT_URI}</code> as a Redirect URI</li>
                            <li>Copy your Client ID and Client Secret below</li>
                        </ol>

                        <div class="input-group">
                            <label for="spotify-client-id">Client ID:</label>
                            <input
                                id="spotify-client-id"
                                type="text"
                                bind:value={spotifyClientId}
                                placeholder="Enter your Spotify Client ID"
                                disabled={isLoading}
                            />
                        </div>

                        <div class="input-group">
                            <label for="spotify-client-secret">Client Secret:</label>
                            <input
                                id="spotify-client-secret"
                                type="password"
                                bind:value={spotifyClientSecret}
                                placeholder="Enter your Spotify Client Secret"
                                disabled={isLoading}
                            />
                        </div>

                        {#if error}
                            <div class="error-message">{error}</div>
                        {/if}

                        <div class="button-group">
                            <button class="secondary-button" onclick={handleClose} disabled={isLoading}>
                                Cancel
                            </button>
                            <button 
                                class="primary-button" 
                                onclick={handleSpotifyCredentialsSubmit}
                                disabled={isLoading || !spotifyClientId || !spotifyClientSecret}
                            >
                                {isLoading ? 'Saving...' : 'Next'}
                            </button>
                        </div>
                    </div>
                {:else if step === 'oauth'}
                    <div class="step-content">
                        <p class="instructions">
                            Now authorize milk to access your Spotify account:
                        </p>
                        <ol class="instructions-list">
                            <li>Click the button below to open Spotify authorization</li>
                            <li>Log in and authorize the application</li>
                            <li>Copy the authorization code from the redirect URL</li>
                            <li>Paste it below</li>
                        </ol>

                        <button class="auth-button" onclick={openSpotifyAuthUrl}>
                            Open Spotify Authorization
                        </button>

                        <div class="input-group">
                            <label for="spotify-auth-code">Authorization Code:</label>
                            <input
                                id="spotify-auth-code"
                                type="text"
                                bind:value={spotifyAuthCode}
                                placeholder="Paste authorization code here"
                                disabled={isLoading}
                            />
                        </div>

                        {#if error}
                            <div class="error-message">{error}</div>
                        {/if}

                        <div class="button-group">
                            <button class="secondary-button" onclick={() => step = 'credentials'} disabled={isLoading}>
                                Back
                            </button>
                            <button 
                                class="primary-button" 
                                onclick={handleSpotifyOAuthSubmit}
                                disabled={isLoading || !spotifyAuthCode}
                            >
                                {isLoading ? 'Authenticating...' : 'Connect'}
                            </button>
                        </div>
                    </div>
                {:else if step === 'complete'}
                    <div class="step-content success">
                        <div class="success-icon">✓</div>
                        <p>Spotify connected successfully!</p>
                    </div>
                {/if}
            {:else if service === 'youtube'}
                {#if step === 'credentials'}
                    <div class="step-content">
                        <p class="instructions">
                            To connect YouTube, you'll need a YouTube Data API key:
                        </p>
                        <ol class="instructions-list">
                            <li>Go to <a href="https://console.cloud.google.com/" target="_blank">Google Cloud Console</a></li>
                            <li>Create a new project or select an existing one</li>
                            <li>Enable the YouTube Data API v3</li>
                            <li>Create an API key in Credentials</li>
                            <li>Copy your API key below</li>
                        </ol>

                        <div class="input-group">
                            <label for="youtube-api-key">API Key:</label>
                            <input
                                id="youtube-api-key"
                                type="password"
                                bind:value={youtubeApiKey}
                                placeholder="Enter your YouTube API key"
                                disabled={isLoading}
                            />
                        </div>

                        <details class="oauth-details">
                            <summary>Optional: OAuth for enhanced features</summary>
                            <div class="oauth-fields">
                                <div class="input-group">
                                    <label for="youtube-client-id">OAuth Client ID:</label>
                                    <input
                                        id="youtube-client-id"
                                        type="text"
                                        bind:value={youtubeClientId}
                                        placeholder="Optional"
                                        disabled={isLoading}
                                    />
                                </div>

                                <div class="input-group">
                                    <label for="youtube-client-secret">OAuth Client Secret:</label>
                                    <input
                                        id="youtube-client-secret"
                                        type="password"
                                        bind:value={youtubeClientSecret}
                                        placeholder="Optional"
                                        disabled={isLoading}
                                    />
                                </div>
                            </div>
                        </details>

                        {#if error}
                            <div class="error-message">{error}</div>
                        {/if}

                        <div class="button-group">
                            <button class="secondary-button" onclick={handleClose} disabled={isLoading}>
                                Cancel
                            </button>
                            <button 
                                class="primary-button" 
                                onclick={handleYoutubeApiKeySubmit}
                                disabled={isLoading || !youtubeApiKey}
                            >
                                {isLoading ? 'Validating...' : 'Next'}
                            </button>
                        </div>
                    </div>
                {:else if step === 'oauth'}
                    <div class="step-content">
                        <p class="instructions">
                            Now authorize milk to access your YouTube account:
                        </p>
                        <ol class="instructions-list">
                            <li>Click the button below to open Google authorization</li>
                            <li>Log in and authorize the application</li>
                            <li>Copy the authorization code from the redirect URL</li>
                            <li>Paste it below</li>
                        </ol>

                        <button class="auth-button" onclick={openYoutubeAuthUrl}>
                            Open Google Authorization
                        </button>

                        <div class="input-group">
                            <label for="youtube-auth-code">Authorization Code:</label>
                            <input
                                id="youtube-auth-code"
                                type="text"
                                bind:value={youtubeAuthCode}
                                placeholder="Paste authorization code here"
                                disabled={isLoading}
                            />
                        </div>

                        {#if error}
                            <div class="error-message">{error}</div>
                        {/if}

                        <div class="button-group">
                            <button class="secondary-button" onclick={() => step = 'credentials'} disabled={isLoading}>
                                Back
                            </button>
                            <button 
                                class="primary-button" 
                                onclick={handleYoutubeOAuthSubmit}
                                disabled={isLoading || !youtubeAuthCode}
                            >
                                {isLoading ? 'Authenticating...' : 'Connect'}
                            </button>
                        </div>
                    </div>
                {:else if step === 'complete'}
                    <div class="step-content success">
                        <div class="success-icon">✓</div>
                        <p>YouTube connected successfully!</p>
                    </div>
                {/if}
            {/if}
        </div>
    </div>
</div>

<style>
    .dialog-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.7);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 2000;
        animation: fadeIn 0.2s ease;
    }

    .dialog-content {
        background: white;
        border-radius: 12px;
        width: 90%;
        max-width: 600px;
        max-height: 90vh;
        overflow-y: auto;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
        animation: slideUp 0.3s ease;
    }

    .dialog-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 24px;
        border-bottom: 1px solid #e0e0e0;
    }

    .dialog-header h2 {
        margin: 0;
        font-size: 24px;
        font-weight: 600;
        color: #333;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 32px;
        color: #999;
        cursor: pointer;
        padding: 0;
        width: 32px;
        height: 32px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 4px;
        transition: all 0.2s;
    }

    .close-button:hover {
        background: #f0f0f0;
        color: #333;
    }

    .dialog-body {
        padding: 24px;
    }

    .step-content {
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .step-content.success {
        align-items: center;
        padding: 40px 0;
    }

    .success-icon {
        width: 80px;
        height: 80px;
        border-radius: 50%;
        background: #4CAF50;
        color: white;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 48px;
        font-weight: bold;
        margin-bottom: 16px;
    }

    .instructions {
        margin: 0;
        color: #666;
        line-height: 1.6;
    }

    .instructions-list {
        margin: 0;
        padding-left: 24px;
        color: #666;
        line-height: 1.8;
    }

    .instructions-list li {
        margin-bottom: 8px;
    }

    .instructions-list a {
        color: #667eea;
        text-decoration: none;
    }

    .instructions-list a:hover {
        text-decoration: underline;
    }

    .instructions-list code {
        background: #f5f5f5;
        padding: 2px 6px;
        border-radius: 3px;
        font-family: monospace;
        font-size: 14px;
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .input-group label {
        font-size: 14px;
        font-weight: 600;
        color: #333;
    }

    .input-group input {
        padding: 12px 16px;
        font-size: 16px;
        border: 2px solid #ddd;
        border-radius: 8px;
        transition: border-color 0.2s;
    }

    .input-group input:focus {
        outline: none;
        border-color: #667eea;
    }

    .input-group input:disabled {
        background: #f5f5f5;
        cursor: not-allowed;
    }

    .auth-button {
        padding: 16px 24px;
        font-size: 16px;
        font-weight: 600;
        background: #667eea;
        color: white;
        border: none;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s;
    }

    .auth-button:hover {
        background: #5568d3;
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
    }

    .oauth-details {
        border: 1px solid #e0e0e0;
        border-radius: 8px;
        padding: 16px;
    }

    .oauth-details summary {
        cursor: pointer;
        font-weight: 600;
        color: #667eea;
        user-select: none;
    }

    .oauth-details summary:hover {
        color: #5568d3;
    }

    .oauth-fields {
        margin-top: 16px;
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .error-message {
        padding: 12px 16px;
        background: #FFEBEE;
        border: 1px solid #F44336;
        border-radius: 8px;
        color: #C62828;
        font-size: 14px;
    }

    .button-group {
        display: flex;
        gap: 12px;
        justify-content: flex-end;
        margin-top: 8px;
    }

    button {
        padding: 12px 24px;
        font-size: 16px;
        font-weight: 600;
        border: none;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s;
    }

    button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .primary-button {
        background: #667eea;
        color: white;
    }

    .primary-button:hover:not(:disabled) {
        background: #5568d3;
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
    }

    .secondary-button {
        background: #e0e0e0;
        color: #333;
    }

    .secondary-button:hover:not(:disabled) {
        background: #d0d0d0;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    @keyframes slideUp {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
</style>
