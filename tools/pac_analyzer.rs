// Standalone Pin2DMD .pac file analyzer
// No dependencies on ffmpeg - just raw file parsing

use std::env;
use std::fs::File;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.pac>", args[0]);
        std::process::exit(1);
    }

    let pac_file = &args[1];
    println!("🔍 Analyzing Pin2DMD .pac file: {}\n", pac_file);

    let mut file = File::open(pac_file)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    println!("📦 File Information:");
    println!("   Size: {} bytes ({:.2} KB)", data.len(), data.len() as f64 / 1024.0);

    if data.len() < 8 {
        eprintln!("Error: File too small to be a valid .pac file");
        std::process::exit(1);
    }

    // Parse header
    let magic = &data[0..4];
    let version_minor = data[4];
    let version_major = data[5];
    let format_type = u16::from_le_bytes([data[6], data[7]]);

    println!("\n📋 Header:");
    println!("   Magic: {:?}", String::from_utf8_lossy(magic));
    println!("   Version: {}.{}", version_major, version_minor);
    println!("   Format Type: 0x{:04x}", format_type);

    // Analyze payload
    let payload = &data[8..];
    println!("\n📊 Payload:");
    println!("   Size: {} bytes", payload.len());

    // Show first 128 bytes
    println!("\n   First 128 bytes (hex):");
    for (i, chunk) in payload.chunks(16).take(8).enumerate() {
        print!("   {:04x}: ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        print!("  ");
        for byte in chunk {
            let c = if byte.is_ascii_graphic() || *byte == b' ' {
                *byte as char
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }

    // DMD frame analysis
    println!("\n🎯 DMD Frame Analysis:");
    println!("   Standard DMD: 128x32 pixels = 4096 pixels");

    let potential_formats = vec![
        (2048, "4-bit color", 16),
        (4096, "8-bit color", 256),
        (4096 + 64, "8-bit + 16-color palette", 16),
        (4096 + 768, "8-bit + 256-color palette", 256),
    ];

    for (frame_size, desc, colors) in potential_formats {
        let num_frames = payload.len() / frame_size;
        let remainder = payload.len() % frame_size;
        if num_frames > 0 && num_frames < 10000 {
            println!("   {} @ {} bytes/frame: {} frames, {} bytes remaining",
                     desc, frame_size, num_frames, remainder);
        }
    }

    // Look for repeating patterns (might indicate frame boundaries)
    println!("\n🔬 Pattern Detection:");

    // Check for common byte values (might be padding or delimiters)
    let mut byte_freq: Vec<usize> = vec![0; 256];
    for &byte in payload.iter() {
        byte_freq[byte as usize] += 1;
    }

    println!("   Most common bytes:");
    let mut sorted_freq: Vec<(u8, usize)> = byte_freq.iter()
        .enumerate()
        .map(|(b, &count)| (b as u8, count))
        .collect();
    sorted_freq.sort_by(|a, b| b.1.cmp(&a.1));

    for (byte, count) in sorted_freq.iter().take(10) {
        let percentage = (*count as f64 / payload.len() as f64) * 100.0;
        println!("      0x{:02x}: {:6} times ({:5.2}%)", byte, count, percentage);
    }

    // Entropy analysis
    let entropy: f64 = byte_freq.iter()
        .map(|&count| {
            if count == 0 {
                0.0
            } else {
                let p = count as f64 / payload.len() as f64;
                -p * p.log2()
            }
        })
        .sum();

    println!("\n   Data entropy: {:.2} bits/byte", entropy);
    if entropy > 7.0 {
        println!("   (High entropy - likely compressed or encrypted)");
    } else if entropy > 5.0 {
        println!("   (Medium entropy - possibly compressed image data)");
    } else {
        println!("   (Low entropy - likely uncompressed or structured data)");
    }

    println!("\n✨ Analysis complete!");
    println!("\n💡 Next steps:");
    println!("   1. Try different frame sizes to find the correct format");
    println!("   2. Look for color palette data in the header");
    println!("   3. Check Pin2DMD documentation for exact format specs");

    Ok(())
}
