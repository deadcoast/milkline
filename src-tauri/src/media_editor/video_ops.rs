// Video operations module
// This module contains video trimming, cropping, and metadata extraction functions

use crate::media_editor::types::{CropRect, VideoMetadata, ExportConfig};
use std::process::Command;
use serde_json::Value;

/// Probe video metadata using FFprobe
/// 
/// Uses FFprobe to extract duration, width, and height from a video file.
/// Returns VideoMetadata on success, or an error string on failure.
pub fn probe_video_metadata(path: &str) -> Result<VideoMetadata, String> {
    // Run FFprobe to get video metadata in JSON format
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=width,height,duration",
            "-show_entries", "format=duration",
            "-of", "json",
            path,
        ])
        .output()
        .map_err(|e| format!("Failed to execute FFprobe: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FFprobe failed: {}", stderr));
    }

    // Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse FFprobe JSON output: {}", e))?;

    // Extract width and height from stream
    let streams = json["streams"]
        .as_array()
        .ok_or_else(|| "No streams found in video".to_string())?;
    
    if streams.is_empty() {
        return Err("No video streams found".to_string());
    }

    let stream = &streams[0];
    let width = stream["width"]
        .as_u64()
        .ok_or_else(|| "Width not found in stream".to_string())? as u32;
    let height = stream["height"]
        .as_u64()
        .ok_or_else(|| "Height not found in stream".to_string())? as u32;

    // Try to get duration from stream first, then from format
    let duration_sec = if let Some(duration) = stream["duration"].as_str() {
        duration.parse::<f64>()
            .map_err(|e| format!("Failed to parse stream duration: {}", e))?
    } else if let Some(duration) = json["format"]["duration"].as_str() {
        duration.parse::<f64>()
            .map_err(|e| format!("Failed to parse format duration: {}", e))?
    } else {
        return Err("Duration not found in video metadata".to_string());
    };

    Ok(VideoMetadata {
        duration_sec,
        width,
        height,
    })
}

/// Tauri command to probe video metadata
#[tauri::command]
pub async fn probe_video_metadata_command(path: String) -> Result<VideoMetadata, String> {
    probe_video_metadata(&path)
}

/// Trim and optionally crop a video using FFmpeg
/// 
/// Uses FFmpeg to trim video between start_sec and end_sec, and optionally apply
/// a crop filter. Uses the provided ExportConfig for codec and quality settings.
pub fn trim_and_crop_video(
    input_path: &str,
    output_path: &str,
    start_sec: f64,
    end_sec: f64,
    crop_rect: Option<CropRect>,
    config: &ExportConfig,
) -> Result<(), String> {
    // For accurate trimming:
    // 1. Use -ss after -i for frame-accurate seeking (slower but precise)
    // 2. Use -t for duration instead of -to
    // 3. Add -avoid_negative_ts make_zero for timestamp handling
    let duration = end_sec - start_sec;
    
    let mut args = vec![
        "-y".to_string(), // Overwrite output file
        "-i".to_string(),
        input_path.to_string(),
        "-ss".to_string(),
        start_sec.to_string(),
        "-t".to_string(),
        duration.to_string(),
        "-avoid_negative_ts".to_string(),
        "make_zero".to_string(),
    ];

    // Add crop filter if provided
    if let Some(crop) = crop_rect {
        let crop_filter = format!(
            "crop={}:{}:{}:{}",
            crop.width, crop.height, crop.x, crop.y
        );
        args.push("-vf".to_string());
        args.push(crop_filter);
    }

    // Add codec and quality settings
    args.push("-c:v".to_string());
    args.push(config.video_codec.clone());
    args.push("-c:a".to_string());
    args.push(config.audio_codec.clone());
    args.push("-crf".to_string());
    args.push(config.quality.clone());

    args.push(output_path.to_string());

    // Execute FFmpeg
    let output = Command::new("ffmpeg")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute FFmpeg: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FFmpeg failed: {}", stderr));
    }

    Ok(())
}

/// Tauri command to trim and crop video
#[tauri::command]
pub async fn trim_and_crop_video_command(
    input_path: String,
    output_path: String,
    start_sec: f64,
    end_sec: f64,
    crop_rect: Option<CropRect>,
    config: ExportConfig,
) -> Result<(), String> {
    trim_and_crop_video(&input_path, &output_path, start_sec, end_sec, crop_rect, &config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command as StdCommand;
    use tempfile::TempDir;

    /// Helper function to create a test video file
    fn create_test_video(path: &str, duration_sec: f64, width: u32, height: u32) -> Result<(), String> {
        // Use 30 fps for better granularity in trimming tests
        // Also set keyframe interval to 1 for frame-accurate seeking
        let output = StdCommand::new("ffmpeg")
            .args([
                "-y",
                "-f", "lavfi",
                "-i", &format!("testsrc=duration={}:size={}x{}:rate=30", duration_sec, width, height),
                "-pix_fmt", "yuv420p",
                "-g", "1", // Set keyframe interval to 1 (every frame is a keyframe)
                path,
            ])
            .output()
            .map_err(|e| format!("Failed to create test video: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("FFmpeg test video creation failed: {}", stderr));
        }

        Ok(())
    }

    #[test]
    fn test_probe_video_metadata() {
        let temp_dir = TempDir::new().unwrap();
        let video_path = temp_dir.path().join("test.mp4");
        let video_path_str = video_path.to_str().unwrap();

        // Create a test video: 5 seconds, 320x240
        create_test_video(video_path_str, 5.0, 320, 240).unwrap();

        // Probe metadata
        let metadata = probe_video_metadata(video_path_str).unwrap();

        // Verify dimensions
        assert_eq!(metadata.width, 320);
        assert_eq!(metadata.height, 240);
        
        // Verify duration (allow small tolerance)
        assert!((metadata.duration_sec - 5.0).abs() < 0.5);
    }

    #[test]
    fn test_trim_video_with_valid_time_range() {
        let temp_dir = TempDir::new().unwrap();
        let input_path = temp_dir.path().join("input.mp4");
        let output_path = temp_dir.path().join("output.mp4");
        let input_path_str = input_path.to_str().unwrap();
        let output_path_str = output_path.to_str().unwrap();

        // Create a 10 second test video
        create_test_video(input_path_str, 10.0, 320, 240).unwrap();

        // Trim from 2 to 6 seconds
        let config = ExportConfig {
            video_codec: "libx264".to_string(),
            audio_codec: "aac".to_string(),
            quality: "23".to_string(),
        };

        trim_and_crop_video(input_path_str, output_path_str, 2.0, 6.0, None, &config).unwrap();

        // Verify output exists
        assert!(output_path.exists());

        // Verify output duration
        let output_metadata = probe_video_metadata(output_path_str).unwrap();
        assert!((output_metadata.duration_sec - 4.0).abs() < 0.5);
    }

    #[test]
    fn test_crop_video_with_valid_rectangle() {
        let temp_dir = TempDir::new().unwrap();
        let input_path = temp_dir.path().join("input.mp4");
        let output_path = temp_dir.path().join("output.mp4");
        let input_path_str = input_path.to_str().unwrap();
        let output_path_str = output_path.to_str().unwrap();

        // Create a test video
        create_test_video(input_path_str, 3.0, 320, 240).unwrap();

        // Crop to 160x120 starting at (80, 60)
        let crop = CropRect {
            x: 80,
            y: 60,
            width: 160,
            height: 120,
        };

        let config = ExportConfig {
            video_codec: "libx264".to_string(),
            audio_codec: "aac".to_string(),
            quality: "23".to_string(),
        };

        trim_and_crop_video(input_path_str, output_path_str, 0.0, 3.0, Some(crop), &config).unwrap();

        // Verify output exists
        assert!(output_path.exists());

        // Verify output dimensions
        let output_metadata = probe_video_metadata(output_path_str).unwrap();
        assert_eq!(output_metadata.width, 160);
        assert_eq!(output_metadata.height, 120);
    }

    #[test]
    fn test_combined_trim_and_crop() {
        let temp_dir = TempDir::new().unwrap();
        let input_path = temp_dir.path().join("input.mp4");
        let output_path = temp_dir.path().join("output.mp4");
        let input_path_str = input_path.to_str().unwrap();
        let output_path_str = output_path.to_str().unwrap();

        // Create a 10 second test video
        create_test_video(input_path_str, 10.0, 640, 480).unwrap();

        // Trim from 3 to 7 seconds and crop to 320x240 at (160, 120)
        let crop = CropRect {
            x: 160,
            y: 120,
            width: 320,
            height: 240,
        };

        let config = ExportConfig {
            video_codec: "libx264".to_string(),
            audio_codec: "aac".to_string(),
            quality: "23".to_string(),
        };

        trim_and_crop_video(input_path_str, output_path_str, 3.0, 7.0, Some(crop), &config).unwrap();

        // Verify output exists
        assert!(output_path.exists());

        // Verify output metadata
        let output_metadata = probe_video_metadata(output_path_str).unwrap();
        assert_eq!(output_metadata.width, 320);
        assert_eq!(output_metadata.height, 240);
        assert!((output_metadata.duration_sec - 4.0).abs() < 0.5);
    }

    // Property-based tests
    use proptest::prelude::*;

    // Feature: media-editor, Property 6: FFmpeg errors are propagated
    // Validates: Requirements 8.1
    proptest! {
        #[test]
        fn test_ffmpeg_error_propagation(
            invalid_path in "[a-z]{10,20}\\.mp4",
            start_sec in 0.0f64..100.0f64,
            end_sec in 100.0f64..200.0f64,
        ) {
            let temp_dir = TempDir::new().unwrap();
            let output_path = temp_dir.path().join("output.mp4");
            let output_path_str = output_path.to_str().unwrap();

            let config = ExportConfig {
                video_codec: "libx264".to_string(),
                audio_codec: "aac".to_string(),
                quality: "23".to_string(),
            };

            // Try to process a non-existent file
            let result = trim_and_crop_video(
                &invalid_path,
                output_path_str,
                start_sec,
                end_sec,
                None,
                &config
            );

            // Should return an error
            prop_assert!(result.is_err());
            
            // Error message should contain "FFmpeg failed"
            if let Err(err_msg) = result {
                prop_assert!(err_msg.contains("FFmpeg failed") || err_msg.contains("Failed to execute FFmpeg"));
            }
        }

        // Feature: media-editor, Property 7: Video trim produces correct duration
        // Validates: Requirements 3.5
        #[test]
        fn test_video_trim_duration(
            start_sec in 1.0f64..5.0f64,
            duration in 2.0f64..5.0f64,
        ) {
            let temp_dir = TempDir::new().unwrap();
            let input_path = temp_dir.path().join("input.mp4");
            let output_path = temp_dir.path().join("output.mp4");
            let input_path_str = input_path.to_str().unwrap();
            let output_path_str = output_path.to_str().unwrap();

            let end_sec = start_sec + duration;
            let total_duration = end_sec + 2.0; // Make sure video is long enough

            // Create a test video
            create_test_video(input_path_str, total_duration, 320, 240).unwrap();

            let config = ExportConfig {
                video_codec: "libx264".to_string(),
                audio_codec: "aac".to_string(),
                quality: "23".to_string(),
            };

            // Trim the video
            trim_and_crop_video(
                input_path_str,
                output_path_str,
                start_sec,
                end_sec,
                None,
                &config
            ).unwrap();

            // Verify output duration
            let output_metadata = probe_video_metadata(output_path_str).unwrap();
            let expected_duration = end_sec - start_sec;
            let actual_duration = output_metadata.duration_sec;
            
            // Allow 0.1 second tolerance as specified in the property
            prop_assert!(
                (actual_duration - expected_duration).abs() < 0.1,
                "Expected duration {} but got {}", expected_duration, actual_duration
            );
        }
    }
}
