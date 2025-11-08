// Create a test 128x32 DMD animation for validating CrabCrust conversion pipeline
// Classic pinball DMD style with "JACKPOT" animation

use std::fs::File;
use std::io::BufWriter;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 32;
const FRAMES: usize = 12;

fn create_frame(frame_num: usize) -> Vec<u8> {
    let mut pixels = vec![0u8; (WIDTH * HEIGHT * 3) as usize];

    // Classic DMD orange color
    let color = [255u8, 128u8, 0u8];

    // Animation phases
    if frame_num < 3 {
        // Flash full screen on even frames
        if frame_num % 2 == 0 {
            for chunk in pixels.chunks_mut(3) {
                chunk.copy_from_slice(&color);
            }
        }
    } else if frame_num < 6 {
        // Draw border and "JACKPOT" text (simplified - just border for now)
        // Top and bottom borders
        for x in 0..WIDTH {
            for y in 0..2 {
                let idx = ((y * WIDTH + x) * 3) as usize;
                pixels[idx..idx + 3].copy_from_slice(&color);
            }
            for y in (HEIGHT - 2)..HEIGHT {
                let idx = ((y * WIDTH + x) * 3) as usize;
                pixels[idx..idx + 3].copy_from_slice(&color);
            }
        }
        // Left and right borders
        for y in 0..HEIGHT {
            for x in 0..2 {
                let idx = ((y * WIDTH + x) * 3) as usize;
                pixels[idx..idx + 3].copy_from_slice(&color);
            }
            for x in (WIDTH - 2)..WIDTH {
                let idx = ((y * WIDTH + x) * 3) as usize;
                pixels[idx..idx + 3].copy_from_slice(&color);
            }
        }

        // Draw simple "JACKPOT" pattern (simplified vertical bars)
        let bars = [20, 35, 50, 65, 80, 95, 110];
        for &bar_x in &bars {
            for y in 12..20 {
                for x in bar_x..(bar_x + 3).min(WIDTH) {
                    let idx = ((y * WIDTH + x) * 3) as usize;
                    pixels[idx..idx + 3].copy_from_slice(&color);
                }
            }
        }
    } else {
        // Sparkle animation - border plus random dots
        // Draw border
        for x in 0..WIDTH {
            for y in 0..2 {
                let idx = ((y * WIDTH + x) * 3) as usize;
                pixels[idx..idx + 3].copy_from_slice(&color);
            }
            for y in (HEIGHT - 2)..HEIGHT {
                let idx = ((y * WIDTH + x) * 3) as usize;
                pixels[idx..idx + 3].copy_from_slice(&color);
            }
        }
        for y in 0..HEIGHT {
            for x in 0..2 {
                let idx = ((y * WIDTH + x) * 3) as usize;
                pixels[idx..idx + 3].copy_from_slice(&color);
            }
            for x in (WIDTH - 2)..WIDTH {
                let idx = ((y * WIDTH + x) * 3) as usize;
                pixels[idx..idx + 3].copy_from_slice(&color);
            }
        }

        // Text bars
        let bars = [20, 35, 50, 65, 80, 95, 110];
        for &bar_x in &bars {
            for y in 12..20 {
                for x in bar_x..(bar_x + 3).min(WIDTH) {
                    let idx = ((y * WIDTH + x) * 3) as usize;
                    pixels[idx..idx + 3].copy_from_slice(&color);
                }
            }
        }

        // Add sparkles (pseudo-random based on frame number)
        let seed = frame_num * 123456789;
        for i in 0..15 {
            let x = ((seed + i * 9876) % WIDTH as usize) as u32;
            let y = ((seed + i * 6789) % HEIGHT as usize) as u32;
            if x >= 4 && x < WIDTH - 4 && y >= 4 && y < HEIGHT - 4 {
                let idx = ((y * WIDTH + x) * 3) as usize;
                pixels[idx..idx + 3].copy_from_slice(&color);
            }
        }
    }

    pixels
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎰 Creating test DMD animation (128x32)...");

    let output_path = "/home/user/CrabCrust/ref/test_jackpot_dmd.gif";

    // Create frames
    let mut frames = Vec::new();
    for i in 0..FRAMES {
        println!("   Frame {}/{}", i + 1, FRAMES);
        let pixels = create_frame(i);
        frames.push(pixels);
    }

    // For now, just save the first frame as a simple PPM
    // (GIF encoding requires external library - let's use a simpler format first)
    let ppm_path = "/home/user/CrabCrust/ref/test_jackpot_dmd.ppm";
    let file = File::create(ppm_path)?;
    let mut writer = BufWriter::new(file);

    use std::io::Write;
    write!(writer, "P6\n{} {}\n255\n", WIDTH, HEIGHT)?;
    writer.write_all(&frames[5])?; // Use middle frame

    println!("\n✨ Created: {}", ppm_path);
    println!("   Dimensions: {}x{}", WIDTH, HEIGHT);
    println!("   Format: PPM (will convert to GIF)");

    Ok(())
}
