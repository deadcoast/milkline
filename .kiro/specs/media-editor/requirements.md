# Requirements Document

## Introduction

The Media Editor is a desktop application that provides simple, intuitive tools for cropping and trimming both images and videos. The application enables users to dynamically adjust canvas dimensions and trim video timelines through a graphical interface, with support for importing and exporting media files in common formats.

## Glossary

- **Media Editor**: The desktop application system that provides image and video editing capabilities
- **Canvas**: The visible area of an image or video that can be cropped or resized
- **Crop Rectangle**: A user-defined rectangular selection area used to specify the desired output dimensions
- **Timeline**: A visual representation of video duration with controls for selecting start and end points
- **Source Coordinates**: The pixel coordinate system of the original media file
- **Preview Coordinates**: The pixel coordinate system of the displayed preview in the application window
- **FFmpeg**: The backend video processing library used for video operations
- **Pillow**: The backend image processing library used for image operations

## Requirements

### Requirement 1

**User Story:** As a user, I want to crop images by dragging a selection rectangle, so that I can remove unwanted portions of the image.

#### Acceptance Criteria

1. WHEN a user opens an image file, THE Media Editor SHALL display the image in a preview area
2. WHEN a user clicks and drags on the image preview, THE Media Editor SHALL draw a visible crop rectangle
3. WHEN a user releases the mouse button, THE Media Editor SHALL maintain the crop rectangle selection
4. WHEN a user exports a cropped image, THE Media Editor SHALL convert the preview coordinates to source coordinates accurately
5. WHEN a user exports a cropped image, THE Media Editor SHALL save only the selected rectangular area to the output file

### Requirement 2

**User Story:** As a user, I want to crop videos by dragging a selection rectangle, so that I can remove black bars or unwanted portions of the video frame.

#### Acceptance Criteria

1. WHEN a user opens a video file, THE Media Editor SHALL display a preview frame from the video
2. WHEN a user clicks and drags on the video preview, THE Media Editor SHALL draw a visible crop rectangle
3. WHEN a user releases the mouse button, THE Media Editor SHALL maintain the crop rectangle selection
4. WHEN a user exports a cropped video, THE Media Editor SHALL convert the preview coordinates to source video coordinates accurately
5. WHEN a user exports a cropped video, THE Media Editor SHALL apply the crop filter to all frames in the output file

### Requirement 3

**User Story:** As a user, I want to trim videos using a timeline interface, so that I can extract specific segments from longer videos.

#### Acceptance Criteria

1. WHEN a user opens a video file, THE Media Editor SHALL display a timeline representing the full video duration
2. WHEN a user drags the start handle on the timeline, THE Media Editor SHALL update the trim start time
3. WHEN a user drags the end handle on the timeline, THE Media Editor SHALL update the trim end time
4. WHEN a user drags the start handle past the end handle, THE Media Editor SHALL constrain the start handle to not exceed the end handle position
5. WHEN a user exports a trimmed video, THE Media Editor SHALL output only the frames between the start and end times

### Requirement 4

**User Story:** As a user, I want to open image and video files, so that I can edit them in the application.

#### Acceptance Criteria

1. WHEN a user selects "Open" from the file menu, THE Media Editor SHALL display a file selection dialog
2. WHEN a user selects an image file with extension png, jpg, jpeg, bmp, or gif, THE Media Editor SHALL load the image into the image editor view
3. WHEN a user selects a video file with extension mp4, mov, or mkv, THE Media Editor SHALL load the video into the video editor view
4. WHEN a user selects a file with an unsupported extension, THE Media Editor SHALL display an error message and prevent loading
5. WHEN a video file is loaded, THE Media Editor SHALL probe the video duration and resolution using FFmpeg

### Requirement 5

**User Story:** As a user, I want to save edited media files, so that I can preserve my edits.

#### Acceptance Criteria

1. WHEN a user selects "Save As" from the file menu, THE Media Editor SHALL display a file save dialog
2. WHEN no media file is loaded and a user selects "Save As", THE Media Editor SHALL display an informational message
3. WHEN a user saves an edited image, THE Media Editor SHALL write the cropped image to the specified output path
4. WHEN a user saves an edited video, THE Media Editor SHALL write the trimmed and cropped video to the specified output path
5. WHEN an error occurs during export, THE Media Editor SHALL display an error message with details

### Requirement 6

**User Story:** As a user, I want the crop rectangle to work consistently for both images and videos, so that I have a familiar editing experience.

#### Acceptance Criteria

1. WHEN a user drags a crop rectangle on any media type, THE Media Editor SHALL use the same overlay component
2. WHEN a user drags a crop rectangle, THE Media Editor SHALL normalize coordinates to the preview area
3. WHEN the Media Editor exports media, THE Media Editor SHALL map normalized coordinates to source media coordinates
4. WHEN a user resizes the application window, THE Media Editor SHALL maintain the crop rectangle proportions relative to the preview
5. WHEN a user clears a crop selection, THE Media Editor SHALL remove the visible crop rectangle

### Requirement 7

**User Story:** As a developer, I want the application to use configurable export settings, so that I can easily adjust quality and codec parameters.

#### Acceptance Criteria

1. WHEN the Media Editor exports a video, THE Media Editor SHALL use the configured video codec from the defaults module
2. WHEN the Media Editor exports a video, THE Media Editor SHALL use the configured audio codec from the defaults module
3. WHEN the Media Editor exports a video, THE Media Editor SHALL use the configured quality settings from the defaults module
4. WHEN the Media Editor exports an image, THE Media Editor SHALL use the configured image format from the defaults module
5. WHERE export presets are defined, THE Media Editor SHALL apply preset parameters to the export operation

### Requirement 8

**User Story:** As a user, I want the application to handle errors gracefully, so that I understand what went wrong when operations fail.

#### Acceptance Criteria

1. WHEN FFmpeg returns a non-zero exit code, THE Media Editor SHALL raise an FFmpegError with the error message
2. WHEN a video export fails, THE Media Editor SHALL display the error message to the user
3. WHEN an image export fails, THE Media Editor SHALL display the error message to the user
4. WHEN FFprobe fails to read video metadata, THE Media Editor SHALL raise a runtime error with details
5. WHEN a user attempts to export without loading media, THE Media Editor SHALL display an informational message
