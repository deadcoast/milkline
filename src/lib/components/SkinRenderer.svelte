<script lang="ts">
    import { onMount } from 'svelte';
    import type { ParsedSkin } from '../types';
    import { loadSkin, applySkin, getSkinAssets } from '../tauri/ipc';

    let { skinPath = $bindable(null) }: { skinPath?: string | null } = $props();
    
    let currentSkin = $state<ParsedSkin | null>(null);
    let error = $state<string | null>(null);
    let skinAssetUrls = $state<Record<string, string>>({});

    /**
     * Convert byte array to data URL for use in CSS
     */
    function byteArrayToDataUrl(bytes: number[], filename: string): string {
        // Determine MIME type from filename
        const ext = filename.toLowerCase().split('.').pop();
        let mimeType = 'image/png';
        
        if (ext === 'bmp') {
            mimeType = 'image/bmp';
        } else if (ext === 'png') {
            mimeType = 'image/png';
        } else if (ext === 'jpg' || ext === 'jpeg') {
            mimeType = 'image/jpeg';
        }

        // Convert byte array to base64
        const uint8Array = new Uint8Array(bytes);
        let binary = '';
        for (let i = 0; i < uint8Array.length; i++) {
            binary += String.fromCharCode(uint8Array[i]);
        }
        const base64 = btoa(binary);
        
        return `data:${mimeType};base64,${base64}`;
    }

    /**
     * Map Winamp skin asset names to UI regions
     */
    function mapAssetsToRegions(assets: Record<string, number[]>): Record<string, string> {
        const assetUrls: Record<string, string> = {};
        
        // Standard Winamp skin asset mappings
        const assetMap: Record<string, string> = {
            // Main window
            'main.bmp': 'main-bg',
            'titlebar.bmp': 'titlebar',
            
            // Buttons
            'cbuttons.bmp': 'control-buttons',
            'playpaus.bmp': 'play-pause-button',
            'shufrep.bmp': 'shuffle-repeat-buttons',
            
            // Position and volume
            'posbar.bmp': 'position-bar',
            'volume.bmp': 'volume-slider',
            'balance.bmp': 'balance-slider',
            
            // Display elements
            'numbers.bmp': 'numbers',
            'text.bmp': 'text',
            'monoster.bmp': 'mono-stereo',
            
            // Equalizer
            'eqmain.bmp': 'eq-main',
            'eq_ex.bmp': 'eq-extended',
            
            // Playlist
            'pledit.bmp': 'playlist',
            
            // Misc
            'mb.bmp': 'mini-browser',
            'avs.bmp': 'visualizer'
        };

        // Convert all assets to data URLs
        for (const [filename, bytes] of Object.entries(assets)) {
            const normalizedName = filename.toLowerCase().split('/').pop() || filename;
            const dataUrl = byteArrayToDataUrl(bytes, normalizedName);
            
            // Map to region name if known, otherwise use filename
            const regionName = assetMap[normalizedName] || normalizedName;
            assetUrls[regionName] = dataUrl;
        }

        return assetUrls;
    }

    /**
     * Apply skin assets to DOM via CSS variables
     */
    function applySkinsToDOM(skin: ParsedSkin, assetUrls: Record<string, string>) {
        if (!skin) return;

        const root = document.documentElement;
        
        // Apply window dimensions if regions are defined
        if (skin.regions?.main) {
            root.style.setProperty('--skin-width', `${skin.regions.main.width}px`);
            root.style.setProperty('--skin-height', `${skin.regions.main.height}px`);
        } else {
            // Default Winamp dimensions
            root.style.setProperty('--skin-width', '275px');
            root.style.setProperty('--skin-height', '116px');
        }

        // Apply asset URLs as CSS variables
        for (const [regionName, dataUrl] of Object.entries(assetUrls)) {
            root.style.setProperty(`--skin-${regionName}`, `url("${dataUrl}")`);
        }

        // Set default colors (extracted from classic Winamp aesthetic)
        root.style.setProperty('--skin-bg-color', '#000000');
        root.style.setProperty('--skin-text-color', '#00ff00');
        root.style.setProperty('--skin-accent-color', '#0080ff');
        root.style.setProperty('--skin-border-color', '#404040');
        
        // Apply main background if available
        if (assetUrls['main-bg']) {
            root.style.setProperty('--color-player-bg', '#000000');
        }
    }

    /**
     * Load and apply a skin file
     */
    async function loadAndApplySkin(path: string) {
        try {
            error = null;
            
            // Apply the skin (this also saves it to config)
            currentSkin = await applySkin(path);
            
            // Get the actual asset byte arrays
            const assets = await getSkinAssets(path);
            
            // Convert assets to data URLs and map to regions
            skinAssetUrls = mapAssetsToRegions(assets);
            
            // Apply to DOM
            applySkinsToDOM(currentSkin, skinAssetUrls);
            
            console.log(`Skin "${currentSkin.name}" applied successfully with ${Object.keys(skinAssetUrls).length} assets`);
        } catch (e) {
            error = `Failed to load skin: ${e}`;
            console.error(error);
            
            // Fall back to default
            currentSkin = await getDefaultSkin();
            skinAssetUrls = {};
            applySkinsToDOM(currentSkin, skinAssetUrls);
        }
    }

    /**
     * Get default fallback skin
     */
    async function getDefaultSkin(): Promise<ParsedSkin> {
        return {
            name: 'default',
            assets: {},
            regions: {
                main: { x: 0, y: 0, width: 275, height: 116 }
            }
        };
    }

    /**
     * Reset to default skin
     */
    export function resetToDefault() {
        getDefaultSkin().then(skin => {
            currentSkin = skin;
            skinAssetUrls = {};
            applySkinsToDOM(skin, skinAssetUrls);
        });
    }

    onMount(() => {
        if (skinPath) {
            loadAndApplySkin(skinPath);
        } else {
            resetToDefault();
        }
    });

    // Reload skin when path changes
    $effect(() => {
        if (skinPath) {
            loadAndApplySkin(skinPath);
        }
    });
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
