// Trophy animation - YOU'RE A CHAMPION!
use super::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

/// Trophy celebration animation with sparkles
pub struct TrophyAnimation {
    elapsed: Duration,
    total_duration: Duration,
}

impl TrophyAnimation {
    pub fn new(duration: Duration) -> Self {
        Self {
            elapsed: Duration::ZERO,
            total_duration: duration,
        }
    }
}

impl Default for TrophyAnimation {
    fn default() -> Self {
        Self::new(Duration::from_millis(2000))
    }
}

impl Animation for TrophyAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;
        self.elapsed < self.total_duration
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = grid.dot_width() / 2;
        let center_y = grid.dot_height() / 2;
        let progress = (self.elapsed.as_secs_f32() / self.total_duration.as_secs_f32()).min(1.0);

        let gold = Color::new(255, 215, 0);
        let dark_gold = Color::new(204, 172, 0);
        let brown = Color::new(139, 69, 19);

        // Trophy appears from bottom up
        let reveal_height = (progress * 80.0) as i32;

        // Draw trophy
        let trophy_x = center_x as i32;
        let trophy_y = center_y as i32;

        // Trophy cup (upper part)
        if reveal_height > 10 {
            for dy in -30..-10 {
                if trophy_y + dy < 0 {
                    continue;
                }
                let width = 20 + ((dy + 30) / 2);
                for dx in -width..width {
                    let x = (trophy_x + dx).max(0) as usize;
                    let y = (trophy_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        // Gradient effect
                        let color = if dx.abs() < width - 2 {
                            gold
                        } else {
                            dark_gold
                        };
                        grid.set_dot_with_color(x, y, color);
                    }
                }
            }

            // Trophy rim (top edge)
            for dx in -25..25 {
                let x = (trophy_x + dx).max(0) as usize;
                let y = (trophy_y - 30).max(0) as usize;
                if x < grid.dot_width() && y < grid.dot_height() {
                    grid.set_dot_with_color(x, y, gold);
                }
                if y > 0 {
                    grid.set_dot_with_color(x, y - 1, gold);
                }
            }

            // Handles (on sides)
            for side in [-1, 1] {
                for dy in -25..-15 {
                    for dx in 0..8 {
                        let x = (trophy_x + side * (20 + dx)).max(0) as usize;
                        let y = (trophy_y + dy).max(0) as usize;
                        if x < grid.dot_width() && y < grid.dot_height() {
                            grid.set_dot_with_color(x, y, gold);
                        }
                    }
                }
            }
        }

        // Trophy stem
        if reveal_height > 40 {
            for dy in -10..5 {
                for dx in -4..4 {
                    let x = (trophy_x + dx).max(0) as usize;
                    let y = (trophy_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, gold);
                    }
                }
            }
        }

        // Trophy base
        if reveal_height > 50 {
            for dy in 5..15 {
                let width = 15 + (dy - 5);
                for dx in -width..width {
                    let x = (trophy_x + dx).max(0) as usize;
                    let y = (trophy_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, brown);
                    }
                }
            }
        }

        // Star on trophy
        if progress > 0.5 {
            let star_x = trophy_x;
            let star_y = trophy_y - 20;
            let star_size = ((progress - 0.5) * 20.0).min(10.0);

            // Draw 5-pointed star
            for angle_deg in (0..360).step_by(72) {
                let angle = (angle_deg as f32).to_radians();
                for r in 0..(star_size as i32) {
                    let x = (star_x + (angle.cos() * r as f32) as i32).max(0) as usize;
                    let y = (star_y + (angle.sin() * r as f32) as i32).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, Color::new(255, 255, 255));
                    }
                }

                // Inner points
                let inner_angle = (angle_deg as f32 + 36.0).to_radians();
                for r in 0..(star_size as i32 / 2) {
                    let x = (star_x + (inner_angle.cos() * r as f32) as i32).max(0) as usize;
                    let y = (star_y + (inner_angle.sin() * r as f32) as i32).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, Color::new(255, 255, 255));
                    }
                }
            }
        }

        // Sparkles around trophy
        if progress > 0.4 {
            let sparkle_time = (self.elapsed.as_secs_f32() * 3.0) % (std::f32::consts::PI * 2.0);

            for i in 0..12 {
                let angle = (i as f32 * 30.0 + sparkle_time.to_degrees()).to_radians();
                let radius = 50.0 + (sparkle_time + i as f32).sin() * 10.0;
                let sparkle_x = (center_x as f32 + angle.cos() * radius) as i32;
                let sparkle_y = (center_y as f32 + angle.sin() * radius * 0.6) as i32;

                // Draw sparkle (4-pointed star)
                for len in 0..5 {
                    // Horizontal
                    let x1 = (sparkle_x - len).max(0) as usize;
                    let x2 = (sparkle_x + len).max(0) as usize;
                    let y = sparkle_y.max(0) as usize;
                    if x1 < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x1, y, Color::new(255, 255, 200));
                    }
                    if x2 < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x2, y, Color::new(255, 255, 200));
                    }

                    // Vertical
                    let x = sparkle_x.max(0) as usize;
                    let y1 = (sparkle_y - len).max(0) as usize;
                    let y2 = (sparkle_y + len).max(0) as usize;
                    if x < grid.dot_width() && y1 < grid.dot_height() {
                        grid.set_dot_with_color(x, y1, Color::new(255, 255, 200));
                    }
                    if x < grid.dot_width() && y2 < grid.dot_height() {
                        grid.set_dot_with_color(x, y2, Color::new(255, 255, 200));
                    }
                }
            }
        }

        // "WINNER!" rays shooting out
        if progress > 0.6 {
            let ray_count = 16;
            for i in 0..ray_count {
                let angle = (i as f32 * 360.0 / ray_count as f32).to_radians();
                let ray_progress = ((progress - 0.6) * 2.5).min(1.0);
                let ray_length = ray_progress * 60.0;

                for r in (30..(ray_length as i32)).step_by(3) {
                    let x = (center_x as f32 + angle.cos() * r as f32) as i32;
                    let y = (center_y as f32 + angle.sin() * r as f32 * 0.6) as i32;

                    if x >= 0 && y >= 0 {
                        let px = x as usize;
                        let py = y as usize;
                        if px < grid.dot_width() && py < grid.dot_height() {
                            let color_intensity = 255 - ((r as f32 / ray_length * 100.0) as u8);
                            grid.set_dot_with_color(
                                px,
                                py,
                                Color::new(255, color_intensity, 0),
                            );
                        }
                    }
                }
            }
        }

        // Podium
        if progress > 0.8 {
            let podium_y = center_y + 15;
            let podium_height = 20;

            for dy in 0..podium_height {
                let width = 30;
                for dx in -width..width {
                    let x = (center_x as i32 + dx).max(0) as usize;
                    let y = (podium_y as i32 + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        // Striped podium
                        let color = if dy % 4 < 2 {
                            Color::new(169, 169, 169) // Light gray
                        } else {
                            Color::new(128, 128, 128) // Gray
                        };
                        grid.set_dot_with_color(x, y, color);
                    }
                }
            }

            // "1ST" on podium
            let text_y = podium_y + 8;
            for dx in -3..4 {
                let x = (center_x as i32 + dx).max(0) as usize;
                let y = text_y as usize;
                if x < grid.dot_width() && y < grid.dot_height() {
                    grid.set_dot_with_color(x, y, Color::new(255, 255, 0));
                }
            }
        }
    }

    fn name(&self) -> &str {
        "Trophy"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }
}
