# Implementation Plan

- [x] 1. Set up Rust backend module structure
  - Create `src-tauri/src/media_editor/` directory
  - Create `mod.rs`, `image_ops.rs`, `video_ops.rs`, `types.rs`, `config.rs` files
  - Register media_editor module in `src-tauri/src/lib.rs`
  - _Requirements: All_

- [x] 2. Implement Rust types and configuration
  - [x] 2.1 Create shared types module
    - Define `CropRect` struct with Serialize/Deserialize
    - Define `VideoMetadata` struct with duration, width, height
    - Define `ExportConfig` struct for codec and quality settings
    - _Requirements: 1.4, 2.4, 4.5_
  
  - [x] 2.2 Create configuration module
    - Define `ExportDefaults` struct with default codec/quality values
    - Define `ExportPreset` struct for named presets
    - Create constants for default config and preset array
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

- [x] 3. Implement Rust image operations
  - [x] 3.1 Create image operations module
    - Implement `crop_image` function using `image` crate
    - Load image, apply crop rectangle, save to output path
    - Handle errors and return Result<(), String>
    - _Requirements: 1.5, 7.4_
  
  - [x] 3.2 Create Tauri command for image cropping
    - Implement `crop_image` Tauri command
    - Accept input_path, output_path, crop_rect parameters
    - Call image operations function
    - _Requirements: 1.4, 1.5_
  
  - [x] 3.3 Write unit tests for image operations
    - Test cropping with valid rectangles
    - Test cropping at image boundaries
    - Test handling invalid crop rectangles
    - _Requirements: 1.5_
  
  - [x] 3.4 Write property test for export without crop
    - **Property 4: Export without crop preserves original dimensions**
    - **Validates: Requirements 1.5**

- [x] 4. Implement Rust video operations
  - [x] 4.1 Create video metadata probing function
    - Implement `probe_video_metadata` using FFprobe subprocess
    - Parse JSON output for duration, width, height
    - Handle FFprobe errors
    - _Requirements: 4.5_
  
  - [x] 4.2 Create Tauri command for video metadata
    - Implement `probe_video_metadata` Tauri command
    - Accept path parameter
    - Return VideoMetadata struct
    - _Requirements: 4.5_
  
  - [x] 4.3 Create video trim and crop function
    - Implement `trim_and_crop_video` using FFmpeg subprocess
    - Build FFmpeg command with trim times and optional crop filter
    - Use config for codec and quality settings
    - Handle FFmpeg errors and capture stderr
    - _Requirements: 2.5, 3.5, 7.1, 7.2, 7.3, 8.1_
  
  - [x] 4.4 Create Tauri command for video processing
    - Implement `trim_and_crop_video` Tauri command
    - Accept input_path, output_path, start_sec, end_sec, crop_rect, config
    - Call video operations function
    - _Requirements: 2.4, 2.5, 3.5_
  
  - [x] 4.5 Write unit tests for video operations
    - Test probing video metadata
    - Test trimming with valid time ranges
    - Test cropping with valid rectangles
    - Test combined trim and crop
    - _Requirements: 3.5, 4.5_
  
  - [x] 4.6 Write property test for FFmpeg error handling
    - **Property 6: FFmpeg errors are propagated**
    - **Validates: Requirements 8.1**
  
  - [x] 4.7 Write property test for video trim duration
    - **Property 7: Video trim produces correct duration**
    - **Validates: Requirements 3.5**

- [x] 5. Register Tauri commands in main.rs
  - [x] 5.1 Import media_editor module commands
    - Add use statements for all Tauri commands
    - Register commands in tauri::Builder invoke_handler
    - _Requirements: All_

- [x] 6. Create frontend coordinate utilities
  - [x] 6.1 Create coordinates utility module
    - Implement `widgetToPreviewCoords` function
    - Implement `previewToSourceCoords` function
    - Implement `clampToSourceBounds` function
    - Implement `calculateScaleFactor` function
    - _Requirements: 1.4, 2.4, 6.2, 6.3_
  
  - [x] 6.2 Write unit tests for coordinate mapping
    - Test mapping from widget to source coordinates
    - Test handling different aspect ratios
    - Test scaling calculations
    - Test boundary clamping
    - _Requirements: 1.4, 2.4_
  
  - [x] 6.3 Write property test for coordinate mapping bounds
    - **Property 1: Crop coordinate mapping preserves bounds**
    - **Validates: Requirements 1.4, 2.4**
  
  - [x] 6.4 Write property test for coordinate scaling round-trip
    - **Property 8: Coordinate scaling is invertible**
    - **Validates: Requirements 1.4, 2.4, 6.3**

- [x] 7. Create media editor Svelte store
  - [x] 7.1 Create mediaEditor store
    - Define MediaEditorState interface
    - Create writable store with initial state
    - Implement `loadMedia` action to set file path and type
    - Implement `setCrop` action to update crop rectangle
    - Implement `setTrim` action to update trim times
    - Implement `clearCrop` action
    - Implement `reset` action
    - _Requirements: 1.3, 2.3, 3.2, 3.3_
  
  - [x] 7.2 Write unit tests for store
    - Test state updates for each action
    - Test reset clears all state
    - _Requirements: 1.3, 2.3_

- [x] 8. Implement CropOverlay Svelte component
  - [x] 8.1 Create CropOverlay component
    - Accept sourceWidth, sourceHeight, previewWidth, previewHeight props
    - Implement mouse down, move, up event handlers
    - Draw crop rectangle using SVG overlay
    - Emit cropChange event with source coordinates
    - Implement clear method
    - _Requirements: 1.2, 1.3, 2.2, 2.3, 6.2, 6.3_
  
  - [x] 8.2 Write component tests for CropOverlay
    - Test mouse drag creates rectangle
    - Test rectangle persists after mouse release
    - Test clear removes rectangle
    - _Requirements: 1.2, 1.3, 6.5_
  
  - [x] 8.3 Write property test for crop normalization
    - **Property 5: Crop rectangle normalization is consistent**
    - **Validates: Requirements 1.2, 2.2, 6.2**

- [x] 9. Implement Timeline Svelte component
  - [x] 9.1 Create Timeline component
    - Accept duration, startTime, endTime props
    - Render timeline bar with start and end handles
    - Implement drag handlers for both handles
    - Constrain start handle to not exceed end handle
    - Emit trimChange event with {startSec, endSec}
    - Display current trim times as label
    - _Requirements: 3.1, 3.2, 3.3, 3.4_
  
  - [x] 9.2 Write component tests for Timeline
    - Test handle dragging updates times
    - Test start handle constraint
    - _Requirements: 3.2, 3.3, 3.4_
  
  - [x] 9.3 Write property test for timeline constraints
    - **Property 2: Timeline trim constraints are maintained**
    - **Validates: Requirements 3.2, 3.3, 3.4**

- [x] 10. Implement ImageEditor Svelte component
  - [x] 10.1 Create ImageEditor component
    - Accept filePath prop
    - Load and display image scaled to fit container
    - Calculate preview dimensions
    - Integrate CropOverlay component
    - Handle cropChange events to update store
    - _Requirements: 1.1, 1.2_
  
  - [x] 10.2 Implement image export functionality
    - Create export method that calls crop_image Tauri command
    - Get crop rectangle from store
    - Handle case where no crop exists (copy file)
    - Display error messages on failure
    - _Requirements: 1.4, 1.5, 5.3, 8.3_

- [x] 11. Implement VideoEditor Svelte component
  - [x] 11.1 Create VideoEditor component
    - Accept filePath prop
    - Call probe_video_metadata Tauri command on mount
    - Display video preview frame (placeholder for now)
    - Calculate preview dimensions
    - Integrate CropOverlay component
    - Integrate Timeline component
    - Handle cropChange and trimChange events to update store
    - _Requirements: 2.1, 3.1, 3.2, 3.3_
  
  - [x] 11.2 Implement video export functionality
    - Create export method that calls trim_and_crop_video Tauri command
    - Get crop rectangle and trim times from store
    - Pass export config from defaults
    - Display error messages on failure
    - _Requirements: 2.4, 2.5, 3.5, 5.4, 8.2_

- [x] 12. Implement MediaEditorWindow Svelte component
  - [x] 12.1 Create MediaEditorWindow component
    - Create file menu with Open and Save As buttons
    - Implement file open dialog using Tauri dialog API
    - Determine media type from file extension
    - Route to ImageEditor for png, jpg, jpeg, bmp, gif
    - Route to VideoEditor for mp4, mov, mkv
    - Display error for unsupported extensions
    - _Requirements: 4.1, 4.2, 4.3, 4.4_
  
  - [x] 12.2 Implement file save logic
    - Implement save dialog using Tauri dialog API
    - Check if media is loaded, show info message if not
    - Delegate to appropriate editor export method based on media type
    - Display error messages on failure
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 8.2, 8.3_
  
  - [x] 12.3 Write property test for media type routing
    - **Property 3: Media type determines editor routing**
    - **Validates: Requirements 4.2, 4.3**
  
  - [x] 12.4 Write unit tests for error handling
    - Test save without loaded media shows info message
    - Test export errors display error dialog
    - _Requirements: 5.2, 5.5, 8.2, 8.3_

- [x] 13. Create media editor route
  - [x] 13.1 Create route page
    - Create `src/routes/media-editor/+page.svelte`
    - Import and render MediaEditorWindow component
    - Add route to navigation if needed
    - _Requirements: All_

- [x] 14. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 15. Write integration tests
  - Test complete workflow: load image → crop → export → verify (Rust)
  - Test complete workflow: load video → trim → export → verify (Rust)
  - Test complete workflow: load video → crop → trim → export → verify (Rust)
  - Test error handling flows (Rust + Frontend)
  - Test Tauri command invocations from frontend (E2E)
  - _Requirements: All_

- [x] 16. Update documentation
  - [x] 16.1 Update README with media editor feature
    - Document FFmpeg installation requirement
    - Document media editor capabilities
    - Document supported formats
    - _Requirements: All_
  
  - [x] 16.2 Create media editor user guide
    - Document how to open media files
    - Document how to crop images and videos
    - Document how to trim videos
    - Document export options
    - _Requirements: All_
