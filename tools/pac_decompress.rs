// Pin2DMD .pac decompressor
// Attempts to decompress the payload using common algorithms

use std::env;
use std::fs::File;
use std::io::{Read, Write};

// Simple DEFLATE decompression attempt
fn try_deflate_decompress(data: &[u8]) -> Option<Vec<u8>> {
    use flate2::read::DeflateDecoder;
    use std::io::Read;

    let mut decoder = DeflateDecoder::new(data);
    let mut decompressed = Vec::new();

    match decoder.read_to_end(&mut decompressed) {
        Ok(_) if !decompressed.is_empty() => Some(decompressed),
        _ => None,
    }
}

// Try ZLIB decompression
fn try_zlib_decompress(data: &[u8]) -> Option<Vec<u8>> {
    use flate2::read::ZlibDecoder;
    use std::io::Read;

    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed = Vec::new();

    match decoder.read_to_end(&mut decompressed) {
        Ok(_) if !decompressed.is_empty() => Some(decompressed),
        _ => None,
    }
}

// Try GZIP decompression
fn try_gzip_decompress(data: &[u8]) -> Option<Vec<u8>> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();

    match decoder.read_to_end(&mut decompressed) {
        Ok(_) if !decompressed.is_empty() => Some(decompressed),
        _ => None,
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.pac> [output.bin]", args[0]);
        std::process::exit(1);
    }

    let pac_file = &args[1];
    let output_file = args.get(2).map(|s| s.as_str()).unwrap_or("decompressed.bin");

    println!("🔓 Attempting to decompress: {}\n", pac_file);

    // Read file
    let mut file = File::open(pac_file)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    println!("📦 Input:");
    println!("   Size: {} bytes", data.len());

    // Skip header (8 bytes)
    let payload = &data[8..];
    println!("   Payload: {} bytes", payload.len());

    // Try different decompression methods
    println!("\n🔬 Testing decompression methods...\n");

    // 1. Try raw DEFLATE
    print!("   DEFLATE: ");
    if let Some(decompressed) = try_deflate_decompress(payload) {
        println!("✅ Success! {} bytes", decompressed.len());

        let mut out = File::create(output_file)?;
        out.write_all(&decompressed)?;

        println!("\n✨ Decompressed data saved to: {}", output_file);
        println!("\n📊 Analysis:");
        println!("   Compressed: {} bytes", payload.len());
        println!("   Decompressed: {} bytes", decompressed.len());
        println!("   Ratio: {:.2}x", decompressed.len() as f64 / payload.len() as f64);

        // DMD frame analysis
        let frame_size = 4096; // 128x32 @ 8-bit
        let num_frames = decompressed.len() / frame_size;
        println!("\n🎬 Potential frames:");
        println!("   {} frames @ {} bytes each", num_frames, frame_size);
        println!("   Remainder: {} bytes", decompressed.len() % frame_size);

        return Ok(());
    } else {
        println!("❌ Failed");
    }

    // 2. Try ZLIB
    print!("   ZLIB: ");
    if let Some(decompressed) = try_zlib_decompress(payload) {
        println!("✅ Success! {} bytes", decompressed.len());

        let mut out = File::create(output_file)?;
        out.write_all(&decompressed)?;

        println!("\n✨ Decompressed data saved to: {}", output_file);
        return Ok(());
    } else {
        println!("❌ Failed");
    }

    // 3. Try GZIP
    print!("   GZIP: ");
    if let Some(decompressed) = try_gzip_decompress(payload) {
        println!("✅ Success! {} bytes", decompressed.len());

        let mut out = File::create(output_file)?;
        out.write_all(&decompressed)?;

        println!("\n✨ Decompressed data saved to: {}", output_file);
        return Ok(());
    } else {
        println!("❌ Failed");
    }

    println!("\n❌ All decompression methods failed.");
    println!("   This .pac file may use a custom compression format.");

    Ok(())
}
