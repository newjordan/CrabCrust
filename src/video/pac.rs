// Pin2DMD .pac file parser
//
// Parses Pin2DMD animation package files (.pac) containing
// colorized DMD (Dot Matrix Display) animations for pinball machines.
//
// Format: 128x32 pixel animations with color palette

use anyhow::{Context, Result, bail};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Pin2DMD .pac file header
#[derive(Debug)]
pub struct PacHeader {
    pub magic: [u8; 4],        // "PAC "
    pub version_minor: u8,
    pub version_major: u8,
    pub format_type: u16,
    pub data: Vec<u8>,
}

impl PacHeader {
    /// Parse header from .pac file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path.as_ref())
            .context("Failed to open .pac file")?;

        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;

        if &magic != b"PAC " {
            bail!("Invalid .pac file: magic header mismatch (expected 'PAC ', got {:?})",
                  String::from_utf8_lossy(&magic));
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

    pub fn version(&self) -> String {
        format!("{}.{}", self.version_major, self.version_minor)
    }
}

/// DMD Frame with color data
#[derive(Debug, Clone)]
pub struct DmdFrame {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,      // Raw pixel data
    pub palette: Vec<[u8; 3]>, // RGB color palette
    pub duration_ms: u32,
}

impl DmdFrame {
    /// Standard DMD size
    pub const DMD_WIDTH: usize = 128;
    pub const DMD_HEIGHT: usize = 32;

    /// Create new DMD frame
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height],
            palette: vec![[0, 0, 0]; 16], // Default 16-color palette
            duration_ms: 50, // Default ~20 FPS
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

/// Pin2DMD animation sequence
#[derive(Debug)]
pub struct PacAnimation {
    pub name: String,
    pub frames: Vec<DmdFrame>,
}

impl PacAnimation {
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

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
