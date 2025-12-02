// Integration tests for media editor
// These tests verify complete workflows from loading to exporting media

use milk_lib::media_editor::{
    image_ops::{crop_image, crop_image_command},
    video_ops::{probe_video_metadata, trim_and_crop_video, probe_video_metadata_command, trim_and_crop_video_command},
    types::{CropRect, ExportConfig},
};
use tempfile::TempDir;
use std::process::Command;
use image::{ImageBuffer, Rgb, DynamicImage, GenericImageView};

/// Helper function to create a test image with a solid color
fn create_test_image(width: u32, height: u32, color: [u8; 3]) -> DynamicImage {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |_, _| {
        Rgb(color)
    });
    DynamicImage::ImageRgb8(img)
}

/// Helper function to create a test video file using FFmpeg
fn create_test_video(path: &str, duration_sec: f64, width: u32, height: u32) -> Result<(), String> {
    let output = Command::new("ffmpeg")
        .args([
            "-y",
            "-f", "lavfi",
            "-i", &format!("testsrc=duration={}:size={}x{}:rate=30", duration_sec, width, height),
            "-pix_fmt", "yuv420p",
            "-g", "1", // Keyframe interval for accurate seeking
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

// Integration Test 1: Complete image workflow
// Load image → crop → export → verify
#[test]
fn test_image_workflow_load_crop_export_verify() {
    let temp_dir = TempDir::new().unwrap();
    
    // Step 1: Create a test image (simulating "load")
    let original_width = 400;
    let original_height = 300;
    let img = create_test_image(original_width, original_height, [255, 128, 64]);
    let input_path = temp_dir.path().join("input.png");
    img.save(&input_path).unwrap();
    
    // Step 2: Define crop rectangle
    let crop_rect = CropRect {
        x: 50,
        y: 50,
        width: 200,
        height: 150,
    };
    
    // Step 3: Export with crop
    let output_path = temp_dir.path().join("output.png");
    let result = crop_image(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &crop_rect,
    );
    
    assert!(result.is_ok(), "Crop operation should succeed");
    
    // Step 4: Verify output
    assert!(output_path.exists(), "Output file should exist");
    
    let output_img = image::open(&output_path).unwrap();
    assert_eq!(output_img.width(), 200, "Output width should match crop width");
    assert_eq!(output_img.height(), 150, "Output height should match crop height");
    
    // Verify the cropped image has the same color
    let pixel = output_img.get_pixel(100, 75);
    assert_eq!(pixel[0], 255, "Red channel should be preserved");
    assert_eq!(pixel[1], 128, "Green channel should be preserved");
    assert_eq!(pixel[2], 64, "Blue channel should be preserved");
}

// Integration Test 2: Complete video workflow with trim only
// Load video → trim → export → verify
#[test]
fn test_video_workflow_load_trim_export_verify() {
    let temp_dir = TempDir::new().unwrap();
    
    // Step 1: Create a test video (simulating "load")
    let input_path = temp_dir.path().join("input.mp4");
    let input_path_str = input_path.to_str().unwrap();
    create_test_video(input_path_str, 10.0, 640, 480).unwrap();
    
    // Step 2: Probe metadata (simulating loading video info)
    let metadata = probe_video_metadata(input_path_str).unwrap();
    assert_eq!(metadata.width, 640);
    assert_eq!(metadata.height, 480);
    assert!((metadata.duration_sec - 10.0).abs() < 0.5);
    
    // Step 3: Define trim parameters
    let start_sec = 2.0;
    let end_sec = 7.0;
    
    // Step 4: Export with trim
    let output_path = temp_dir.path().join("output.mp4");
    let output_path_str = output_path.to_str().unwrap();
    
    let config = ExportConfig {
        video_codec: "libx264".to_string(),
        audio_codec: "aac".to_string(),
        quality: "23".to_string(),
    };
    
    let result = trim_and_crop_video(
        input_path_str,
        output_path_str,
        start_sec,
        end_sec,
        None,
        &config,
    );
    
    assert!(result.is_ok(), "Trim operation should succeed");
    
    // Step 5: Verify output
    assert!(output_path.exists(), "Output file should exist");
    
    let output_metadata = probe_video_metadata(output_path_str).unwrap();
    assert_eq!(output_metadata.width, 640, "Width should be preserved");
    assert_eq!(output_metadata.height, 480, "Height should be preserved");
    
    let expected_duration = end_sec - start_sec;
    assert!(
        (output_metadata.duration_sec - expected_duration).abs() < 0.5,
        "Duration should be approximately {} seconds, got {}",
        expected_duration,
        output_metadata.duration_sec
    );
}

// Integration Test 3: Complete video workflow with crop and trim
// Load video → crop → trim → export → verify
#[test]
fn test_video_workflow_load_crop_trim_export_verify() {
    let temp_dir = TempDir::new().unwrap();
    
    // Step 1: Create a test video (simulating "load")
    let input_path = temp_dir.path().join("input.mp4");
    let input_path_str = input_path.to_str().unwrap();
    create_test_video(input_path_str, 12.0, 800, 600).unwrap();
    
    // Step 2: Probe metadata
    let metadata = probe_video_metadata(input_path_str).unwrap();
    assert_eq!(metadata.width, 800);
    assert_eq!(metadata.height, 600);
    
    // Step 3: Define crop and trim parameters
    let crop_rect = CropRect {
        x: 100,
        y: 100,
        width: 400,
        height: 300,
    };
    let start_sec = 3.0;
    let end_sec = 9.0;
    
    // Step 4: Export with both crop and trim
    let output_path = temp_dir.path().join("output.mp4");
    let output_path_str = output_path.to_str().unwrap();
    
    let config = ExportConfig {
        video_codec: "libx264".to_string(),
        audio_codec: "aac".to_string(),
        quality: "23".to_string(),
    };
    
    let result = trim_and_crop_video(
        input_path_str,
        output_path_str,
        start_sec,
        end_sec,
        Some(crop_rect),
        &config,
    );
    
    assert!(result.is_ok(), "Crop and trim operation should succeed");
    
    // Step 5: Verify output
    assert!(output_path.exists(), "Output file should exist");
    
    let output_metadata = probe_video_metadata(output_path_str).unwrap();
    assert_eq!(output_metadata.width, 400, "Width should match crop width");
    assert_eq!(output_metadata.height, 300, "Height should match crop height");
    
    let expected_duration = end_sec - start_sec;
    assert!(
        (output_metadata.duration_sec - expected_duration).abs() < 0.5,
        "Duration should be approximately {} seconds, got {}",
        expected_duration,
        output_metadata.duration_sec
    );
}

// Integration Test 4: Error handling - invalid image file
#[test]
fn test_error_handling_invalid_image_file() {
    let temp_dir = TempDir::new().unwrap();
    
    // Try to crop a non-existent file
    let input_path = temp_dir.path().join("nonexistent.png");
    let output_path = temp_dir.path().join("output.png");
    
    let crop_rect = CropRect {
        x: 0,
        y: 0,
        width: 100,
        height: 100,
    };
    
    let result = crop_image(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &crop_rect,
    );
    
    assert!(result.is_err(), "Should fail with non-existent file");
    assert!(result.unwrap_err().contains("Failed to load image"));
}

// Integration Test 5: Error handling - invalid video file
#[test]
fn test_error_handling_invalid_video_file() {
    let temp_dir = TempDir::new().unwrap();
    
    // Try to process a non-existent video
    let input_path = temp_dir.path().join("nonexistent.mp4");
    let output_path = temp_dir.path().join("output.mp4");
    
    let config = ExportConfig {
        video_codec: "libx264".to_string(),
        audio_codec: "aac".to_string(),
        quality: "23".to_string(),
    };
    
    let result = trim_and_crop_video(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        0.0,
        5.0,
        None,
        &config,
    );
    
    assert!(result.is_err(), "Should fail with non-existent file");
    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("FFmpeg failed") || error_msg.contains("Failed to execute FFmpeg"),
        "Error message should indicate FFmpeg failure"
    );
}

// Integration Test 6: Error handling - invalid crop rectangle
#[test]
fn test_error_handling_invalid_crop_rectangle() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test image
    let img = create_test_image(200, 200, [100, 100, 100]);
    let input_path = temp_dir.path().join("input.png");
    img.save(&input_path).unwrap();
    
    // Try to crop with origin outside bounds
    let crop_rect = CropRect {
        x: 300,
        y: 300,
        width: 50,
        height: 50,
    };
    
    let output_path = temp_dir.path().join("output.png");
    let result = crop_image(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &crop_rect,
    );
    
    assert!(result.is_err(), "Should fail with invalid crop origin");
    assert!(result.unwrap_err().contains("outside image bounds"));
}

// Integration Test 7: Error handling - invalid trim times
#[test]
fn test_error_handling_invalid_trim_times() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a short test video
    let input_path = temp_dir.path().join("input.mp4");
    let input_path_str = input_path.to_str().unwrap();
    create_test_video(input_path_str, 5.0, 320, 240).unwrap();
    
    // Try to trim beyond video duration
    let output_path = temp_dir.path().join("output.mp4");
    let output_path_str = output_path.to_str().unwrap();
    
    let config = ExportConfig {
        video_codec: "libx264".to_string(),
        audio_codec: "aac".to_string(),
        quality: "23".to_string(),
    };
    
    // This should still work but produce a shorter video than requested
    let result = trim_and_crop_video(
        input_path_str,
        output_path_str,
        10.0, // Start beyond video duration
        15.0,
        None,
        &config,
    );
    
    // FFmpeg will handle this gracefully, but the output will be empty or very short
    // This is expected behavior - FFmpeg doesn't error, it just produces minimal output
    assert!(result.is_ok(), "FFmpeg handles out-of-range times gracefully");
}

// Integration Test 8: Tauri command invocations (async)
#[tokio::test]
async fn test_tauri_command_crop_image() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test image
    let img = create_test_image(300, 200, [200, 150, 100]);
    let input_path = temp_dir.path().join("input.png");
    img.save(&input_path).unwrap();
    
    // Call the Tauri command
    let crop_rect = CropRect {
        x: 50,
        y: 50,
        width: 150,
        height: 100,
    };
    
    let output_path = temp_dir.path().join("output.png");
    let result = crop_image_command(
        input_path.to_str().unwrap().to_string(),
        output_path.to_str().unwrap().to_string(),
        crop_rect,
    ).await;
    
    assert!(result.is_ok(), "Tauri command should succeed");
    assert!(output_path.exists(), "Output file should exist");
    
    let output_img = image::open(&output_path).unwrap();
    assert_eq!(output_img.width(), 150);
    assert_eq!(output_img.height(), 100);
}

#[tokio::test]
async fn test_tauri_command_probe_video_metadata() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test video
    let input_path = temp_dir.path().join("input.mp4");
    let input_path_str = input_path.to_str().unwrap();
    create_test_video(input_path_str, 8.0, 480, 360).unwrap();
    
    // Call the Tauri command
    let result = probe_video_metadata_command(input_path_str.to_string()).await;
    
    assert!(result.is_ok(), "Tauri command should succeed");
    
    let metadata = result.unwrap();
    assert_eq!(metadata.width, 480);
    assert_eq!(metadata.height, 360);
    assert!((metadata.duration_sec - 8.0).abs() < 0.5);
}

#[tokio::test]
async fn test_tauri_command_trim_and_crop_video() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test video
    let input_path = temp_dir.path().join("input.mp4");
    let input_path_str = input_path.to_str().unwrap();
    create_test_video(input_path_str, 10.0, 640, 480).unwrap();
    
    // Call the Tauri command
    let crop_rect = CropRect {
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
    
    let output_path = temp_dir.path().join("output.mp4");
    let output_path_str = output_path.to_str().unwrap();
    
    let result = trim_and_crop_video_command(
        input_path_str.to_string(),
        output_path_str.to_string(),
        2.0,
        6.0,
        Some(crop_rect),
        config,
    ).await;
    
    assert!(result.is_ok(), "Tauri command should succeed");
    assert!(output_path.exists(), "Output file should exist");
    
    let output_metadata = probe_video_metadata(output_path_str).unwrap();
    assert_eq!(output_metadata.width, 320);
    assert_eq!(output_metadata.height, 240);
    assert!((output_metadata.duration_sec - 4.0).abs() < 0.5);
}

// Integration Test 9: Multiple operations in sequence
#[test]
fn test_multiple_operations_in_sequence() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create initial video
    let video1_path = temp_dir.path().join("video1.mp4");
    let video1_path_str = video1_path.to_str().unwrap();
    create_test_video(video1_path_str, 15.0, 800, 600).unwrap();
    
    // First operation: trim
    let video2_path = temp_dir.path().join("video2.mp4");
    let video2_path_str = video2_path.to_str().unwrap();
    
    let config = ExportConfig {
        video_codec: "libx264".to_string(),
        audio_codec: "aac".to_string(),
        quality: "23".to_string(),
    };
    
    trim_and_crop_video(video1_path_str, video2_path_str, 3.0, 12.0, None, &config).unwrap();
    
    // Second operation: crop the trimmed video
    let video3_path = temp_dir.path().join("video3.mp4");
    let video3_path_str = video3_path.to_str().unwrap();
    
    let crop_rect = CropRect {
        x: 200,
        y: 150,
        width: 400,
        height: 300,
    };
    
    let metadata2 = probe_video_metadata(video2_path_str).unwrap();
    trim_and_crop_video(
        video2_path_str,
        video3_path_str,
        0.0,
        metadata2.duration_sec,
        Some(crop_rect),
        &config,
    ).unwrap();
    
    // Verify final output
    let metadata3 = probe_video_metadata(video3_path_str).unwrap();
    assert_eq!(metadata3.width, 400);
    assert_eq!(metadata3.height, 300);
    assert!((metadata3.duration_sec - 9.0).abs() < 0.5);
}

// Integration Test 10: Edge case - crop entire image (no actual cropping)
#[test]
fn test_edge_case_crop_entire_image() {
    let temp_dir = TempDir::new().unwrap();
    
    let width = 250;
    let height = 180;
    
    // Create a test image
    let img = create_test_image(width, height, [50, 100, 150]);
    let input_path = temp_dir.path().join("input.png");
    img.save(&input_path).unwrap();
    
    // Crop the entire image
    let crop_rect = CropRect {
        x: 0,
        y: 0,
        width,
        height,
    };
    
    let output_path = temp_dir.path().join("output.png");
    let result = crop_image(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        &crop_rect,
    );
    
    assert!(result.is_ok(), "Cropping entire image should succeed");
    
    let output_img = image::open(&output_path).unwrap();
    assert_eq!(output_img.width(), width);
    assert_eq!(output_img.height(), height);
}
