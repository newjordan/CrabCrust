//! Parser for Pin2DMD `.pac` animation packages.
//!
//! The header reader is usable today. Frame extraction is still a
//! placeholder while the packed TMNT layout is documented.

use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Pin2DMD `.pac` file header.
#[derive(Debug)]
pub struct PacHeader {
    /// Four-byte magic. Valid files start with `b"PAC "`.
    pub magic: [u8; 4],
    /// Minor version byte.
    pub version_minor: u8,
    /// Major version byte.
    pub version_major: u8,
    /// Format identifier stored after the version.
    pub format_type: u16,
    /// Bytes that follow the fixed header.
    pub data: Vec<u8>,
}

impl PacHeader {
    /// Parse header from .pac file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path.as_ref()).context("Failed to open .pac file")?;

        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;

        if &magic != b"PAC " {
            bail!(
                "Invalid .pac file: magic header mismatch (expected 'PAC ', got {:?})",
                String::from_utf8_lossy(&magic)
            );
        }

        let mut version_bytes = [0u8; 2];
        file.read_exact(&mut version_bytes)?;
        let version_minor = version_bytes[0];
        let version_major = version_bytes[1];

        let mut format_bytes = [0u8; 2];
        file.read_exact(&mut format_bytes)?;
        let format_type = u16::from_le_bytes(format_bytes);

        // Read remaining data
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        Ok(Self {
            magic,
            version_minor,
            version_major,
            format_type,
            data,
        })
    }

    /// Version string formatted as `major.minor`.
    pub fn version(&self) -> String {
        format!("{}.{}", self.version_major, self.version_minor)
    }
}

/// One colorized DMD frame.
#[derive(Debug, Clone)]
pub struct DmdFrame {
    /// Width in pixels.
    pub width: usize,
    /// Height in pixels.
    pub height: usize,
    /// Palette indices, row-major.
    pub pixels: Vec<u8>,
    /// RGB palette entries.
    pub palette: Vec<[u8; 3]>,
    /// How long this frame should stay on screen.
    pub duration_ms: u32,
}

impl DmdFrame {
    /// Standard DMD width, in pixels.
    pub const DMD_WIDTH: usize = 128;
    /// Standard DMD height, in pixels.
    pub const DMD_HEIGHT: usize = 32;

    /// Create new DMD frame
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height],
            palette: vec![[0, 0, 0]; 16], // Default 16-color palette
            duration_ms: 50,              // Default ~20 FPS
        }
    }

    /// Get pixel at position
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.pixels[y * self.width + x])
    }

    /// Set pixel at position
    pub fn set_pixel(&mut self, x: usize, y: usize, color_index: u8) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color_index;
        }
    }

    /// Get RGB color for pixel
    pub fn get_pixel_color(&self, x: usize, y: usize) -> Option<[u8; 3]> {
        let color_index = self.get_pixel(x, y)? as usize;
        self.palette.get(color_index).copied()
    }
}

/// A named sequence of DMD frames from a `.pac` file.
#[derive(Debug)]
pub struct PacAnimation {
    /// Clip name from the package, when the parser knows it.
    pub name: String,
    /// Frames in playback order.
    pub frames: Vec<DmdFrame>,
}

impl PacAnimation {
    /// Number of frames in the clip.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Sum of per-frame durations, in milliseconds.
    pub fn total_duration_ms(&self) -> u32 {
        self.frames.iter().map(|f| f.duration_ms).sum()
    }
}

/// Parse .pac file and extract animations
pub fn parse_pac_file<P: AsRef<Path>>(path: P) -> Result<Vec<PacAnimation>> {
    let header = PacHeader::from_file(&path)?;

    println!("📦 Pin2DMD .pac file");
    println!("   Version: {}", header.version());
    println!("   Format: 0x{:04x}", header.format_type);
    println!("   Data size: {} bytes", header.data.len());

    // TODO: Implement actual frame extraction
    // The .pac format can be compressed/encoded in various ways
    // Need to reverse engineer the specific TMNT pack format

    // For now, return an empty placeholder
    // This will be implemented once we understand the data structure

    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmd_frame() {
        let mut frame = DmdFrame::new(128, 32);
        assert_eq!(frame.width, 128);
        assert_eq!(frame.height, 32);

        frame.set_pixel(0, 0, 15);
        assert_eq!(frame.get_pixel(0, 0), Some(15));
    }
}
