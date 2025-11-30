# Design Document

## Overview

The Media Editor is a feature window within the milk Tauri application that provides intuitive image and video editing capabilities. It integrates with the existing Tauri + Svelte architecture, using Rust backend commands for FFmpeg video processing and image manipulation, with a Svelte frontend for the UI.

The design emphasizes:
- Integration with existing milk application architecture
- Tauri commands for backend media processing (Rust + FFmpeg)
- Svelte components for the editing UI
- Reusable crop overlay component for both images and videos
- Configurable export settings

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│              Frontend (Svelte Components)               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │MediaEditor   │  │ ImageEditor  │  │ VideoEditor  │   │
│  │  Window      │  │  Component   │  │  Component   │   │
│  └──────────────┘  └──────────────┘  └──────────────┘   │
│         │                  │                  │         │
│         └──────────────────┴──────────────────┘         │
│                            │                            │
│                   ┌────────▼────────┐                   │
│                   │  CropOverlay    │                   │
│                   │   Component     │                   │
│                   └─────────────────┘                   │
└─────────────────────────────────────────────────────────┘
                            │
                    Tauri IPC (invoke)
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│              Backend (Rust Tauri Commands)              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │ media_editor │  │ image_ops    │  │  video_ops   │   │
│  │   module     │  │   module     │  │   module     │   │
│  └──────────────┘  └──────────────┘  └──────────────┘   │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│              External Tools (System)                    │
│         image crate (Rust)    FFmpeg (subprocess)       │
└─────────────────────────────────────────────────────────┘
```

### Directory Structure

```
milk/
├── src/                           # Svelte frontend
│   ├── lib/
│   │   ├── components/
│   │   │   ├── media-editor/     # NEW: Media editor components
│   │   │   │   ├── MediaEditorWindow.svelte
│   │   │   │   ├── ImageEditor.svelte
│   │   │   │   ├── VideoEditor.svelte
│   │   │   │   ├── CropOverlay.svelte
│   │   │   │   └── Timeline.svelte
│   │   ├── stores/
│   │   │   └── mediaEditor.ts    # NEW: Media editor state
│   │   └── utils/
│   │       └── coordinates.ts    # NEW: Coordinate mapping
│   └── routes/
│       └── media-editor/         # NEW: Media editor route
│           └── +page.svelte
│
├── src-tauri/                     # Rust backend
│   └── src/
│       ├── media_editor/         # NEW: Media editor module
│       │   ├── mod.rs
│       │   ├── image_ops.rs      # Image operations
│       │   ├── video_ops.rs      # Video operations (FFmpeg)
│       │   ├── types.rs          # Shared types
│       │   └── config.rs         # Export configuration
│       └── main.rs               # Register commands
│
└── .kiro/specs/media-editor/     # Specification files
```

## Components and Interfaces

### Frontend (Svelte)

#### MediaEditorStore
Svelte store that tracks the current media file and editing operations.

```typescript
interface MediaEditorState {
  filePath: string | null;
  mediaType: 'image' | 'video' | null;
  crop: CropRect | null;        // {x, y, width, height}
  trim: TrimState | null;       // {startSec, endSec, durationSec}
  isLoading: boolean;
  error: string | null;
}
```

#### MediaEditorWindow.svelte
Main component that routes between image and video editors.

**Responsibilities:**
- File open/save dialogs
- Route to appropriate editor based on media type
- Coordinate save operations via Tauri commands

#### CropOverlay.svelte
Reusable component for drawing and managing crop rectangles.

**Props:**
- `sourceWidth: number` - Original media width
- `sourceHeight: number` - Original media height
- `previewWidth: number` - Display preview width
- `previewHeight: number` - Display preview height

**Events:**
- `on:cropChange` - Emits crop rectangle in source coordinates

#### ImageEditor.svelte
Hande preview and editing interface.

**Responsibilities:**
- Display image scaled to fit container
- Integrate CropOverlay
- Calculate preview rectangle for coordinate mapping

#### VideoEditor.svelte
Video preview and timeline interface.

**Responsibilities:**
- Display video preview frame
- Provide timeline controls for trim points
- Integrate CropOverlay

#### Timeline.svelte
Timeline component with start/end handles.

**Props:**
- `duration: number` - Total video duration in seconds
- `startTime: number` - Current start time
- `endTime: number` - Current end time

**Events:**
- `on:trimChange` - Emits {startSec, endSec}

### Backend (Rust Tauri Commands)

#### Image Operations

**Command: `crop_image`**
```rust
#[tauri::command]
async fn crop_image(
    input_path: String,
    output_path: String,
    crop_rect: CropRect,
) -> Result<(), String>
```

Uses the `image` crate to load and crop images.

#### Video Operations

**Command: `probe_video_metadata`**
```rust
#[tauri::command]
async fn probe_video_metadata(
    path: String,
) -> Result<VideoMetadata, String>

struct VideoMetadata {
    duration_sec: f64,
    width: u32,
    height: u32,
}
```

Uses FFprobe to extract video metadata.

**Command: `trim_and_crop_video`**
```rust
#[tauri::command]
async fn trim_and_crop_video(
    input_path: String,
    output_path: String,
    start_sec: f64,
    end_sec: f64,
    crop_rect: Option<CropRect>,
    config: ExportConfig,
) -> Result<(), String>
```

Uses FFmpeg subprocess to process video.

#### Shared Types

```rust
#[derive(Serialize, Deserialize)]
struct CropRect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize)]
struct ExportConfig {
    video_codec: String,
    audio_codec: String,
    quality: String,
}
```

### Configuration

#### Export Configuration (Rust)

```rust
// src-tauri/src/media_editor/config.rs

pub struct ExportDefaults {
    pub video_codec: &'static str,
    pub audio_codec: &'static str,
    pub image_format: &'static str,
    pub video_quality: &'static str,
}

pub const DEFAULT_CONFIG: ExportDefaults = ExportDefaults {
    video_codec: "libx264",
    audio_codec: "aac",
    image_format: "png",
    video_quality: "23",  // CRF value
};

pub struct ExportPreset {
    pub name: &'static str,
    pub video_codec: &'static str,
    pub crf: u8,
    pub preset: &'stattr,
    pub audio_codec: &'static str,
}

pub const PRESETS: &[ExportPreset] = &[
    ExportPreset {
        name: "high_quality_video",
        video_codec: "libx264",
        crf: 18,
        preset: "slow",
        audio_codec: "aac",
    },
    ExportPreset {
        name: "mobile_video",
        video_codec: "libx265",
        crf: 28,
        preset: "fast",
        audio_codec: "aac",
    },
];
```

## Data Models

### Coordinate Systems

The application uses three coordinate systems:

1. **Widget Coordinates**: Mouse position in the Qt widget (pixels)
2. **Preview Coordinates**: Position relative to the displayed preview area (pixels)
3. **Source Coordinates**: Position in the original media file (pixels)

**Mapping Flow:**
```
Widget Coords → Preview Coords → Source Coords
```

**Transformation:**
1. Subtract preview offset from widget coordinates
2. Calculate scale factor: `source_dimension / preview_dimension`
3. Multiply by scale factor to get source coordinates
4. Clamp to valid source bounds

### State Management

State flows unidirectionally:
```
User Input → UI Component → MediaState → Service → Backend Tool
```

The `MediaState` object is shared across components and serves as the single source of truth for the current editing session.

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Crop coordinate mapping preserves bounds

*For any* crop rectangle drawn in the preview area, when mapped to source coordinates, all coordinates SHALL be within the valid source media bounds (0 ≤ x < width, 0 ≤ y < height, width > 0, height > 0).

**Validates: Requirements 1.4, 2.4**

### Property 2: Timeline trim constraints are maintained

*For any* video timeline state, the start time SHALL always be less than or equal to the end time, and both SHALL be within the video duration bounds (0 ≤ start ≤ end ≤ duration).

**Validates: Requirements 3.2, 3.3, 3.4**

### Property 3: Media type determines editor routing

*For any* loaded media file, if the file extension is in the image set {png, jpg, jpeg, bmp, gif}, the application SHALL route to the image editor, and if the extension is in the video set {mp4, mov, mkv}, the application SHALL route to the video editor.

**Validates: Requirements 4.2, 4.3**

### Property 4: Export without crop preserves original dimensions

*For any* media file, if no crop rectangle is defined when exporting, the output file SHALL have the same dimensions as the input file.

**Validates: Requirements 1.5, 2.5**

### Property 5: Crop rectangle normalization is consistent

*For any* two crop rectangles with the same start and end points but drawn in opposite directions (top-left to bottom-right vs bottom-right to top-left), the normalized rectangles SHALL be identical.

**Validates: Requirements 1.2, 2.2, 6.2**

### Property 6: FFmpeg errors are propagated

*For any* FFmpeg operation that returns a non-zero exit code, the system SHALL raise an FFmpegError containing the stderr output.

**Validates: Requirements 8.1**

### Property 7: Video trim produces correct duration

*For any* video export with trim points start_sec and end_sec, the output video duration SHALL be approximately (end_sec - start_sec) within a tolerance of 0.1 seconds.

**Validates: Requirements 3.5**

### Property 8: Coordinate scaling is invertible

*For any* point in source coordinates, when scaled to preview coordinates and then back to source coordinates, the result SHALL be within 1 pixel of the original point.

**Validates: Requirements 1.4, 2.4, 6.3**

## Error Handling

### Error Categories

1. **User Errors**
   - Unsupported file format
   - No media loaded when attempting save
   - Invalid file paths

2. **Backend Errors**
   - FFmpeg execution failures
   - FFprobe metadata reading failures
   - Pillow image loading failures

3. **System Errors**
   - File I/O errors
   - Insufficient permissions
   - Missing external dependencies

### Error Handling Strategy

**User Errors:**
- Display informational QMessageBox
- Allow user to retry or cancel
- Do not crash application

**Backend Errors:**
- Raise custom exceptions (FFmpegError)
- Catch at UI boundary
- Display error details in QMessageBox
- Log full error for debugging

**System Errors:**
- Catch at top level
- Display critical error dialog
- Provide guidance for resolution
- Graceful degradation where possible

### Error Messages

All error messages should:
- Be clear and actionable
- Include relevant context (file path, operation)
- Avoid technical jargon for user-facing messages
- Include full details in logs

## Testing Strategy

### Unit Testing

#### Rust Backend Tests

Unit tests will verify specific examples and edge cases:

**Image Operations Tests (`src-tauri/src/media_editor/image_ops.rs`):**
- Loading various image formats
- Cropping with valid rectangles
- Cropping at image boundaries
- Handling invalid crop rectangles

**Video Operations Tests (`src-tauri/src/media_editor/video_ops.rs`):**
- Probing duration and resolution
- Trimming with valid time ranges
- Trimming at video boundaries
- Cropping with valid rectangles
- Combined trim and crop operations

#### Frontend Tests

**Coordinate Mapping Tests (`src/lib/utils/coordinates.test.ts`):**
- Mapping from widget to source coordinates
- Handling different aspect ratios
- Scaling calculations
- Boundary clamping

**Component Tests:**
- Crop overlay mouse interactions (Svelte Testing Library)
- Timeline slider constraints
- File dialog interactions
- Error message display

### Property-Based Testing

#### Rust Property Tests

Property-based tests will verify universal properties across all inputs using the **proptest** crate (already in dev-dependencies).

**Configuration:**
- Each property test SHALL run a minimum of 100 iterations
- Each test SHALL be tagged with a comment referencing the design document property
- Tag format: `// Feature: media-editor, Property {number}: {property_text}`

#### Frontend Property Tests

Property-based tests using **fast-check** library (already in package.json).

**Configuration:**
- Each property test SHALL run a minimum of 100 iterations
- Each test SHALL be tagged with a comment referencing the design document property
- Tag format: `// Feature: media-editor, Property {number}: {property_text}`

**Property Test Coverage:**

1. **Crop Coordinate Mapping (Property 1)**
   - Generate random crop rectangles in preview space
   - Verify mapped coordinates are within source bounds
   - Verify dimensions are positive

2. **Timeline Constraints (Property 2)**
   - Generate random slider positions
   - Verify start ≤ end always holds
   - Verify both are within [0, duration]

3. **Media Type Routing (Property 3)**
   - Generate random file extensions
   - Verify correct editor is selected
   - Verify unsupported extensions are rejected

4. **Export Without Crop (Property 4)**
   - Generate random media files
   - Export without crop
   - Verify output dimensions match input

5. **Crop Normalization (Property 5)**
   - Generate random rectangle pairs (opposite corners)
   - Verify normalized rectangles are identical

6. **FFmpeg Error Propagation (Property 6)**
   - Generate invalid FFmpeg commands
   - Verify FFmpegError is raised
   - Verify error message is captured

7. **Video Trim Duration (Property 7)**
   - Generate random trim points
   - Export and measure output duration
   - Verify duration matches expected within tolerance

8. **Coordinate Scaling Round-Trip (Property 8)**
   - Generate random source coordinates
   - Scale to preview and back
   - Verify result is within 1 pixel of original

### Integration Testing

Integration tests will verify end-to-end workflows:
- Load image → crop → export → verify output
- Load video → trim → export → verify output
- Load video → crop → trim → export → verify output
- Error handling flows

### Test Data

Tests will use:
- Generated test images (solid colors, patterns)
- Short generated test videos (FFmpeg test patterns)
- Real sample files for integration tests
- Edge cases (1x1 images, very long videos, unusual aspect ratios)

## Dependencies

### Required External Tools

- **FFmpeg**: Video processing (must be installed separately and in PATH)
- **FFprobe**: Video metadata extraction (included with FFmpeg)

### Rust Dependencies (Cargo.toml)

```toml
[dependencies]
# Existing dependencies...
image = "0.25"  # Already present
tokio = { version = "1", features = ["full"] }  # Already present

# No new dependencies needed - FFmpeg via subprocess
```

### Frontend Dependencies (package.json)

```json
{
  "devDependencies": {
    "fast-check": "^4.3.0"  // Already present for property-based testing
  }
}
```

### Installation

```bash
# Install FFmpeg (platform-specific)
# macOS: brew install ffmpeg
# Linux: apt-get install ffmpeg
# Windows: Download from ffmpeg.org and add to PATH

# Install frontend dependencies (already done)
pnpm install

# Rust dependencies are managed by Cargo
```

## Future Enhancements

The architecture supports future additions:

1. **Video Preview Playback**
   - Integrate QVideoWidget or mpv
   - Scrubbing on timeline
   - Preview crop in real-time

2. **Advanced Crop Features**
   - Aspect ratio locking
   - Rule of thirds guides
   - Preset crop ratios (16:9, 4:3, 1:1)

3. **Export Settings UI**
   - Dialog for selecting presets
   - Custom codec/quality settings
   - Batch export

4. **Undo/Redo**
   - Command pattern for operations
   - History stack

5. **Additional Backends**
   - ImageMagick as alternative to Pillow
   - GPU-accelerated encoding

6. **Zoom and Pan**
   - Zoom into preview for precise cropping
   - Pan around zoomed view
