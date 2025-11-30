# Design Document

## Overview

The Media Editor is a Python desktop application built with PySide6 (Qt for Python) that provides intuitive image and video editing capabilities. The application uses a clean architecture with separation between GUI components, business logic services, and backend processing tools (Pillow for images, FFmpeg for videos).

The design emphasizes:
- Unified crop overlay system shared between image and video editing
- Service-oriented architecture for media processing
- Configurable export settings for future extensibility
- Clear separation of concerns between UI, services, and models

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     GUI Layer (PySide6)                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ MainWindow   │  │ ImageEditor  │  │ VideoEditor  │  │
│  │              │  │   Widget     │  │   Widget     │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│         │                  │                  │          │
│         └──────────────────┴──────────────────┘          │
│                            │                             │
│                   ┌────────▼────────┐                    │
│                   │  CropOverlay    │                    │
│                   │    Widget       │                    │
│                   └─────────────────┘                    │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                    Models Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ MediaState   │  │  CropState   │  │  TrimState   │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                   Services Layer                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ImageService  │  │VideoService  │  │FFmpegService │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│              Backend Tools (External)                    │
│         Pillow (Images)    FFmpeg (Videos)               │
└─────────────────────────────────────────────────────────┘
```

### Directory Structure

```
media_editor/
  pyproject.toml
  media_editor/
    __init__.py
    main.py                # Application entry point
    
    models/
      __init__.py
      media_state.py       # State containers
      enums.py             # MediaType enum
    
    services/
      __init__.py
      image_service.py     # Image operations (Pillow)
      video_service.py     # Video operations (FFmpeg)
      ffmpeg_service.py    # FFmpeg command execution
    
    ui/
      __init__.py
      main_window.py       # Main application window
      image_editor.py      # Image editing widget
      video_editor.py      # Video editing widget
      crop_overlay.py      # Shared crop overlay
    
    config/
      __init__.py
      defaults.py          # Default settings
      presets.py           # Export presets
      schema.py            # Configuration schemas
    
    utils/
      __init__.py
      paths.py             # Path utilities
      math_utils.py        # Coordinate mapping
```

## Components and Interfaces

### Models

#### MediaState
Central state container that tracks the current media file and editing operations.

```python
@dataclass
class MediaState:
    path: Optional[Path]              # Current media file path
    media_type: Optional[MediaType]   # IMAGE or VIDEO
    crop: CropState                   # Crop rectangle state
    trim: TrimState                   # Video trim state
```

#### CropState
Stores crop rectangle in source coordinates.

```python
@dataclass
class CropState:
    rect: Optional[Rect]  # (x, y, width, height) in source coords
```

#### TrimState
Stores video trim points.

```python
@dataclass
class TrimState:
    start_sec: float      # Trim start time
    end_sec: float        # Trim end time
    duration_sec: float   # Total video duration
```

### Services

#### ImageService
Handles image loading and cropping using Pillow.

**Interface:**
- `load_image(path: Path) -> Image.Image`
- `crop_image(input_path: Path, output_path: Path, rect: Rect) -> None`

#### VideoService
Handles video operations using FFmpeg.

**Interface:**
- `probe_duration(input_path: Path) -> float`
- `probe_resolution(input_path: Path) -> tuple[int, int]`
- `trim_and_crop_video(input_path: Path, output_path: Path, start_sec: float, end_sec: float, crop_rect: Optional[Rect]) -> None`

#### FFmpegService
Low-level FFmpeg command execution.

**Interface:**
- `run_ffmpeg(args: List[str]) -> None`
  - Raises `FFmpegError` on non-zero exit code

### UI Components

#### MainWindow
Main application window with menu bar and stacked widget for switching between editors.

**Responsibilities:**
- File menu (Open, Save As)
- Route to appropriate editor based on media type
- Coordinate save operations

#### CropOverlay
Transparent overlay widget that handles crop rectangle drawing and coordinate mapping.

**Key Features:**
- Mouse drag to create crop rectangle
- Coordinate normalization (widget → preview → source)
- Shared between image and video editors

**Interface:**
- `set_source_resolution(w: int, h: int) -> None`
- `set_preview_rect(rect: QRect) -> None`
- `get_crop_rect_source() -> Optional[tuple[int, int, int, int]]`
- `clear_crop() -> None`
- `has_crop() -> bool`

#### ImageEditorWidget
Image preview and editing interface.

**Responsibilities:**
- Display image scaled to fit widget
- Integrate CropOverlay
- Calculate preview rectangle for coordinate mapping
- Export cropped image

#### VideoEditorWidget
Video preview and timeline interface.

**Responsibilities:**
- Display video preview frame
- Provide timeline sliders for trim points
- Integrate CropOverlay
- Export trimmed and cropped video

### Configuration

#### Defaults Module
Centralized default settings for export operations.

```python
DEFAULT_VIDEO_CODEC = "libx264"
DEFAULT_AUDIO_CODEC = "aac"
DEFAULT_IMAGE_FORMAT = "png"
DEFAULT_VIDEO_QUALITY = "23"  # CRF value
```

#### Presets Module
Named export presets for common use cases.

```python
PRESETS = {
    "high_quality_video": {...},
    "mobile_video": {...},
    "png_lossless": {...},
    "jpeg_small": {...}
}
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

Unit tests will verify specific examples and edge cases:

**Image Service Tests:**
- Loading various image formats
- Cropping with valid rectangles
- Cropping at image boundaries
- Handling invalid crop rectangles

**Video Service Tests:**
- Probing duration and resolution
- Trimming with valid time ranges
- Trimming at video boundaries
- Cropping with valid rectangles
- Combined trim and crop operations

**Coordinate Mapping Tests:**
- Mapping from widget to source coordinates
- Handling different aspect ratios
- Scaling calculations
- Boundary clamping

**UI Component Tests:**
- Crop overlay mouse interactions
- Timeline slider constraints
- File dialog interactions
- Error message display

### Property-Based Testing

Property-based tests will verify universal properties across all inputs using the **Hypothesis** library for Python.

**Configuration:**
- Each property test SHALL run a minimum of 100 iterations
- Each test SHALL be tagged with a comment referencing the design document property
- Tag format: `# Feature: media-editor, Property {number}: {property_text}`

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

### Python Dependencies

- **PySide6**: Qt bindings for GUI (>=6.0)
- **Pillow**: Image processing (>=9.0)
- **Hypothesis**: Property-based testing (>=6.0, dev dependency)
- **pytest**: Test framework (dev dependency)

### Installation

```bash
# Install FFmpeg (platform-specific)
# macOS: brew install ffmpeg
# Linux: apt-get install ffmpeg
# Windows: Download from ffmpeg.org

# Install Python dependencies
pip install PySide6 Pillow

# Install dev dependencies
pip install pytest hypothesis
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
