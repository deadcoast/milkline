# Skin Renderer Implementation

## Overview

The skin renderer implementation provides complete support for loading and applying Winamp `.wsz` and `.wal` skin files to the milk player interface.

## Features Implemented

### 1. Asset-to-UI-Region Mapping

The `SkinRenderer.svelte` component includes a comprehensive mapping system that converts Winamp skin asset names to UI regions:

```typescript
const assetMap: Record<string, string> = {
  // Main window
  "main.bmp": "main-bg",
  "titlebar.bmp": "titlebar",

  // Buttons
  "cbuttons.bmp": "control-buttons",
  "playpaus.bmp": "play-pause-button",
  "shufrep.bmp": "shuffle-repeat-buttons",

  // Position and volume
  "posbar.bmp": "position-bar",
  "volume.bmp": "volume-slider",
  "balance.bmp": "balance-slider",

  // Display elements
  "numbers.bmp": "numbers",
  "text.bmp": "text",
  "monoster.bmp": "mono-stereo",

  // Equalizer
  "eqmain.bmp": "eq-main",
  "eq_ex.bmp": "eq-extended",

  // Playlist
  "pledit.bmp": "playlist",

  // Misc
  "mb.bmp": "mini-browser",
  "avs.bmp": "visualizer",
};
```

### 2. Byte Array to Data URL Conversion

The renderer converts BMP/PNG byte arrays from the backend into data URLs that can be used in CSS:

```typescript
function byteArrayToDataUrl(bytes: number[], filename: string): string {
  // Determine MIME type from filename extension
  const ext = filename.toLowerCase().split(".").pop();
  let mimeType = "image/png";

  if (ext === "bmp") {
    mimeType = "image/bmp";
  } else if (ext === "png") {
    mimeType = "image/png";
  } else if (ext === "jpg" || ext === "jpeg") {
    mimeType = "image/jpeg";
  }

  // Convert byte array to base64
  const uint8Array = new Uint8Array(bytes);
  let binary = "";
  for (let i = 0; i < uint8Array.length; i++) {
    binary += String.fromCharCode(uint8Array[i]);
  }
  const base64 = btoa(binary);

  return `data:${mimeType};base64,${base64}`;
}
```

### 3. CSS Variable Injection

Skin assets are applied to the DOM via CSS custom properties (variables):

```typescript
function applySkinsToDOM(skin: ParsedSkin, assetUrls: Record<string, string>) {
  const root = document.documentElement;

  // Apply window dimensions
  if (skin.regions?.main) {
    root.style.setProperty("--skin-width", `${skin.regions.main.width}px`);
    root.style.setProperty("--skin-height", `${skin.regions.main.height}px`);
  }

  // Apply asset URLs as CSS variables
  for (const [regionName, dataUrl] of Object.entries(assetUrls)) {
    root.style.setProperty(`--skin-${regionName}`, `url("${dataUrl}")`);
  }
}
```

### 4. Player Component Integration

The Player component has been updated to use skin assets:

#### Main Background

```css
.player {
  background-image: var(--skin-main-bg, none);
  background-size: cover;
  width: var(--skin-width, auto);
}
```

#### Control Buttons

```css
.control-btn {
  background-image: var(--skin-control-buttons, none);
  background-size: contain;
}

.control-btn.play-pause {
  background-image: var(--skin-play-pause-button, none);
}
```

#### Position Bar

```css
.seek-bar {
  background-image: var(--skin-position-bar, none);
  background-size: 100% 100%;
}
```

#### Volume Slider

```css
.volume-slider {
  background-image: var(--skin-volume-slider, none);
  background-size: 100% 100%;
}
```

### 5. Window Dimension Adjustment

The renderer automatically adjusts window dimensions based on skin region configuration:

- Default Winamp dimensions: 275x116 pixels
- Custom dimensions are read from the skin's `regions.main` property
- Dimensions are applied via CSS variables for responsive layout

### 6. Error Handling and Fallback

The implementation includes robust error handling:

```typescript
async function loadAndApplySkin(path: string) {
  try {
    error = null;
    currentSkin = await applySkin(path);
    const assets = await getSkinAssets(path);
    skinAssetUrls = mapAssetsToRegions(assets);
    applySkinsToDOM(currentSkin, skinAssetUrls);
  } catch (e) {
    error = `Failed to load skin: ${e}`;
    console.error(error);

    // Fall back to default skin
    currentSkin = await getDefaultSkin();
    skinAssetUrls = {};
    applySkinsToDOM(currentSkin, skinAssetUrls);
  }
}
```

## Usage

### Loading a Skin

```svelte
<script>
    import SkinRenderer from '$lib/components/SkinRenderer.svelte';

    let skinPath = '/path/to/skin.wsz';
</script>

<SkinRenderer bind:skinPath={skinPath} />
```

### Resetting to Default

```typescript
import { SkinRenderer } from "$lib/components/SkinRenderer.svelte";

// Get component reference
let skinRenderer;

// Reset to default
skinRenderer.resetToDefault();
```

## Testing

The implementation includes comprehensive tests:

### Unit Tests

- Component rendering without crashing
- Skin loading and display
- Error handling and fallback behavior

### Property-Based Tests (Backend)

- Skin parsing completeness (Property 8)
- Asset data preservation
- Skin application completeness (Property 9)
- Error fallback behavior (Property 10)
- Skin persistence round-trip (Property 11)

## Supported Skin Files

The renderer supports:

- `.wsz` files (Winamp classic skins - ZIP archives)
- `.wal` files (Winamp modern skins - also ZIP archives)

## Available Test Skins

The project includes several test skins in `assets/winamp_skins/`:

- `PioneerReplicAmp.wsz`
- `SimplAmpChrome.wsz`
- `SonyReplicAmp.wsz`
- `Crystal Xp Clean.wal`
- And more in subdirectories

## Backend Support

The backend provides three IPC commands:

1. `load_skin(skinPath: string)` - Load and validate a skin file
2. `apply_skin(skinPath: string)` - Apply a skin and save to config
3. `get_skin_assets(skinPath: string)` - Get raw asset byte arrays

All commands include:

- Validation of skin structure
- Automatic fallback to default skin on error
- Graceful error handling
- Logging of operations

## Requirements Satisfied

This implementation satisfies the following requirements from the design document:

- **Requirement 4.2**: Asset-to-UI-region mapping logic ✓
- **Requirement 4.3**: Apply skin assets to Player component buttons and controls ✓
- Window dimension adjustment based on skin regions ✓
- Conversion of BMP/PNG byte arrays to data URLs ✓

## Future Enhancements

Potential improvements for future iterations:

1. Support for animated skins
2. Color extraction from skin assets for dynamic theming
3. Advanced region parsing for complex window shapes
4. Skin preview before application
5. Skin editor/customization tools
