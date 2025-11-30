<script lang="ts">
    import { onMount } from 'svelte';
    import { farmerStore, configStore } from '../stores';
    import { saveConfig, validateDirectoryPath } from '../tauri';
    import FarmerBuddy from './FarmerBuddy.svelte';
    import type { AppConfig } from '../types';
    
    // Props
    let {
        onComplete = () => {}
    }: {
        onComplete?: () => void;
    } = $props();

    // Setup state
    let currentStep = $state<'welcome' | 'library' | 'streaming' | 'complete'>('welcome');
    let libraryPath = $state('');
    let libraryPathError = $state<string | null>(null);
    let spotifyEnabled = $state(false);
    let youtubeEnabled = $state(false);
    let isValidating = $state(false);
    let isSaving = $state(false);

    onMount(() => {
        // Welcome the user with farmer
        farmerStore.prompt("Welcome to milk! Let's get you set up.");
    });

    async function validateLibraryPath(path: string): Promise<boolean> {
        if (!path || path.trim() === '') {
            libraryPathError = 'Please enter a library path';
            return false;
        }

        const trimmedPath = path.trim();
        
        // Check for invalid characters (basic check)
        if (trimmedPath.includes('<') || trimmedPath.includes('>') || 
            trimmedPath.includes('|') || trimmedPath.includes('"')) {
            libraryPathError = 'Path contains invalid characters';
            return false;
        }

        // Check minimum length
        if (trimmedPath.length < 3) {
            libraryPathError = 'Path is too short';
            return false;
        }

        // Validate with backend
        try {
            const isValid = await validateDirectoryPath(trimmedPath);
            if (!isValid) {
                libraryPathError = 'Directory does not exist or is not accessible';
                return false;
            }
        } catch (error) {
            console.error('Failed to validate path:', error);
            libraryPathError = 'Failed to validate path';
            return false;
        }

        libraryPathError = null;
        return true;
    }

    async function handleWelcomeNext() {
        currentStep = 'library';
        farmerStore.prompt("Where's your music library? Point me to your tunes!");
    }

    async function handleLibraryNext() {
        isValidating = true;
        
        const isValid = await validateLibraryPath(libraryPath);
        
        if (!isValid) {
            farmerStore.showError(libraryPathError || 'Invalid library path');
            isValidating = false;
            return;
        }

        isValidating = false;
        currentStep = 'streaming';
        farmerStore.prompt("Want to connect Spotify or YouTube? (Optional)");
    }

    async function handleLibrarySkip() {
        currentStep = 'streaming';
        farmerStore.prompt("Want to connect Spotify or YouTube? (Optional)");
    }

    async function handleStreamingNext() {
        await completeSetup();
    }

    async function handleStreamingSkip() {
        await completeSetup();
    }

    async function completeSetup() {
        isSaving = true;
        farmerStore.transition('celebrating', "You're all set! Let's rock!");

        try {
            // Get current config from store
            let currentConfig: AppConfig | null = null;
            const unsubscribe = configStore.subscribe(c => {
                currentConfig = c;
            });
            unsubscribe();

            // Create updated config with setup values
            const updatedConfig: AppConfig = {
                ...(currentConfig || {
                    libraryPath: null,
                    lastSkin: null,
                    volume: 0.7,
                    visualizerStyle: 'bars' as const,
                    spotifyEnabled: false,
                    youtubeEnabled: false,
                    windowPosition: { x: 100, y: 100 },
                    windowSize: { width: 800, height: 600 }
                }),
                libraryPath: libraryPath.trim() || null,
                spotifyEnabled,
                youtubeEnabled
            };

            // Save configuration
            await saveConfig(updatedConfig);
            configStore.setConfig(updatedConfig);

            // Wait for celebration animation
            setTimeout(() => {
                currentStep = 'complete';
                isSaving = false;
                
                // Notify parent component
                setTimeout(() => {
                    onComplete();
                }, 500);
            }, 2000);
        } catch (error) {
            console.error('Failed to save configuration:', error);
            farmerStore.showError('Failed to save settings. Please try again.');
            isSaving = false;
        }
    }

    function handleLibraryPathInput(event: Event) {
        const target = event.target as HTMLInputElement;
        libraryPath = target.value;
        libraryPathError = null;
    }
</script>

<div class="setup-wizard">
    <div class="wizard-content">
        <div class="farmer-container">
            <FarmerBuddy />
        </div>

        {#if currentStep === 'welcome'}
            <div class="step-content">
                <h1>Welcome to milk!</h1>
                <p>Let's get you set up in just a few steps.</p>
                <p>I'm farmer, your music buddy. I'll help you configure milk.</p>
                
                <div class="button-group">
                    <button class="primary-button" onclick={handleWelcomeNext}>
                        Let's Go!
                    </button>
                </div>
            </div>
        {/if}

        {#if currentStep === 'library'}
            <div class="step-content">
                <h2>Music Library</h2>
                <p>Where do you keep your music files?</p>
                
                <div class="input-group">
                    <label for="library-path">Library Path:</label>
                    <input
                        id="library-path"
                        type="text"
                        value={libraryPath}
                        oninput={handleLibraryPathInput}
                        placeholder="C:\Users\YourName\Music"
                        class:error={libraryPathError !== null}
                    />
                    {#if libraryPathError}
                        <span class="error-message">{libraryPathError}</span>
                    {/if}
                </div>

                <div class="button-group">
                    <button 
                        class="secondary-button" 
                        onclick={handleLibrarySkip}
                        disabled={isValidating}
                    >
                        Skip for Now
                    </button>
                    <button 
                        class="primary-button" 
                        onclick={handleLibraryNext}
                        disabled={isValidating}
                    >
                        {isValidating ? 'Validating...' : 'Next'}
                    </button>
                </div>
            </div>
        {/if}

        {#if currentStep === 'streaming'}
            <div class="step-content">
                <h2>Streaming Services</h2>
                <p>Connect your streaming accounts (you can do this later too)</p>
                
                <div class="checkbox-group">
                    <label>
                        <input
                            type="checkbox"
                            bind:checked={spotifyEnabled}
                        />
                        <span>Enable Spotify integration</span>
                    </label>
                    
                    <label>
                        <input
                            type="checkbox"
                            bind:checked={youtubeEnabled}
                        />
                        <span>Enable YouTube integration</span>
                    </label>
                </div>

                <div class="button-group">
                    <button 
                        class="secondary-button" 
                        onclick={handleStreamingSkip}
                        disabled={isSaving}
                    >
                        Skip
                    </button>
                    <button 
                        class="primary-button" 
                        onclick={handleStreamingNext}
                        disabled={isSaving}
                    >
                        {isSaving ? 'Saving...' : 'Finish Setup'}
                    </button>
                </div>
            </div>
        {/if}

        {#if currentStep === 'complete'}
            <div class="step-content">
                <h2>All Set!</h2>
                <p>milk is ready to play your music.</p>
            </div>
        {/if}
    </div>
</div>

<style>
    .setup-wizard {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .wizard-content {
        background: white;
        border-radius: 16px;
        padding: 48px;
        max-width: 600px;
        width: 90%;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
        position: relative;
    }

    .farmer-container {
        position: absolute;
        top: -40px;
        left: 50%;
        transform: translateX(-50%);
        width: 120px;
        height: 120px;
        background: white;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .step-content {
        margin-top: 60px;
    }

    h1 {
        font-size: 32px;
        font-weight: 700;
        color: #333;
        margin: 0 0 16px 0;
        text-align: center;
    }

    h2 {
        font-size: 24px;
        font-weight: 600;
        color: #333;
        margin: 0 0 12px 0;
        text-align: center;
    }

    p {
        font-size: 16px;
        color: #666;
        margin: 0 0 24px 0;
        text-align: center;
        line-height: 1.6;
    }

    .input-group {
        margin-bottom: 32px;
    }

    .input-group label {
        display: block;
        font-size: 14px;
        font-weight: 600;
        color: #333;
        margin-bottom: 8px;
    }

    .input-group input[type="text"] {
        width: 100%;
        padding: 12px 16px;
        font-size: 16px;
        border: 2px solid #ddd;
        border-radius: 8px;
        transition: border-color 0.2s;
        box-sizing: border-box;
    }

    .input-group input[type="text"]:focus {
        outline: none;
        border-color: #667eea;
    }

    .input-group input[type="text"].error {
        border-color: #f44336;
    }

    .error-message {
        display: block;
        color: #f44336;
        font-size: 14px;
        margin-top: 8px;
    }

    .checkbox-group {
        margin-bottom: 32px;
    }

    .checkbox-group label {
        display: flex;
        align-items: center;
        padding: 12px;
        margin-bottom: 8px;
        border: 2px solid #ddd;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s;
    }

    .checkbox-group label:hover {
        border-color: #667eea;
        background: #f8f9ff;
    }

    .checkbox-group input[type="checkbox"] {
        width: 20px;
        height: 20px;
        margin-right: 12px;
        cursor: pointer;
    }

    .checkbox-group span {
        font-size: 16px;
        color: #333;
    }

    .button-group {
        display: flex;
        gap: 12px;
        justify-content: center;
        margin-top: 32px;
    }

    button {
        padding: 12px 32px;
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
</style>
