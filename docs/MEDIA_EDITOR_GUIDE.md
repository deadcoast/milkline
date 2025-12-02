# Media Editor User Guide

The milk media editor provides simple, intuitive tools for cropping images and videos, and trimming video clips. This guide will walk you through all the features and how to use them.

## Table of Contents

- [Getting Started](#getting-started)
- [Opening Media Files](#opening-media-files)
- [Cropping Images](#cropping-images)
- [Cropping Videos](#cropping-videos)
- [Trimming Videos](#trimming-videos)
- [Combining Crop and Trim](#combining-crop-and-trim)
- [Export Options](#export-options)
- [Supported Formats](#supported-formats)
- [Troubleshooting](#troubleshooting)

## Getting Started

### Prerequisites

Before using the media editor, ensure you have **FFmpeg** installed on your system:

**macOS:**
```bash
brew install ffmpeg
```

**Windows:**
1. Download FFmpeg from [ffmpeg.org](https://ffmpeg.org/download.html)
2. Extract the archive
3. Add the `bin` folder to your system PATH

**Linux:**
```bash
# Debian/Ubuntu
sudo apt-get install ffmpeg

# Fedora
sudo dnf install ffmpeg

# Arch
sudo pacman -S ffmpeg
```

To verify FFmpeg is installed correctly:
```bash
ffmpeg -version
```

### Accessing the Media Editor

1. Launch the milk application
2. Navigate to the Media Editor from the main menu or use the keyboard shortcut
3. The media editor window will open with a file menu

## Opening Media Files

### Using the File Menu

1. Click **"Open"** in the file menu
2. Browse to your media file
3. Select the file and click **"Open"**

The media editor will automatically detect the file type and load the appropriate editor:
- **Image files** → Image Editor
- **Video files** → Video Editor

### Supported File Extensions

**Images:** `.png`, `.jpg`, `.jpeg`, `.bmp`, `.gif`  
**Videos:** `.mp4`, `.mov`, `.mkv`

If you try to open an unsupported file type, you'll see an error message indicating the file format is not supported.

## Cropping Images

The image editor allows you to select and crop a rectangular area from your image.

### How to Crop an Image

1. **Open an image file** using the file menu
2. The image will display in the preview area, scaled to fit the window
3. **Click and drag** on the image to draw a crop rectangle:
   - Click where you want one corner of the crop area
   - Hold the mouse button and drag to the opposite corner
   - Release the mouse button to complete the selection
4. The crop rectangle will remain visible, showing your selection
5. You can **redraw the rectangle** at any time by clicking and dragging again
6. Click **"Save As"** to export the cropped image

### Tips for Image Cropping

- The crop rectangle can be drawn in any direction (top-left to bottom-right, or vice versa)
- The preview automatically scales to fit your window, but the export uses the original image resolution
- If you save without drawing a crop rectangle, the original image will be copied unchanged

## Cropping Videos

Video cropping works the same way as image cropping, allowing you to remove black bars or unwanted portions from video frames.

### How to Crop a Video

1. **Open a video file** using the file menu
2. A preview frame from the video will display
3. **Click and drag** on the preview to draw a crop rectangle (same as images)
4. The crop will be applied to **all frames** in the video when you export
5. Click **"Save As"** to export the cropped video

### Important Notes

- Video cropping processes the entire video, which may take time depending on video length and resolution
- The crop is applied to every frame in the video
- Audio is preserved in the cropped video
- If you don't draw a crop rectangle, the video dimensions remain unchanged

## Trimming Videos

The video editor includes a timeline interface for selecting specific segments of a video.

### How to Trim a Video

1. **Open a video file** using the file menu
2. Below the video preview, you'll see a **timeline bar** representing the full video duration
3. The timeline has two handles:
   - **Start handle** (left) - Sets where the output video begins
   - **End handle** (right) - Sets where the output video ends
4. **Drag the handles** to select your desired segment:
   - Drag the start handle to set the beginning of your clip
   - Drag the end handle to set the end of your clip
   - The current trim times are displayed on the timeline
5. Click **"Save As"** to export the trimmed video

### Timeline Constraints

- The start time cannot exceed the end time (the handles won't cross)
- Both handles are constrained to the video duration
- The timeline displays the current start and end times in seconds

### Tips for Video Trimming

- Drag handles slowly for precise control
- The exported video will contain only the frames between the start and end times
- If you don't adjust the handles, the full video duration is exported

## Combining Crop and Trim

You can apply both crop and trim operations to a video simultaneously.

### How to Crop and Trim Together

1. **Open a video file**
2. **Draw a crop rectangle** on the video preview (see [Cropping Videos](#cropping-videos))
3. **Adjust the timeline handles** to select your desired segment (see [Trimming Videos](#trimming-videos))
4. Click **"Save As"** to export

The exported video will:
- Contain only the frames between the start and end times (trim)
- Have the crop filter applied to all frames (crop)
- Preserve audio from the trimmed segment

## Export Options

### Saving Your Edits

1. After making your edits (crop, trim, or both), click **"Save As"** in the file menu
2. Choose a location and filename for the output file
3. Click **"Save"**
4. The application will process your edits and save the result

### Export Settings

The media editor uses optimized default settings for exports:

**Video Exports:**
- **Video Codec:** H.264 (libx264)
- **Audio Codec:** AAC
- **Quality:** CRF 23 (high quality, reasonable file size)

**Image Exports:**
- **Format:** PNG (lossless)
- Preserves original image quality

### Processing Time

- **Image crops:** Nearly instant
- **Video operations:** Depends on video length, resolution, and your system performance
  - Short clips (< 1 minute): A few seconds
  - Longer videos: May take several minutes
  - The application will show an error if processing fails

### What Happens If You Don't Make Edits?

- **Images:** If you save without drawing a crop rectangle, the original image is copied unchanged
- **Videos:** If you save without cropping or trimming, the video is re-encoded with default settings

## Supported Formats

### Input Formats

**Images:**
- PNG (`.png`)
- JPEG (`.jpg`, `.jpeg`)
- BMP (`.bmp`)
- GIF (`.gif`)

**Videos:**
- MP4 (`.mp4`)
- MOV (`.mov`)
- MKV (`.mkv`)

### Output Formats

**Images:** PNG (lossless, high quality)  
**Videos:** MP4 with H.264 video and AAC audio

## Troubleshooting

### "FFmpeg not found" or video operations fail

**Problem:** FFmpeg is not installed or not in your system PATH.

**Solution:**
1. Install FFmpeg (see [Prerequisites](#prerequisites))
2. Verify installation: `ffmpeg -version`
3. Restart the milk application after installing FFmpeg

### "Unsupported file format" error

**Problem:** You're trying to open a file type that isn't supported.

**Solution:**
- Check the [Supported Formats](#supported-formats) section
- Convert your file to a supported format using another tool
- For videos, MP4 is the most universally supported format

### Video export takes a very long time

**Problem:** Video processing is CPU-intensive and depends on video length and resolution.

**Solution:**
- This is normal behavior for video processing
- Longer videos and higher resolutions take more time
- Consider trimming the video to a shorter segment first
- Close other applications to free up system resources

### Exported video has no audio

**Problem:** The source video may not have had audio, or there was an error during processing.

**Solution:**
- Check that the original video has audio
- Try exporting again
- Check the error messages for details

### Crop rectangle doesn't appear where I clicked

**Problem:** The preview is scaled to fit the window, which can affect precision.

**Solution:**
- The coordinates are automatically mapped to the source resolution
- Try maximizing the window for a larger preview
- The export will use the correct source coordinates

### "No media loaded" when trying to save

**Problem:** You haven't opened a media file yet.

**Solution:**
1. Click **"Open"** in the file menu
2. Select a media file
3. Make your edits
4. Then click **"Save As"**

## Keyboard Shortcuts

Currently, the media editor uses mouse-based interactions. Keyboard shortcuts may be added in future versions.

## Tips and Best Practices

1. **Work with copies:** Always keep your original files. The media editor creates new files, but it's good practice to maintain backups.

2. **Preview scaling:** The preview is scaled to fit your window. Maximize the window for better precision when drawing crop rectangles.

3. **Video quality:** The default export settings (CRF 23) provide a good balance between quality and file size. The output quality will be similar to the input.

4. **Batch operations:** Currently, the media editor processes one file at a time. For batch operations, you'll need to process each file individually.

5. **File naming:** Use descriptive filenames when saving (e.g., `video_cropped.mp4`, `image_trimmed_0-30s.mp4`) to keep track of your edits.

## Future Enhancements

Planned features for future versions:
- Real-time video preview playback
- Aspect ratio locking for crops
- Custom export quality settings
- Batch processing
- Undo/redo functionality
- Zoom and pan for precise cropping

## Getting Help

If you encounter issues not covered in this guide:
1. Check the [Troubleshooting](#troubleshooting) section
2. Verify FFmpeg is installed correctly
3. Check the application logs for error details
4. Report issues on the project's issue tracker

---

**Happy editing!** 🎬✂️
