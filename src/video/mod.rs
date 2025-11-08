// Video and image conversion utilities for CrabCrust
//
// This module provides functions to convert video frames and images
// into Braille dot patterns for terminal rendering.

#[cfg(any(feature = "gif", feature = "video"))]
use crate::braille::BrailleGrid;

/// Convert luminance (grayscale) data to Braille dots
///
/// This function takes a grayscale image and converts it to Braille dots
/// using nearest-neighbor scaling and binary thresholding.
///
/// # Arguments
/// * `luma` - Grayscale pixel data (0-255, where 255 is white)
/// * `img_w` - Source image width in pixels
/// * `img_h` - Source image height in pixels
/// * `threshold` - Luminance threshold (pixels >= threshold become dots)
/// * `braille` - Target Braille grid to write dots to
///
/// # Examples
///
/// ```no_run
/// use crabcrust::braille::BrailleGrid;
/// use crabcrust::video::blit_luma_to_braille;
///
/// let mut grid = BrailleGrid::new(64, 16); // 128x64 dot resolution
/// let luma_data = vec![255u8; 128 * 32]; // White 128x32 image
/// blit_luma_to_braille(&luma_data, 128, 32, 128, &mut grid);
/// ```
#[cfg(any(feature = "gif", feature = "video"))]
pub fn blit_luma_to_braille(
    luma: &[u8],
    img_w: usize,
    img_h: usize,
    threshold: u8,
    braille: &mut BrailleGrid,
) {
    let dot_w = braille.dot_width();
    let dot_h = braille.dot_height();

    for dy in 0..dot_h {
        // Map dot Y to source image Y using nearest-neighbor
        let sy = (dy * img_h) / dot_h;
        let sy_off = sy * img_w;

        for dx in 0..dot_w {
            // Map dot X to source image X using nearest-neighbor
            let sx = (dx * img_w) / dot_w;
            let v = luma[sy_off + sx];

            // Binary threshold: bright pixels become dots
            if v >= threshold {
                braille.set_dot(dx, dy);
            }
        }
    }
}

#[cfg(any(feature = "gif", feature = "video"))]
pub mod converter;

#[cfg(any(feature = "gif", feature = "video"))]
pub mod pac;

#[cfg(test)]
#[cfg(any(feature = "gif", feature = "video"))]
mod tests {
    use super::*;

    #[test]
    fn test_blit_luma_to_braille() {
        let mut grid = BrailleGrid::new(2, 2); // 4x8 dot resolution

        // Create a simple 4x8 white image
        let luma = vec![255u8; 4 * 8];

        blit_luma_to_braille(&luma, 4, 8, 128, &mut grid);

        // All cells should have dots
        for y in 0..2 {
            for x in 0..2 {
                assert!(!grid.is_empty(x, y));
            }
        }
    }

    #[test]
    fn test_blit_luma_threshold() {
        let mut grid = BrailleGrid::new(2, 2);

        // Create an image with half dark, half bright
        let mut luma = vec![0u8; 4 * 8];
        for i in 0..luma.len() / 2 {
            luma[i] = 255;
        }

        blit_luma_to_braille(&luma, 4, 8, 128, &mut grid);

        // Top cells should have dots, bottom should be empty
        assert!(!grid.is_empty(0, 0));
        assert!(!grid.is_empty(1, 0));
    }
}
