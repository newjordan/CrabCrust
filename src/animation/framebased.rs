// Frame-based animation for pre-rendered content
//
// This animation type plays back pre-converted frames, perfect for
// DMD animations, video clips, and other pre-rendered content.

use crate::animation::Animation;
use crate::braille::BrailleGrid;
use std::time::Duration;

#[cfg(any(feature = "gif", feature = "video"))]
use crate::video::converter::BrailleFrame;

/// Animation that plays pre-rendered frames
pub struct FrameBasedAnimation {
    frames: Vec<FrameData>,
    current_frame: usize,
    elapsed: Duration,
    loop_animation: bool,
    finished: bool,
}

/// Internal frame data
struct FrameData {
    patterns: Vec<u8>,
    width: usize,
    height: usize,
    duration: Duration,
}

impl FrameBasedAnimation {
    /// Create a new frame-based animation
    pub fn new(loop_animation: bool) -> Self {
        Self {
            frames: Vec::new(),
            current_frame: 0,
            elapsed: Duration::ZERO,
            loop_animation,
            finished: false,
        }
    }

    /// Add a frame manually
    pub fn add_frame(&mut self, patterns: Vec<u8>, width: usize, height: usize, duration: Duration) {
        self.frames.push(FrameData {
            patterns,
            width,
            height,
            duration,
        });
    }

    /// Load frames from converted video/GIF frames
    #[cfg(any(feature = "gif", feature = "video"))]
    pub fn from_braille_frames(frames: Vec<BrailleFrame>, loop_animation: bool) -> Self {
        let mut anim = Self::new(loop_animation);

        for frame in frames {
            anim.frames.push(FrameData {
                patterns: frame.patterns,
                width: frame.width,
                height: frame.height,
                duration: Duration::from_millis(frame.duration_ms as u64),
            });
        }

        anim
    }

    /// Get total number of frames
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Get current frame index
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }
}

impl Animation for FrameBasedAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        if self.frames.is_empty() || self.finished {
            return false;
        }

        self.elapsed += delta_time;

        // Check if we need to advance to the next frame
        let current_duration = self.frames[self.current_frame].duration;
        if self.elapsed >= current_duration {
            self.elapsed -= current_duration;
            self.current_frame += 1;

            // Check if animation is done
            if self.current_frame >= self.frames.len() {
                if self.loop_animation {
                    self.current_frame = 0;
                } else {
                    self.finished = true;
                    return false;
                }
            }
        }

        true
    }

    fn render(&self, grid: &mut BrailleGrid) {
        if self.frames.is_empty() || self.current_frame >= self.frames.len() {
            return;
        }

        let frame = &self.frames[self.current_frame];

        // Render frame to grid
        for y in 0..frame.height.min(grid.height()) {
            for x in 0..frame.width.min(grid.width()) {
                let pattern = frame.patterns[y * frame.width + x];

                if pattern != 0 {
                    // Reconstruct the dots by setting each bit
                    for bit in 0..8 {
                        if (pattern & (1 << bit)) != 0 {
                            // Map bit to dot position
                            let (dot_x, dot_y) = match bit {
                                0 => (x * 2, y * 4),         // Dot 1
                                1 => (x * 2, y * 4 + 1),     // Dot 2
                                2 => (x * 2, y * 4 + 2),     // Dot 3
                                3 => (x * 2 + 1, y * 4),     // Dot 4
                                4 => (x * 2 + 1, y * 4 + 1), // Dot 5
                                5 => (x * 2 + 1, y * 4 + 2), // Dot 6
                                6 => (x * 2, y * 4 + 3),     // Dot 7
                                7 => (x * 2 + 1, y * 4 + 3), // Dot 8
                                _ => unreachable!(),
                            };

                            if dot_x < grid.dot_width() && dot_y < grid.dot_height() {
                                grid.set_dot(dot_x, dot_y);
                            }
                        }
                    }
                }
            }
        }
    }

    fn name(&self) -> &str {
        "Frame-Based Animation"
    }

    fn duration(&self) -> Option<Duration> {
        if self.loop_animation {
            None
        } else {
            Some(
                self.frames
                    .iter()
                    .map(|f| f.duration)
                    .sum()
            )
        }
    }
}

impl Default for FrameBasedAnimation {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_based_animation() {
        let mut anim = FrameBasedAnimation::new(false);

        // Add a simple frame
        anim.add_frame(vec![0xFF; 10 * 10], 10, 10, Duration::from_millis(100));

        assert_eq!(anim.frame_count(), 1);
        assert_eq!(anim.current_frame(), 0);

        // Update should advance after duration
        let result = anim.update(Duration::from_millis(150));
        assert!(!result); // Should finish (non-looping, 1 frame)
    }

    #[test]
    fn test_looping_animation() {
        let mut anim = FrameBasedAnimation::new(true);

        anim.add_frame(vec![0xFF; 10 * 10], 10, 10, Duration::from_millis(100));
        anim.add_frame(vec![0x00; 10 * 10], 10, 10, Duration::from_millis(100));

        // Advance through frames
        anim.update(Duration::from_millis(150));
        assert_eq!(anim.current_frame(), 1);

        // Should loop back to frame 0
        anim.update(Duration::from_millis(150));
        assert_eq!(anim.current_frame(), 0);
    }
}
