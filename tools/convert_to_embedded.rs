// Tool to pre-convert video/gif to embedded Rust frame data
// Run with: cargo run --features video --bin convert_to_embedded -- <input_file> <output_name>
//
// This generates Rust source code with pre-converted Braille frames
// that can be directly embedded in the binary without runtime FFmpeg.

use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_video> <output_name>", args[0]);
        eprintln!("Example: {} ref/computer_fingers.mp4 computer_fingers", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_name = &args[2];

    println!("Converting {} to embedded frames...", input_path);

    // Initialize FFmpeg
    ffmpeg_next::init().context("Failed to initialize FFmpeg")?;

    // Convert video to frames
    let frames = convert_video_to_frames(input_path, 124, 19, 50)?;

    println!("Converted {} frames", frames.len());

    // Generate Rust source code
    let rust_code = generate_rust_code(output_name, &frames);

    // Write to file
    let output_path = format!("src/animation/embedded_{}.rs", output_name);
    let mut file = File::create(&output_path)?;
    file.write_all(rust_code.as_bytes())?;

    println!("Generated {}", output_path);
    println!("\nAdd to src/animation/mod.rs:");
    println!("  mod embedded_{};", output_name);
    println!("  pub use embedded_{}::get_{}_frames;", output_name, output_name);

    Ok(())
}

struct BrailleFrame {
    patterns: Vec<u8>,
    width: usize,
    height: usize,
    duration_ms: u32,
}

fn convert_video_to_frames(path: &str, width: usize, height: usize, threshold: u8) -> Result<Vec<BrailleFrame>> {
    use ffmpeg_next as ffmpeg;

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
        (width * 2) as u32,
        (height * 4) as u32,
        ffmpeg::software::scaling::Flags::BILINEAR,
    ).context("Failed to create scaler")?;

    let mut braille_frames = Vec::new();

    let fps = input_stream.avg_frame_rate();
    let frame_duration_ms = if fps.numerator() > 0 {
        (1000 * fps.denominator() as u32) / fps.numerator() as u32
    } else {
        33
    };

    let process_frames = |decoder: &mut ffmpeg::decoder::Video,
                          braille_frames: &mut Vec<BrailleFrame>,
                          scaler: &mut ffmpeg::software::scaling::Context| -> Result<()> {
        let mut decoded = ffmpeg::util::frame::video::Video::empty();
        while decoder.receive_frame(&mut decoded).is_ok() {
            let mut gray_frame = ffmpeg::util::frame::video::Video::empty();
            scaler.run(&decoded, &mut gray_frame)
                .context("Failed to scale frame")?;

            let data = gray_frame.data(0);
            let patterns = convert_to_braille(data, width * 2, height * 4, threshold, width, height);

            braille_frames.push(BrailleFrame {
                patterns,
                width,
                height,
                duration_ms: frame_duration_ms,
            });
        }
        Ok(())
    };

    for (stream, packet) in input.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).context("Failed to send packet")?;
            process_frames(&mut decoder, &mut braille_frames, &mut scaler)?;
        }
    }

    decoder.send_eof().context("Failed to send EOF")?;
    process_frames(&mut decoder, &mut braille_frames, &mut scaler)?;

    Ok(braille_frames)
}

fn convert_to_braille(
    gray_data: &[u8],
    img_width: usize,
    img_height: usize,
    threshold: u8,
    cell_width: usize,
    cell_height: usize,
) -> Vec<u8> {
    let mut patterns = vec![0u8; cell_width * cell_height];

    for cy in 0..cell_height {
        for cx in 0..cell_width {
            let mut pattern: u8 = 0;

            // Each Braille cell is 2x4 dots
            for dy in 0..4 {
                for dx in 0..2 {
                    let px = cx * 2 + dx;
                    let py = cy * 4 + dy;

                    if px < img_width && py < img_height {
                        let pixel = gray_data[py * img_width + px];
                        if pixel < threshold {
                            // Map dot position to bit
                            let bit = match (dx, dy) {
                                (0, 0) => 0,
                                (0, 1) => 1,
                                (0, 2) => 2,
                                (1, 0) => 3,
                                (1, 1) => 4,
                                (1, 2) => 5,
                                (0, 3) => 6,
                                (1, 3) => 7,
                                _ => unreachable!(),
                            };
                            pattern |= 1 << bit;
                        }
                    }
                }
            }

            patterns[cy * cell_width + cx] = pattern;
        }
    }

    patterns
}

fn generate_rust_code(name: &str, frames: &[BrailleFrame]) -> String {
    let mut code = String::new();

    code.push_str(&format!("// Auto-generated embedded animation: {}\n", name));
    code.push_str("// Do not edit manually - regenerate with convert_to_embedded tool\n\n");
    code.push_str("use crate::video::converter::BrailleFrame;\n\n");

    // Generate frame data as const arrays
    code.push_str(&format!("const FRAME_COUNT: usize = {};\n", frames.len()));
    code.push_str(&format!("const FRAME_WIDTH: usize = {};\n", frames[0].width));
    code.push_str(&format!("const FRAME_HEIGHT: usize = {};\n", frames[0].height));
    code.push_str(&format!("const FRAME_DURATION_MS: u32 = {};\n\n", frames[0].duration_ms));

    // Generate pattern data as a single large const array
    let total_patterns: usize = frames.iter().map(|f| f.patterns.len()).sum();
    code.push_str(&format!("const PATTERN_DATA: [u8; {}] = [\n", total_patterns));

    for (i, frame) in frames.iter().enumerate() {
        code.push_str(&format!("    // Frame {}\n    ", i));
        for (j, &pattern) in frame.patterns.iter().enumerate() {
            code.push_str(&format!("{:#04x},", pattern));
            if (j + 1) % 20 == 0 {
                code.push_str("\n    ");
            }
        }
        code.push_str("\n");
    }
    code.push_str("];\n\n");

    // Generate function to create frames
    let upper_name = name.to_uppercase();
    code.push_str(&format!("/// Get pre-converted frames for {} animation\n", name));
    code.push_str(&format!("pub fn get_{}_frames() -> Vec<BrailleFrame> {{\n", name));
    code.push_str("    let patterns_per_frame = FRAME_WIDTH * FRAME_HEIGHT;\n");
    code.push_str("    (0..FRAME_COUNT)\n");
    code.push_str("        .map(|i| {\n");
    code.push_str("            let start = i * patterns_per_frame;\n");
    code.push_str("            let end = start + patterns_per_frame;\n");
    code.push_str("            BrailleFrame {\n");
    code.push_str("                patterns: PATTERN_DATA[start..end].to_vec(),\n");
    code.push_str("                width: FRAME_WIDTH,\n");
    code.push_str("                height: FRAME_HEIGHT,\n");
    code.push_str("                duration_ms: FRAME_DURATION_MS,\n");
    code.push_str("            }\n");
    code.push_str("        })\n");
    code.push_str("        .collect()\n");
    code.push_str("}\n");

    code
}
