// Matrix Rain Animation - EXPERIMENTAL / NON-FUNCTIONAL
//
// ⚠️ WARNING: This implementation does not work as intended.
//
// PROBLEM: Braille rendering is designed for graphics (dots), not text (characters).
// Matrix rain requires actual character glyphs (ﾊﾐﾋ, O, A, etc.), but BrailleGrid
// only renders dot patterns. The result is unreadable garbled output.
//
// See docs/MATRIX_RAIN_POSTMORTEM.md for full explanation and alternatives.
//
// This code is kept for reference but should not be used in production.
// A proper implementation would require a character-based rendering system,
// which is outside the scope of CrabCrust's Braille-focused architecture.
//
// Original concept: Matrix rain that decodes output text through cascading
// digital rain, with random characters progressively revealing real text.
// Chaos → Order (but needs character rendering, not Braille dots).

use crate::animation::Animation;
use crate::braille::BrailleGrid;
use std::time::Duration;

/// Matrix rain that decodes output text
pub struct MatrixRainAnimation {
    /// Lines of text to decode (the real output)
    output_lines: Vec<String>,

    /// Rain columns (one per character position)
    columns: Vec<RainColumn>,

    /// Animation elapsed time
    elapsed: Duration,

    /// Total duration to run
    duration: Duration,

    /// Decode threshold (0.0-1.0) - when to reveal real char
    decode_threshold: f32,

    /// Speed variation factor (0.0-1.0) - higher = more chaos
    noise_factor: f32,

    /// Grid dimensions
    width: usize,
    height: usize,

    /// All columns finished?
    all_decoded: bool,
}

/// Single column of falling Matrix rain
struct RainColumn {
    /// Column X position
    x: usize,

    /// Target row (where this column should end up)
    target_y: usize,

    /// Real character to reveal
    target_char: char,

    /// Current fall progress (0.0 = top, 1.0 = target reached)
    progress: f32,

    /// Fall speed (cells per second) - randomized
    speed: f32,

    /// Has this column decoded?
    decoded: bool,

    /// Trail length (characters above current position)
    trail_length: usize,
}

impl MatrixRainAnimation {
    /// Matrix character pool (Katakana + symbols)
    const MATRIX_CHARS: &'static [char] = &[
        'ﾊ', 'ﾐ', 'ﾋ', 'ｰ', 'ｳ', 'ｼ', 'ﾅ', 'ﾓ', 'ﾆ', 'ｻ', 'ﾜ', 'ﾂ', 'ｵ', 'ﾘ', 'ｱ',
        'ﾎ', 'ﾃ', 'ﾏ', 'ｹ', 'ﾒ', 'ｴ', 'ｶ', 'ｷ', 'ﾑ', 'ﾕ', 'ﾗ', 'ｾ', 'ﾈ', 'ｽ', 'ﾀ',
        'ﾇ', 'ﾍ', ':', '・', '.', '"', '=', '*', '+', '-', '<', '>', '¦', '|',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    ];

    /// Create new Matrix rain animation that decodes output text
    pub fn new(output: String) -> Self {
        Self::with_params(output, Duration::from_secs(3), 0.7, 0.5)
    }

    /// Create with custom parameters
    pub fn with_params(
        output: String,
        duration: Duration,
        decode_threshold: f32,
        noise_factor: f32,
    ) -> Self {
        let output_lines: Vec<String> = output.lines().map(|s| s.to_string()).collect();

        // Calculate grid size needed
        let width = output_lines.iter().map(|s| s.len()).max().unwrap_or(80);
        let height = output_lines.len();

        // Create a column for each character position
        let mut columns = Vec::new();

        for (y, line) in output_lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                // Random speed with noise variation
                let base_speed = 15.0; // cells per second
                let noise = (Self::pseudo_random(x + y * 1000) as f32 / u32::MAX as f32) * 2.0 - 1.0;
                let speed = base_speed * (1.0 + noise * noise_factor);

                // Random trail length (3-8 chars)
                let trail_length = 3 + (Self::pseudo_random(x * 7 + y * 13) % 6) as usize;

                columns.push(RainColumn {
                    x,
                    target_y: y,
                    target_char: ch,
                    progress: 0.0,
                    speed,
                    decoded: false,
                    trail_length,
                });
            }
        }

        Self {
            output_lines,
            columns,
            elapsed: Duration::ZERO,
            duration,
            decode_threshold,
            noise_factor,
            width,
            height,
            all_decoded: false,
        }
    }

    /// Get a random Matrix character
    fn random_matrix_char(seed: usize) -> char {
        let idx = Self::pseudo_random(seed) as usize % Self::MATRIX_CHARS.len();
        Self::MATRIX_CHARS[idx]
    }

    /// Simple pseudo-random number generator (for consistency)
    fn pseudo_random(seed: usize) -> u32 {
        let mut x = seed as u32;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        x
    }

    /// Convert character to Braille dots (simple mapping)
    fn char_to_braille_pattern(ch: char) -> u8 {
        // Simple mapping - for Matrix effect we'll use patterns
        match ch {
            ' ' => 0x00,
            _ => {
                // Use character code to generate pseudo-random pattern
                let code = ch as u32;
                ((code * 31337) % 256) as u8
            }
        }
    }
}

impl Animation for MatrixRainAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;

        // Update each column
        let dt_secs = delta_time.as_secs_f32();
        let mut all_decoded = true;

        for column in &mut self.columns {
            if !column.decoded {
                // Advance progress
                column.progress += column.speed * dt_secs / self.height as f32;

                // Check if we should decode
                if column.progress >= self.decode_threshold {
                    column.decoded = true;
                } else {
                    all_decoded = false;
                }
            }
        }

        self.all_decoded = all_decoded;

        // Continue until duration elapsed or all decoded
        self.elapsed < self.duration && !self.all_decoded
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let cell_width = grid.width();
        let cell_height = grid.height();

        // Clear grid
        grid.clear();

        // Render each column
        for column in &self.columns {
            // Calculate current Y position in grid cells
            let current_y = (column.progress * column.target_y as f32).min(column.target_y as f32);

            // Skip if column hasn't entered visible area yet
            if current_y < 0.0 {
                continue;
            }

            let y_cell = current_y as usize;

            // Bounds check
            if column.x >= cell_width || y_cell >= cell_height {
                continue;
            }

            if column.decoded {
                // Show the real character (decoded)
                let pattern = Self::char_to_braille_pattern(column.target_char);

                // Set dots based on pattern
                for bit in 0..8 {
                    if (pattern & (1 << bit)) != 0 {
                        let (dx, dy) = match bit {
                            0 => (0, 0),
                            1 => (0, 1),
                            2 => (0, 2),
                            3 => (1, 0),
                            4 => (1, 1),
                            5 => (1, 2),
                            6 => (0, 3),
                            7 => (1, 3),
                            _ => unreachable!(),
                        };

                        let dot_x = column.x * 2 + dx;
                        let dot_y = column.target_y * 4 + dy;

                        if dot_x < grid.dot_width() && dot_y < grid.dot_height() {
                            grid.set_dot(dot_x, dot_y);
                        }
                    }
                }
            } else {
                // Show Matrix rain (random characters in trail)
                for trail_offset in 0..column.trail_length {
                    let trail_y = y_cell.saturating_sub(trail_offset);

                    if trail_y >= cell_height {
                        continue;
                    }

                    // Get random Matrix character
                    let seed = column.x * 1000 + trail_y * 100 + self.elapsed.as_millis() as usize / 50;
                    let matrix_char = Self::random_matrix_char(seed);
                    let pattern = Self::char_to_braille_pattern(matrix_char);

                    // Fade out trail (brighter at head)
                    let brightness = 1.0 - (trail_offset as f32 / column.trail_length as f32);

                    // Only render if bright enough
                    if brightness > 0.2 {
                        for bit in 0..8 {
                            if (pattern & (1 << bit)) != 0 {
                                // Random chance to show dot based on brightness
                                let show_seed = seed + bit;
                                if (Self::pseudo_random(show_seed) as f32 / u32::MAX as f32) < brightness {
                                    let (dx, dy) = match bit {
                                        0 => (0, 0),
                                        1 => (0, 1),
                                        2 => (0, 2),
                                        3 => (1, 0),
                                        4 => (1, 1),
                                        5 => (1, 2),
                                        6 => (0, 3),
                                        7 => (1, 3),
                                        _ => unreachable!(),
                                    };

                                    let dot_x = column.x * 2 + dx;
                                    let dot_y = trail_y * 4 + dy;

                                    if dot_x < grid.dot_width() && dot_y < grid.dot_height() {
                                        grid.set_dot(dot_x, dot_y);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn name(&self) -> &str {
        "Matrix Rain"
    }
}

impl Default for MatrixRainAnimation {
    fn default() -> Self {
        Self::new("The Matrix has you...".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_rain_creation() {
        let output = "On branch main\nYour branch is up to date".to_string();
        let anim = MatrixRainAnimation::new(output);

        assert_eq!(anim.output_lines.len(), 2);
        assert!(anim.columns.len() > 0);
    }

    #[test]
    fn test_matrix_rain_decode() {
        let output = "Test".to_string();
        let mut anim = MatrixRainAnimation::with_params(
            output,
            Duration::from_secs(1),
            0.5,  // Decode at 50%
            0.1,  // Low noise
        );

        // Initial state - nothing decoded
        let initial_decoded = anim.columns.iter().filter(|c| c.decoded).count();
        assert_eq!(initial_decoded, 0);

        // Update enough to decode
        anim.update(Duration::from_secs(1));

        // Should have some decoded columns
        let decoded = anim.columns.iter().filter(|c| c.decoded).count();
        assert!(decoded > 0);
    }
}
