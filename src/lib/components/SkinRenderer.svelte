<script lang="ts">
    import { onMount } from 'svelte';
    import type { ParsedSkin } from '../types';
    import { loadSkin, applySkin } from '../tauri/ipc';

    export let skinPath: string | null = null;
    
    let currentSkin: ParsedSkin | null = null;
    let error: string | null = null;

    async function loadAndApplySkin(path: string) {
        try {
            error = null;
            currentSkin = await applySkin(path);
            applySkinsToDOM(currentSkin);
        } catch (e) {
            error = `Failed to load skin: ${e}`;
            console.error(error);
            // Fall back to default
            currentSkin = await getDefaultSkin();
            applySkinsToDOM(currentSkin);
        }
    }

    async function getDefaultSkin(): Promise<ParsedSkin> {
        return {
            name: 'default',
            assets: {},
            regions: {
                main: { x: 0, y: 0, width: 275, height: 116 }
            }
        };
    }

    function applySkinsToDOM(skin: ParsedSkin) {
        if (!skin) return;

        // Apply CSS variables for skin colors
        const root = document.documentElement;
        
        // Set default colors (can be extracted from skin assets in a more complete implementation)
        root.style.setProperty('--skin-bg-color', '#000000');
        root.style.setProperty('--skin-text-color', '#00ff00');
        root.style.setProperty('--skin-accent-color', '#0080ff');

        // Apply window dimensions if regions are defined
        if (skin.regions?.main) {
            root.style.setProperty('--skin-width', `${skin.regions.main.width}px`);
            root.style.setProperty('--skin-height', `${skin.regions.main.height}px`);
        }

        // In a complete implementation, we would:
        // 1. Convert asset byte arrays to data URLs
        // 2. Map assets to specific UI regions (buttons, sliders, etc.)
        // 3. Apply as background images via CSS
    }

    function resetToDefault() {
        getDefaultSkin().then(skin => {
            currentSkin = skin;
            applySkinsToDOM(skin);
        });
    }

    onMount(() => {
        if (skinPath) {
            loadAndApplySkin(skinPath);
        } else {
            resetToDefault();
        }
    });

    // Reactive statement to reload skin when path changes
    $: if (skinPath) {
        loadAndApplySkin(skinPath);
    }
</script>

<div class="skin-renderer">
    {#if error}
        <div class="error-message">
            {error}
        </div>
    {/if}
    
    {#if currentSkin}
        <div class="skin-info">
            <span>Skin: {currentSkin.name}</span>
        </div>
    {/if}
</div>

<style>
    .skin-renderer {
        position: relative;
    }

    .error-message {
        color: red;
        font-size: 12px;
        padding: 4px;
    }

    .skin-info {
        font-size: 10px;
        color: var(--skin-text-color, #00ff00);
        padding: 2px;
    }
</style>
