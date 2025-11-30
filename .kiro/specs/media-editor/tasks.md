# Implementation Plan

- [ ] 1. Set up project structure and dependencies
  - Create directory structure for models, services, ui, config, and utils modules
  - Create pyproject.toml with PySide6 and Pillow dependencies
  - Create __init__.py files for all modules
  - Set up pytest and Hypothesis for testing
  - _Requirements: All_

- [ ] 2. Implement core data models
  - [ ] 2.1 Create enums module with MediaType enum
    - Define IMAGE and VIDEO enum values
    - _Requirements: 4.2, 4.3_
  
  - [ ] 2.2 Create media state models
    - Implement CropState dataclass with optional rect field
    - Implement TrimState dataclass with start_sec, end_sec, duration_sec fields
    - Implement MediaState dataclass with path, media_type, crop, and trim fields
    - _Requirements: 1.3, 2.3, 3.2, 3.3_

- [ ] 3. Implement backend services
  - [ ] 3.1 Create FFmpeg service for command execution
    - Implement run_ffmpeg function that executes FFmpeg commands
    - Implement FFmpegError exception class
    - Handle non-zero exit codes and capture stderr
    - _Requirements: 8.1_
  
  - [ ] 3.2 Write property test for FFmpeg error handling
    - **Property 6: FFmpeg errors are propagated**
    - **Validates: Requirements 8.1**
  
  - [ ] 3.3 Create video service for video operations
    - Implement probe_duration function using ffprobe
    - Implement probe_resolution function using ffprobe
    - Implement trim_and_crop_video function with FFmpeg filters
    - Support optional crop_rect parameter
    - Use config defaults for codec and quality settings
    - _Requirements: 3.5, 4.5, 7.1, 7.2, 7.3_
  
  - [ ] 3.4 Write property test for video trim duration
    - **Property 7: Video trim produces correct duration**
    - **Validates: Requirements 3.5**
  
  - [ ] 3.5 Create image service for image operations
    - Implement load_image function using Pillow
    - Implement crop_image function with rectangle parameter
    - Use config defaults for image format
    - _Requirements: 1.5, 7.4_

- [ ] 4. Implement configuration system
  - [ ] 4.1 Create defaults module
    - Define DEFAULT_VIDEO_CODEC, DEFAULT_AUDIO_CODEC constants
    - Define DEFAULT_IMAGE_FORMAT, DEFAULT_VIDEO_QUALITY constants
    - _Requirements: 7.1, 7.2, 7.3, 7.4_
  
  - [ ] 4.2 Create presets module
    - Define PRESETS dictionary with high_quality_video preset
    - Define mobile_video, png_lossless, jpeg_small presets
    - _Requirements: 7.5_
  
  - [ ] 4.3 Create schema module
    - Define VideoExportSettings dataclass
    - Define ImageExportSettings dataclass
    - _Requirements: 7.5_

- [ ] 5. Implement coordinate mapping utilities
  - [ ] 5.1 Create math utilities module
    - Implement coordinate mapping from widget to preview coordinates
    - Implement coordinate mapping from preview to source coordinates
    - Implement clamping to valid bounds
    - Implement scale factor calculation
    - _Requirements: 1.4, 2.4, 6.2, 6.3_
  
  - [ ] 5.2 Write property test for coordinate mapping bounds
    - **Property 1: Crop coordinate mapping preserves bounds**
    - **Validates: Requirements 1.4, 2.4**
  
  - [ ] 5.3 Write property test for coordinate scaling round-trip
    - **Property 8: Coordinate scaling is invertible**
    - **Validates: Requirements 1.4, 2.4, 6.3**

- [ ] 6. Implement shared crop overlay widget
  - [ ] 6.1 Create CropOverlay widget class
    - Implement mouse press, move, and release event handlers
    - Implement crop rectangle drawing in paintEvent
    - Implement set_source_resolution method
    - Implement set_preview_rect method
    - Implement get_crop_rect_source method with coordinate mapping
    - Implement clear_crop and has_crop methods
    - _Requirements: 1.2, 1.3, 2.2, 2.3, 6.2, 6.3_
  
  - [ ] 6.2 Write property test for crop normalization
    - **Property 5: Crop rectangle normalization is consistent**
    - **Validates: Requirements 1.2, 2.2, 6.2**
  
  - [ ] 6.3 Write unit tests for crop overlay
    - Test mouse drag creates rectangle
    - Test rectangle persists after mouse release
    - Test clear_crop removes rectangle
    - _Requirements: 1.2, 1.3, 6.5_

- [ ] 7. Implement image editor widget
  - [ ] 7.1 Create ImageEditorWidget class
    - Implement load_image method to display image scaled to fit
    - Calculate and set preview rectangle for crop overlay
    - Integrate CropOverlay as child widget
    - Implement paintEvent to draw scaled image
    - _Requirements: 1.1, 1.2_
  
  - [ ] 7.2 Implement image export functionality
    - Implement export_image method
    - Get crop rectangle from overlay in source coordinates
    - Call image_service.crop_image if crop exists
    - Copy file directly if no crop
    - _Requirements: 1.4, 1.5_
  
  - [ ] 7.3 Write property test for export without crop
    - **Property 4: Export without crop preserves original dimensions**
    - **Validates: Requirements 1.5**

- [ ] 8. Implement video editor widget
  - [ ] 8.1 Create VideoEditorWidget class with timeline
    - Create QSlider widgets for start and end handles
    - Implement load_video method to probe duration and resolution
    - Display timeline label with current trim times
    - Integrate CropOverlay as child widget
    - _Requirements: 2.1, 3.1, 3.2, 3.3_
  
  - [ ] 8.2 Implement timeline constraint logic
    - Connect slider valueChanged signals to update handler
    - Implement constraint: start handle cannot exceed end handle
    - Update MediaState trim values from slider positions
    - _Requirements: 3.2, 3.3, 3.4_
  
  - [ ] 8.3 Write property test for timeline constraints
    - **Property 2: Timeline trim constraints are maintained**
    - **Validates: Requirements 3.2, 3.3, 3.4**
  
  - [ ] 8.4 Implement video export functionality
    - Implement export_video method
    - Get crop rectangle from overlay in source coordinates
    - Get trim times from MediaState
    - Call video_service.trim_and_crop_video
    - _Requirements: 2.4, 2.5, 3.5_

- [ ] 9. Implement main window and routing
  - [ ] 9.1 Create MainWindow class with menu bar
    - Create File menu with Open and Save As actions
    - Create QStackedWidget for editor switching
    - Initialize MediaState shared across editors
    - Add ImageEditorWidget and VideoEditorWidget to stack
    - _Requirements: 4.1, 5.1_
  
  - [ ] 9.2 Implement file opening logic
    - Implement on_open slot to show file dialog
    - Determine media type from file extension
    - Route to image editor for png, jpg, jpeg, bmp, gif
    - Route to video editor for mp4, mov, mkv
    - Display error for unsupported extensions
    - _Requirements: 4.1, 4.2, 4.3, 4.4_
  
  - [ ] 9.3 Write property test for media type routing
    - **Property 3: Media type determines editor routing**
    - **Validates: Requirements 4.2, 4.3**
  
  - [ ] 9.4 Implement file saving logic
    - Implement on_save_as slot to show save dialog
    - Check if media is loaded, show info message if not
    - Delegate to appropriate editor export method based on media type
    - Catch and display exceptions in error dialog
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 8.2, 8.3_
  
  - [ ] 9.5 Write unit tests for error handling
    - Test save without loaded media shows info message
    - Test export errors display error dialog
    - _Requirements: 5.2, 5.5, 8.2, 8.3_

- [ ] 10. Implement application entry point
  - [ ] 10.1 Create main.py with application entry point
    - Create QApplication instance
    - Create and show MainWindow
    - Execute application event loop
    - _Requirements: All_

- [ ] 11. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 12. Create project packaging configuration
  - [ ] 12.1 Create pyproject.toml with metadata
    - Define project name, version, description
    - List PySide6 and Pillow as dependencies
    - List pytest and Hypothesis as dev dependencies
    - Define entry point for main function
    - _Requirements: All_
  
  - [ ] 12.2 Create README with installation instructions
    - Document FFmpeg installation requirement
    - Document Python dependency installation
    - Document how to run the application
    - _Requirements: All_

- [ ] 13. Write integration tests
  - Test complete workflow: load image → crop → export → verify
  - Test complete workflow: load video → trim → export → verify
  - Test complete workflow: load video → crop → trim → export → verify
  - Test error handling flows
  - _Requirements: All_
