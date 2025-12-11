// Image operations module
// This module contains image cropping and manipulation functions

use crate::media_editor::types::CropRect;
use image::{DynamicImage, GenericImageView};
use std::path::Path;

/// Crops an image to the specified rectangle and saves it to the output path
///
/// # Arguments
/// * `input_path` - Path to the input image file
/// * `output_path` - Path where the cropped image will be saved
/// * `crop_rect` - Rectangle defining the crop area (x, y, width, height)
///
/// # Returns
/// * `Ok(())` if the operation succeeds
/// * `Err(String)` with error description if the operation fails
///
/// # Requirements
/// * 1.5: Export cropped image with only the selected rectangular area
/// * 7.4: Use configured image format from defaults
pub fn crop_image(
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    crop_rect: &CropRect,
) -> Result<(), String> {
    // Load the image
    let img = image::open(&input_path).map_err(|e| format!("Failed to load image: {}", e))?;

    // Get image dimensions
    let (img_width, img_height) = img.dimensions();

    // Validate crop rectangle bounds
    if crop_rect.x >= img_width || crop_rect.y >= img_height {
        return Err(format!(
            "Crop rectangle origin ({}, {}) is outside image bounds ({}x{})",
            crop_rect.x, crop_rect.y, img_width, img_height
        ));
    }

    if crop_rect.width == 0 || crop_rect.height == 0 {
        return Err("Crop rectangle dimensions must be greater than zero".to_string());
    }

    // Clamp crop rectangle to image bounds
    let actual_width = crop_rect.width.min(img_width - crop_rect.x);
    let actual_height = crop_rect.height.min(img_height - crop_rect.y);

    // Perform the crop
    let cropped = img.crop_imm(crop_rect.x, crop_rect.y, actual_width, actual_height);

    // Save the cropped image
    cropped
        .save(&output_path)
        .map_err(|e| format!("Failed to save cropped image: {}", e))?;

    Ok(())
}

/// Tauri command for cropping an image
///
/// # Arguments
/// * `input_path` - Path to the input image file
/// * `output_path` - Path where the cropped image will be saved
/// * `crop_rect` - Rectangle defining the crop area
///
/// # Returns
/// * `Ok(())` if the operation succeeds
/// * `Err(String)` with error description if the operation fails
///
/// # Requirements
/// * 1.4: Convert preview coordinates to source coordinates accurately
/// * 1.5: Save only the selected rectangular area to output file
#[tauri::command]
pub async fn crop_image_command(
    input_path: String,
    output_path: String,
    crop_rect: CropRect,
) -> Result<(), String> {
    crop_image(input_path, output_path, &crop_rect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Helper function to create a test image with a solid color
    fn create_test_image(width: u32, height: u32, color: [u8; 3]) -> DynamicImage {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_fn(width, height, |_, _| Rgb(color));
        DynamicImage::ImageRgb8(img)
    }

    /// Helper function to save a test image and return its path
    fn save_test_image(dir: &TempDir, name: &str, img: &DynamicImage) -> PathBuf {
        let path = dir.path().join(name);
        img.save(&path).unwrap();
        path
    }

    #[test]
    fn test_crop_image_with_valid_rectangle() {
        let temp_dir = TempDir::new().unwrap();

        // Create a 100x100 red image
        let img = create_test_image(100, 100, [255, 0, 0]);
        let input_path = save_test_image(&temp_dir, "input.png", &img);
        let output_path = temp_dir.path().join("output.png");

        // Crop a 50x50 rectangle from (10, 10)
        let crop_rect = CropRect {
            x: 10,
            y: 10,
            width: 50,
            height: 50,
        };

        let result = crop_image(&input_path, &output_path, &crop_rect);
        assert!(result.is_ok(), "Crop operation should succeed");

        // Verify the output image exists and has correct dimensions
        let cropped_img = image::open(&output_path).unwrap();
        assert_eq!(cropped_img.width(), 50);
        assert_eq!(cropped_img.height(), 50);
    }

    #[test]
    fn test_crop_at_image_boundaries() {
        let temp_dir = TempDir::new().unwrap();

        // Create a 100x100 image
        let img = create_test_image(100, 100, [0, 255, 0]);
        let input_path = save_test_image(&temp_dir, "input.png", &img);
        let output_path = temp_dir.path().join("output.png");

        // Crop from (0, 0) to full size
        let crop_rect = CropRect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };

        let result = crop_image(&input_path, &output_path, &crop_rect);
        assert!(result.is_ok(), "Crop at boundaries should succeed");

        let cropped_img = image::open(&output_path).unwrap();
        assert_eq!(cropped_img.width(), 100);
        assert_eq!(cropped_img.height(), 100);
    }

    #[test]
    fn test_crop_with_rectangle_extending_beyond_bounds() {
        let temp_dir = TempDir::new().unwrap();

        // Create a 100x100 image
        let img = create_test_image(100, 100, [0, 0, 255]);
        let input_path = save_test_image(&temp_dir, "input.png", &img);
        let output_path = temp_dir.path().join("output.png");

        // Crop rectangle extends beyond image bounds
        let crop_rect = CropRect {
            x: 50,
            y: 50,
            width: 100,  // Would extend to x=150, but image is only 100 wide
            height: 100, // Would extend to y=150, but image is only 100 tall
        };

        let result = crop_image(&input_path, &output_path, &crop_rect);
        assert!(result.is_ok(), "Crop should succeed with clamping");

        // Should be clamped to 50x50 (from 50,50 to 100,100)
        let cropped_img = image::open(&output_path).unwrap();
        assert_eq!(cropped_img.width(), 50);
        assert_eq!(cropped_img.height(), 50);
    }

    #[test]
    fn test_crop_with_invalid_origin() {
        let temp_dir = TempDir::new().unwrap();

        // Create a 100x100 image
        let img = create_test_image(100, 100, [255, 255, 0]);
        let input_path = save_test_image(&temp_dir, "input.png", &img);
        let output_path = temp_dir.path().join("output.png");

        // Crop rectangle origin is outside image bounds
        let crop_rect = CropRect {
            x: 150,
            y: 150,
            width: 50,
            height: 50,
        };

        let result = crop_image(&input_path, &output_path, &crop_rect);
        assert!(result.is_err(), "Crop with invalid origin should fail");
        assert!(result.unwrap_err().contains("outside image bounds"));
    }

    #[test]
    fn test_crop_with_zero_dimensions() {
        let temp_dir = TempDir::new().unwrap();

        // Create a 100x100 image
        let img = create_test_image(100, 100, [255, 0, 255]);
        let input_path = save_test_image(&temp_dir, "input.png", &img);
        let output_path = temp_dir.path().join("output.png");

        // Crop rectangle with zero width
        let crop_rect = CropRect {
            x: 10,
            y: 10,
            width: 0,
            height: 50,
        };

        let result = crop_image(&input_path, &output_path, &crop_rect);
        assert!(result.is_err(), "Crop with zero width should fail");
        assert!(result.unwrap_err().contains("must be greater than zero"));
    }

    #[test]
    fn test_crop_with_nonexistent_input() {
        let temp_dir = TempDir::new().unwrap();

        let input_path = temp_dir.path().join("nonexistent.png");
        let output_path = temp_dir.path().join("output.png");

        let crop_rect = CropRect {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };

        let result = crop_image(&input_path, &output_path, &crop_rect);
        assert!(result.is_err(), "Crop with nonexistent input should fail");
        assert!(result.unwrap_err().contains("Failed to load image"));
    }

    // Property-based tests
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        /// Feature: media-editor, Property 4: Export without crop preserves original dimensions
        ///
        /// For any media file, if no crop rectangle is defined when exporting (i.e., the crop
        /// rectangle covers the entire image), the output file SHALL have the same dimensions
        /// as the input file.
        ///
        /// Validates: Requirements 1.5
        #[test]
        fn prop_export_without_crop_preserves_dimensions(
            width in 10u32..500u32,
            height in 10u32..500u32,
            r in 0u8..=255,
            g in 0u8..=255,
            b in 0u8..=255,
        ) {
            let temp_dir = TempDir::new().unwrap();

            // Create a test image with random dimensions and color
            let img = create_test_image(width, height, [r, g, b]);
            let input_path = save_test_image(&temp_dir, "input.png", &img);
            let output_path = temp_dir.path().join("output.png");

            // Crop the entire image (no actual cropping)
            let crop_rect = CropRect {
                x: 0,
                y: 0,
                width,
                height,
            };

            let result = crop_image(&input_path, &output_path, &crop_rect);
            prop_assert!(result.is_ok(), "Export without crop should succeed");

            // Verify the output has the same dimensions as input
            let output_img = image::open(&output_path).unwrap();
            prop_assert_eq!(output_img.width(), width, "Width should be preserved");
            prop_assert_eq!(output_img.height(), height, "Height should be preserved");
        }
    }
}
