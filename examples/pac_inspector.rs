// Pin2DMD .pac file inspector
// Examines .pac file structure and attempts to extract animations

use anyhow::Result;
use std::env;

#[cfg(feature = "video")]
use crabcrust::video::pac::{PacHeader, parse_pac_file};

fn main() -> Result<()> {
    #[cfg(not(feature = "video"))]
    {
        eprintln!("Error: This example requires the 'video' feature");
        eprintln!("Build with: cargo run --example pac_inspector --features video <file.pac>");
        std::process::exit(1);
    }

    #[cfg(feature = "video")]
    {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: {} <file.pac>", args[0]);
            std::process::exit(1);
        }

        let pac_file = &args[1];
        println!("🔍 Inspecting Pin2DMD .pac file: {}\n", pac_file);

        // Parse header
        let header = PacHeader::from_file(pac_file)?;

        println!("📦 Header Information:");
        println!("   Magic: {:?}", String::from_utf8_lossy(&header.magic));
        println!("   Version: {}", header.version());
        println!("   Format Type: 0x{:04x}", header.format_type);
        println!("   Data Size: {} bytes ({:.2} KB)", header.data.len(), header.data.len() as f64 / 1024.0);

        // Analyze data structure
        println!("\n🔬 Data Analysis:");
        println!("   First 64 bytes (hex):");
        for (i, chunk) in header.data.chunks(16).take(4).enumerate() {
            print!("   {:04x}: ", i * 16);
            for byte in chunk {
                print!("{:02x} ", byte);
            }
            println!();
        }

        // Look for patterns
        println!("\n🎯 Searching for patterns...");

        // DMD dimensions are typically 128x32 = 4096 pixels
        // With 4-bit color (16 colors), that's 2048 bytes per frame
        // With 8-bit color, that's 4096 bytes per frame

        let potential_frame_sizes = vec![
            (2048, "4-bit color (128x32)"),
            (4096, "8-bit color (128x32)"),
            (8192, "8-bit color with metadata"),
        ];

        for (size, desc) in potential_frame_sizes {
            let num_frames = header.data.len() / size;
            if num_frames > 0 && num_frames < 1000 {
                println!("   Possible: {} frames @ {} bytes each ({})", num_frames, size, desc);
            }
        }

        // Try to parse animations
        println!("\n📽️  Attempting to parse animations...");
        match parse_pac_file(pac_file) {
            Ok(animations) => {
                println!("   Found {} animations", animations.len());
                for (i, anim) in animations.iter().enumerate() {
                    println!("   Animation {}: \"{}\" ({} frames, {:.2}s)",
                             i + 1,
                             anim.name,
                             anim.frame_count(),
                             anim.total_duration_ms() as f64 / 1000.0);
                }
            }
            Err(e) => {
                println!("   ⚠️  Parser not yet implemented: {}", e);
                println!("   (This .pac format needs reverse engineering)");
            }
        }

        println!("\n✨ Inspection complete!");
    }

    Ok(())
}
