// Animation Calibration Tool with Dotmax Rendering
// Run with: cargo run --features video --bin calibrate_animation -- <video_file>
//
// Controls:
//   Q/Esc      - Quit
//   Space      - Pause/Resume
//   Up/Down    - Adjust threshold (+/- 10)
//   Left/Right - Adjust threshold (+/- 1)
//   PgUp/PgDn  - Adjust threshold (+/- 25)
//   I          - Invert colors
//   D          - Cycle dithering mode (None -> Floyd-Steinberg -> Ordered -> Atkinson)
//   C          - Cycle color scheme
//   R          - Reset to defaults
//   S          - Save current settings and regenerate embedded file
//   +/-        - Adjust playback speed

use anyhow::{Context, Result};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{self, ClearType},
};
use std::env;
use std::fs::File;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <video_file> [output_name]", args[0]);
        eprintln!("Example: {} ref/computer_fingers.mp4 computer_fingers", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_name = args.get(2).map(|s| s.as_str()).unwrap_or("calibrated");

    // Initialize FFmpeg
    ffmpeg_next::init().context("Failed to initialize FFmpeg")?;

    // Default calibration settings
    let mut settings = CalibrationSettings {
        threshold: 128,
        width: 124,
        height: 19,
        invert: false,
        dither_mode: DitherMode::FloydSteinberg,
        color_scheme: 0,
        speed: 1.0,
        paused: false,
        gamma: 1.0,
        contrast: 1.0,
    };

    // Enter raw mode for keyboard input
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = run_calibration(&mut stdout, input_path, output_name, &mut settings);

    // Cleanup
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    result
}

#[derive(Clone, Copy, PartialEq)]
enum DitherMode {
    None,
    FloydSteinberg,
    Ordered,
    Atkinson,
}

impl DitherMode {
    fn name(&self) -> &'static str {
        match self {
            DitherMode::None => "None",
            DitherMode::FloydSteinberg => "Floyd-Steinberg",
            DitherMode::Ordered => "Ordered 4x4",
            DitherMode::Atkinson => "Atkinson",
        }
    }

    fn next(&self) -> DitherMode {
        match self {
            DitherMode::None => DitherMode::FloydSteinberg,
            DitherMode::FloydSteinberg => DitherMode::Ordered,
            DitherMode::Ordered => DitherMode::Atkinson,
            DitherMode::Atkinson => DitherMode::None,
        }
    }
}

struct CalibrationSettings {
    threshold: u8,
    width: usize,
    height: usize,
    invert: bool,
    dither_mode: DitherMode,
    color_scheme: usize,
    speed: f32,
    paused: bool,
    gamma: f32,
    contrast: f32,
}

fn run_calibration(
    stdout: &mut std::io::Stdout,
    input_path: &str,
    output_name: &str,
    settings: &mut CalibrationSettings,
) -> Result<()> {
    let mut frames = convert_video(input_path, settings)?;
    let mut current_frame = 0;
    let mut last_frame_time = Instant::now();
    let mut needs_reconvert = false;

    loop {
        // Handle input - only process key press events, not release or repeat
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                // Skip key release and repeat events to prevent multiple triggers
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => settings.paused = !settings.paused,
                    KeyCode::Up => {
                        settings.threshold = settings.threshold.saturating_add(10);
                        needs_reconvert = true;
                    }
                    KeyCode::Down => {
                        settings.threshold = settings.threshold.saturating_sub(10);
                        needs_reconvert = true;
                    }
                    KeyCode::Right => {
                        settings.threshold = settings.threshold.saturating_add(1);
                        needs_reconvert = true;
                    }
                    KeyCode::Left => {
                        settings.threshold = settings.threshold.saturating_sub(1);
                        needs_reconvert = true;
                    }
                    KeyCode::PageUp => {
                        settings.threshold = settings.threshold.saturating_add(25);
                        needs_reconvert = true;
                    }
                    KeyCode::PageDown => {
                        settings.threshold = settings.threshold.saturating_sub(25);
                        needs_reconvert = true;
                    }
                    KeyCode::Char('i') => {
                        settings.invert = !settings.invert;
                        needs_reconvert = true;
                    }
                    KeyCode::Char('d') => {
                        settings.dither_mode = settings.dither_mode.next();
                        needs_reconvert = true;
                    }
                    KeyCode::Char('c') => {
                        settings.color_scheme = (settings.color_scheme + 1) % 5;
                        needs_reconvert = true;
                    }
                    KeyCode::Char('g') => {
                        settings.gamma = (settings.gamma + 0.1).min(3.0);
                        needs_reconvert = true;
                    }
                    KeyCode::Char('G') => {
                        settings.gamma = (settings.gamma - 0.1).max(0.1);
                        needs_reconvert = true;
                    }
                    KeyCode::Char('k') => {
                        settings.contrast = (settings.contrast + 0.1).min(3.0);
                        needs_reconvert = true;
                    }
                    KeyCode::Char('K') => {
                        settings.contrast = (settings.contrast - 0.1).max(0.1);
                        needs_reconvert = true;
                    }
                    KeyCode::Char('r') => {
                        settings.threshold = 128;
                        settings.invert = false;
                        settings.dither_mode = DitherMode::FloydSteinberg;
                        settings.speed = 1.0;
                        settings.gamma = 1.0;
                        settings.contrast = 1.0;
                        needs_reconvert = true;
                    }
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        settings.speed = (settings.speed + 0.25).min(4.0);
                    }
                    KeyCode::Char('-') => {
                        settings.speed = (settings.speed - 0.25).max(0.25);
                    }
                    KeyCode::Char('s') => {
                        // Save settings
                        save_embedded_file(input_path, output_name, settings)?;
                        show_message(stdout, &format!("Saved to src/animation/embedded_{}.rs", output_name))?;
                        std::thread::sleep(Duration::from_secs(2));
                    }
                    _ => {}
                }
            }
        }

        // Reconvert if settings changed
        if needs_reconvert {
            frames = convert_video(input_path, settings)?;
            needs_reconvert = false;
        }

        // Update frame
        if !settings.paused && !frames.is_empty() {
            let frame_duration = Duration::from_millis(
                (frames[current_frame].duration_ms as f32 / settings.speed) as u64
            );

            if last_frame_time.elapsed() >= frame_duration {
                current_frame = (current_frame + 1) % frames.len();
                last_frame_time = Instant::now();
            }
        }

        // Render
        render_frame(stdout, &frames, current_frame, settings)?;
    }

    Ok(())
}

struct BrailleFrame {
    patterns: Vec<u8>,
    width: usize,
    height: usize,
    duration_ms: u32,
}

fn convert_video(path: &str, settings: &CalibrationSettings) -> Result<Vec<BrailleFrame>> {
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
        (settings.width * 2) as u32,
        (settings.height * 4) as u32,
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
                          scaler: &mut ffmpeg::software::scaling::Context,
                          settings: &CalibrationSettings| -> Result<()> {
        let mut decoded = ffmpeg::util::frame::video::Video::empty();
        while decoder.receive_frame(&mut decoded).is_ok() {
            let mut gray_frame = ffmpeg::util::frame::video::Video::empty();
            scaler.run(&decoded, &mut gray_frame)
                .context("Failed to scale frame")?;

            // Get the actual stride (bytes per row) - may be larger than width due to alignment
            let stride = gray_frame.stride(0);
            let data = gray_frame.data(0);
            let target_width = settings.width * 2;
            let target_height = settings.height * 4;

            // Copy data respecting stride to get correct image
            let mut corrected_data = Vec::with_capacity(target_width * target_height);
            for y in 0..target_height {
                let row_start = y * stride;
                for x in 0..target_width {
                    if row_start + x < data.len() {
                        corrected_data.push(data[row_start + x]);
                    } else {
                        corrected_data.push(0);
                    }
                }
            }

            let patterns = convert_to_braille_dotmax(&corrected_data, settings);

            braille_frames.push(BrailleFrame {
                patterns,
                width: settings.width,
                height: settings.height,
                duration_ms: frame_duration_ms,
            });
        }
        Ok(())
    };

    for (stream, packet) in input.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).context("Failed to send packet")?;
            process_frames(&mut decoder, &mut braille_frames, &mut scaler, settings)?;
        }
    }

    decoder.send_eof().context("Failed to send EOF")?;
    process_frames(&mut decoder, &mut braille_frames, &mut scaler, settings)?;

    Ok(braille_frames)
}

/// Convert grayscale image data to braille patterns using dotmax-style processing
fn convert_to_braille_dotmax(gray_data: &[u8], settings: &CalibrationSettings) -> Vec<u8> {
    let img_width = settings.width * 2;
    let img_height = settings.height * 4;

    // Pre-process: apply gamma and contrast
    let processed: Vec<u8> = gray_data.iter().map(|&p| {
        let mut val = p as f32 / 255.0;

        // Apply gamma correction
        val = val.powf(settings.gamma);

        // Apply contrast
        val = ((val - 0.5) * settings.contrast + 0.5).clamp(0.0, 1.0);

        // Invert if needed
        if settings.invert {
            val = 1.0 - val;
        }

        (val * 255.0) as u8
    }).collect();

    // Apply dithering based on mode
    let dithered = match settings.dither_mode {
        DitherMode::None => threshold_only(&processed, img_width, img_height, settings.threshold),
        DitherMode::FloydSteinberg => floyd_steinberg_dither(&processed, img_width, img_height, settings.threshold),
        DitherMode::Ordered => ordered_dither(&processed, img_width, img_height, settings.threshold),
        DitherMode::Atkinson => atkinson_dither(&processed, img_width, img_height, settings.threshold),
    };

    // Convert dithered binary image to braille patterns
    let mut patterns = vec![0u8; settings.width * settings.height];

    for cy in 0..settings.height {
        for cx in 0..settings.width {
            let mut pattern: u8 = 0;

            for dy in 0..4 {
                for dx in 0..2 {
                    let px = cx * 2 + dx;
                    let py = cy * 4 + dy;

                    if px < img_width && py < img_height {
                        if dithered[py * img_width + px] {
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

            patterns[cy * settings.width + cx] = pattern;
        }
    }

    patterns
}

/// Simple threshold (no dithering)
fn threshold_only(data: &[u8], width: usize, height: usize, threshold: u8) -> Vec<bool> {
    data.iter().map(|&p| p < threshold).collect()
}

/// Floyd-Steinberg dithering
fn floyd_steinberg_dither(data: &[u8], width: usize, height: usize, threshold: u8) -> Vec<bool> {
    let mut errors: Vec<f32> = data.iter().map(|&p| p as f32).collect();
    let mut result = vec![false; width * height];
    let threshold_f = threshold as f32;

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let old_pixel = errors[idx];
            let new_pixel = if old_pixel < threshold_f { 0.0 } else { 255.0 };
            result[idx] = new_pixel < 128.0;
            let error = old_pixel - new_pixel;

            // Distribute error to neighbors
            if x + 1 < width {
                errors[idx + 1] += error * 7.0 / 16.0;
            }
            if y + 1 < height {
                if x > 0 {
                    errors[idx + width - 1] += error * 3.0 / 16.0;
                }
                errors[idx + width] += error * 5.0 / 16.0;
                if x + 1 < width {
                    errors[idx + width + 1] += error * 1.0 / 16.0;
                }
            }
        }
    }

    result
}

/// Ordered dithering with 4x4 Bayer matrix
fn ordered_dither(data: &[u8], width: usize, height: usize, threshold: u8) -> Vec<bool> {
    // 4x4 Bayer matrix (normalized to 0-255 range)
    const BAYER: [[u8; 4]; 4] = [
        [  0, 128,  32, 160],
        [192,  64, 224,  96],
        [ 48, 176,  16, 144],
        [240, 112, 208,  80],
    ];

    let mut result = vec![false; width * height];
    let base_threshold = threshold as i16;

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let pixel = data[idx] as i16;
            let bayer_val = BAYER[y % 4][x % 4] as i16 - 128;
            let adjusted_threshold = (base_threshold + bayer_val / 2).clamp(0, 255) as u8;
            result[idx] = pixel < adjusted_threshold as i16;
        }
    }

    result
}

/// Atkinson dithering (good for high contrast)
fn atkinson_dither(data: &[u8], width: usize, height: usize, threshold: u8) -> Vec<bool> {
    let mut errors: Vec<f32> = data.iter().map(|&p| p as f32).collect();
    let mut result = vec![false; width * height];
    let threshold_f = threshold as f32;

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let old_pixel = errors[idx];
            let new_pixel = if old_pixel < threshold_f { 0.0 } else { 255.0 };
            result[idx] = new_pixel < 128.0;
            let error = (old_pixel - new_pixel) / 8.0; // Atkinson only distributes 6/8 of error

            // Distribute error to neighbors (Atkinson pattern)
            if x + 1 < width {
                errors[idx + 1] += error;
            }
            if x + 2 < width {
                errors[idx + 2] += error;
            }
            if y + 1 < height {
                if x > 0 {
                    errors[idx + width - 1] += error;
                }
                errors[idx + width] += error;
                if x + 1 < width {
                    errors[idx + width + 1] += error;
                }
            }
            if y + 2 < height {
                errors[idx + width * 2] += error;
            }
        }
    }

    result
}

fn render_frame(
    stdout: &mut std::io::Stdout,
    frames: &[BrailleFrame],
    current_frame: usize,
    settings: &CalibrationSettings,
) -> Result<()> {
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // Header with controls
    writeln!(stdout, "=== Animation Calibration Tool (Dotmax Enhanced) ===\r")?;
    writeln!(stdout, "Threshold: {:3} | Dither: {:15} | Gamma: {:.1} | Contrast: {:.1}\r",
        settings.threshold,
        settings.dither_mode.name(),
        settings.gamma,
        settings.contrast,
    )?;
    writeln!(stdout, "Invert: {:5} | Speed: {:.2}x | Frame: {}/{}\r",
        if settings.invert { "ON" } else { "OFF" },
        settings.speed,
        current_frame + 1,
        frames.len()
    )?;
    writeln!(stdout, "Keys: Arrows=threshold D=dither I=invert g/G=gamma k/K=contrast S=save Q=quit\r")?;
    writeln!(stdout, "{}\r", "─".repeat(80))?;

    // Render current frame
    if !frames.is_empty() {
        let frame = &frames[current_frame];
        for y in 0..frame.height {
            for x in 0..frame.width {
                let pattern = frame.patterns[y * frame.width + x];
                let ch = char::from_u32(0x2800 + pattern as u32).unwrap_or(' ');
                write!(stdout, "{}", ch)?;
            }
            writeln!(stdout, "\r")?;
        }
    }

    // Status line
    let status = if settings.paused { "PAUSED" } else { "PLAYING" };
    writeln!(stdout, "{}\r", "─".repeat(80))?;
    writeln!(stdout, "Status: {} | Space=pause/play\r", status)?;

    stdout.flush()?;
    Ok(())
}

fn show_message(stdout: &mut std::io::Stdout, msg: &str) -> Result<()> {
    let (_, rows) = terminal::size()?;
    execute!(stdout, cursor::MoveTo(0, rows - 2))?;
    writeln!(stdout, ">>> {} <<<\r", msg)?;
    stdout.flush()?;
    Ok(())
}

fn save_embedded_file(input_path: &str, output_name: &str, settings: &CalibrationSettings) -> Result<()> {
    let frames = convert_video(input_path, settings)?;
    let rust_code = generate_rust_code(output_name, &frames, settings);

    let output_path = format!("src/animation/embedded_{}.rs", output_name);
    let mut file = File::create(&output_path)?;
    file.write_all(rust_code.as_bytes())?;

    Ok(())
}

fn generate_rust_code(name: &str, frames: &[BrailleFrame], settings: &CalibrationSettings) -> String {
    let mut code = String::new();

    code.push_str(&format!("// Auto-generated embedded animation: {}\n", name));
    code.push_str("// Do not edit manually - regenerate with calibrate_animation tool\n");
    code.push_str(&format!("// Settings: threshold={}, dither={}, gamma={:.1}, contrast={:.1}, invert={}\n\n",
        settings.threshold, settings.dither_mode.name(), settings.gamma, settings.contrast, settings.invert));
    code.push_str("use crate::video::converter::BrailleFrame;\n\n");

    code.push_str(&format!("const FRAME_COUNT: usize = {};\n", frames.len()));
    code.push_str(&format!("const FRAME_WIDTH: usize = {};\n", frames[0].width));
    code.push_str(&format!("const FRAME_HEIGHT: usize = {};\n", frames[0].height));
    code.push_str(&format!("const FRAME_DURATION_MS: u32 = {};\n\n", frames[0].duration_ms));

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
