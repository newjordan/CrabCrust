// Video and GIF to Braille frame converter
//
// Converts video files and animated GIFs into sequences of Braille frames
// that can be played back in CrabCrust animations.

use crate::braille::BrailleGrid;
use anyhow::{Context, Result};
use std::path::Path;

/// Represents a single frame of animation data
#[derive(Clone)]
pub struct BrailleFrame {
    /// Dot patterns for each cell
    pub patterns: Vec<u8>,
    /// Width in terminal cells
    pub width: usize,
    /// Height in terminal cells
    pub height: usize,
    /// Frame duration in milliseconds
    pub duration_ms: u32,
}

impl BrailleFrame {
    /// Create a new frame from a BrailleGrid
    pub fn from_grid(grid: &BrailleGrid, duration_ms: u32) -> Self {
        let width = grid.width();
        let height = grid.height();
        let mut patterns = vec![0u8; width * height];

        // Extract patterns from grid
        for y in 0..height {
            for x in 0..width {
                let ch = grid.get_char(x, y);
                // Convert char back to pattern byte
                let pattern = (ch as u32 - 0x2800) as u8;
                patterns[y * width + x] = pattern;
            }
        }

        Self {
            patterns,
            width,
            height,
            duration_ms,
        }
    }

    /// Apply this frame to a BrailleGrid for rendering
    pub fn apply_to_grid(&self, grid: &mut BrailleGrid) {
        for y in 0..self.height.min(grid.height()) {
            for x in 0..self.width.min(grid.width()) {
                let pattern = self.patterns[y * self.width + x];
                if pattern != 0 {
                    // Reconstruct the dots by setting each bit
                    for bit in 0..8 {
                        if (pattern & (1 << bit)) != 0 {
                            // Map bit to dot position
                            let (dot_x, dot_y) = match bit {
                                0 => (x * 2, y * 4),     // Dot 1
                                1 => (x * 2, y * 4 + 1), // Dot 2
                                2 => (x * 2, y * 4 + 2), // Dot 3
                                3 => (x * 2 + 1, y * 4), // Dot 4
                                4 => (x * 2 + 1, y * 4 + 1), // Dot 5
                                5 => (x * 2 + 1, y * 4 + 2), // Dot 6
                                6 => (x * 2, y * 4 + 3), // Dot 7
                                7 => (x * 2 + 1, y * 4 + 3), // Dot 8
                                _ => unreachable!(),
                            };
                            grid.set_dot(dot_x, dot_y);
                        }
                    }
                }
            }
        }
    }
}

/// Convert a static image to a single Braille frame
pub fn image_to_frame<P: AsRef<Path>>(
    path: P,
    width: usize,
    height: usize,
    threshold: u8,
) -> Result<BrailleFrame> {
    let img = image::open(path.as_ref())
        .context("Failed to open image file")?;

    // Convert to grayscale
    let gray = img.to_luma8();
    let (img_w, img_h) = gray.dimensions();

    // Create Braille grid
    let mut grid = BrailleGrid::new(width, height);

    // Convert to Braille
    super::blit_luma_to_braille(
        gray.as_raw(),
        img_w as usize,
        img_h as usize,
        threshold,
        &mut grid,
    );

    Ok(BrailleFrame::from_grid(&grid, 100))
}

/// Convert an animated GIF to a sequence of Braille frames
pub fn gif_to_frames<P: AsRef<Path>>(
    path: P,
    width: usize,
    height: usize,
    threshold: u8,
) -> Result<Vec<BrailleFrame>> {
    use image::AnimationDecoder;
    use std::fs::File;

    let file = File::open(path.as_ref())
        .context("Failed to open GIF file")?;

    let decoder = image::codecs::gif::GifDecoder::new(file)
        .context("Failed to decode GIF")?;

    let frames = decoder.into_frames();
    let mut braille_frames = Vec::new();

    for frame_result in frames {
        let frame = frame_result.context("Failed to decode GIF frame")?;
        let (delay_num, delay_den) = frame.delay().numer_denom_ms();
        let duration_ms = (delay_num / delay_den.max(1)) as u32;

        // Convert frame to grayscale
        let img = frame.buffer();
        let gray = image::DynamicImage::ImageRgba8(img.clone()).to_luma8();
        let (img_w, img_h) = gray.dimensions();

        // Create Braille grid
        let mut grid = BrailleGrid::new(width, height);

        // Convert to Braille
        super::blit_luma_to_braille(
            gray.as_raw(),
            img_w as usize,
            img_h as usize,
            threshold,
            &mut grid,
        );

        braille_frames.push(BrailleFrame::from_grid(&grid, duration_ms.max(10)));
    }

    Ok(braille_frames)
}

/// Convert a video file to a sequence of Braille frames using ffmpeg
#[cfg(feature = "video")]
pub fn video_to_frames<P: AsRef<Path>>(
    path: P,
    width: usize,
    height: usize,
    threshold: u8,
    max_frames: Option<usize>,
) -> Result<Vec<BrailleFrame>> {
    use ffmpeg_next as ffmpeg;

    ffmpeg::init().context("Failed to initialize FFmpeg")?;

    let mut input = ffmpeg::format::input(&path)
        .context("Failed to open video file")?;

    let input_stream = input
        .streams()
        .best(ffmpeg::media::Type::Video)
        .context("No video stream found")?;

    let video_stream_index = input_stream.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())
        .context("Failed to create codec context")?;

    let mut decoder = context_decoder
        .decoder()
        .video()
        .context("Failed to create video decoder")?;

    let mut scaler = ffmpeg::software::scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::GRAY8,
        width * 2,  // Braille dot width
        height * 4, // Braille dot height
        ffmpeg::software::scaling::Flags::BILINEAR,
    ).context("Failed to create scaler")?;

    let mut braille_frames = Vec::new();
    let mut frame_count = 0;

    let time_base = input_stream.time_base();
    let fps = input_stream.avg_frame_rate();
    let frame_duration_ms = if fps.numerator() > 0 {
        (1000 * fps.denominator() as u32) / fps.numerator() as u32
    } else {
        33 // Default to ~30fps
    };

    let receive_and_process_frames = |decoder: &mut ffmpeg::decoder::Video,
                                       braille_frames: &mut Vec<BrailleFrame>,
                                       frame_count: &mut usize,
                                       scaler: &mut ffmpeg::software::scaling::Context,
                                       width: usize,
                                       height: usize,
                                       threshold: u8,
                                       frame_duration_ms: u32,
                                       max_frames: Option<usize>| -> Result<bool> {
        let mut decoded = ffmpeg::util::frame::video::Video::empty();
        while decoder.receive_frame(&mut decoded).is_ok() {
            if let Some(max) = max_frames {
                if *frame_count >= max {
                    return Ok(true); // Done
                }
            }

            let mut gray_frame = ffmpeg::util::frame::video::Video::empty();
            scaler.run(&decoded, &mut gray_frame)
                .context("Failed to scale frame")?;

            // Create Braille grid
            let mut grid = BrailleGrid::new(width, height);

            // Get grayscale data
            let data = gray_frame.data(0);

            // Convert to Braille
            super::blit_luma_to_braille(
                data,
                width * 2,
                height * 4,
                threshold,
                &mut grid,
            );

            braille_frames.push(BrailleFrame::from_grid(&grid, frame_duration_ms));
            *frame_count += 1;
        }
        Ok(false)
    };

    for (stream, packet) in input.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).context("Failed to send packet")?;

            if receive_and_process_frames(
                &mut decoder,
                &mut braille_frames,
                &mut frame_count,
                &mut scaler,
                width,
                height,
                threshold,
                frame_duration_ms,
                max_frames,
            )? {
                break;
            }
        }
    }

    // Flush decoder
    decoder.send_eof().context("Failed to send EOF")?;
    receive_and_process_frames(
        &mut decoder,
        &mut braille_frames,
        &mut frame_count,
        &mut scaler,
        width,
        height,
        threshold,
        frame_duration_ms,
        max_frames,
    )?;

    Ok(braille_frames)
}
