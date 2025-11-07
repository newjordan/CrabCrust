// Classic pinball BONUS MULTIPLIER animation
//
// Counting up: 2X... 3X... 4X... 5X!
// That sweet bonus multiplier racking up the points!

use crate::animation::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

pub struct BonusMultiplierAnimation {
    elapsed: Duration,
    duration: Duration,
    current_multiplier: u32,
    max_multiplier: u32,
    advance_timer: Duration,
    flash_state: bool,
    flash_timer: Duration,
    rays: Vec<Ray>,
}

struct Ray {
    angle: f32,
    length: f32,
    speed: f32,
}

impl BonusMultiplierAnimation {
    pub fn new(duration: Duration, max_multiplier: u32) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as f32;

        let mut rays = Vec::new();
        for i in 0..16 {
            rays.push(Ray {
                angle: (i as f32 / 16.0) * 6.28 + seed * 0.001,
                length: 20.0,
                speed: 20.0 + (i as f32 * 2.0),
            });
        }

        Self {
            elapsed: Duration::ZERO,
            duration,
            current_multiplier: 2,
            max_multiplier,
            advance_timer: Duration::ZERO,
            flash_state: true,
            flash_timer: Duration::ZERO,
            rays,
        }
    }

    fn draw_digit(&self, grid: &mut BrailleGrid, digit: u32, x_offset: i32, color: Color) {
        let center_x = (grid.dot_width() / 2) as i32 + x_offset;
        let center_y = (grid.dot_height() / 2) as i32;

        let w = 20;
        let h = 30;
        let x = center_x - w / 2;
        let y = center_y - h / 2;

        // Draw 7-segment style digits
        match digit {
            2 => {
                // Top
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, y as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + 1) as usize, color);
                }
                // Top right
                for dy in 2..(h / 2) {
                    grid.set_dot_with_color((x + w - 2) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + w - 3) as usize, (y + dy) as usize, color);
                }
                // Middle
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2 + 1) as usize, color);
                }
                // Bottom left
                for dy in (h / 2 + 2)..(h - 2) {
                    grid.set_dot_with_color((x + 1) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + 2) as usize, (y + dy) as usize, color);
                }
                // Bottom
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 2) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 1) as usize, color);
                }
            }
            3 => {
                // Top, middle, bottom horizontals
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, y as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + 1) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2 + 1) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 2) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 1) as usize, color);
                }
                // Right vertical
                for dy in 2..(h - 2) {
                    grid.set_dot_with_color((x + w - 2) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + w - 3) as usize, (y + dy) as usize, color);
                }
            }
            4 => {
                // Top left
                for dy in 0..(h / 2 + 2) {
                    grid.set_dot_with_color((x + 1) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + 2) as usize, (y + dy) as usize, color);
                }
                // Middle
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2 + 1) as usize, color);
                }
                // Right vertical (full)
                for dy in 0..h {
                    grid.set_dot_with_color((x + w - 2) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + w - 3) as usize, (y + dy) as usize, color);
                }
            }
            5 => {
                // Top
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, y as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + 1) as usize, color);
                }
                // Top left
                for dy in 2..(h / 2) {
                    grid.set_dot_with_color((x + 1) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + 2) as usize, (y + dy) as usize, color);
                }
                // Middle
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2 + 1) as usize, color);
                }
                // Bottom right
                for dy in (h / 2 + 2)..(h - 2) {
                    grid.set_dot_with_color((x + w - 2) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + w - 3) as usize, (y + dy) as usize, color);
                }
                // Bottom
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 2) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 1) as usize, color);
                }
            }
            6 => {
                // Top, middle, bottom
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, y as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + 1) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2 + 1) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 2) as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 1) as usize, color);
                }
                // Left vertical (full)
                for dy in 2..(h - 2) {
                    grid.set_dot_with_color((x + 1) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + 2) as usize, (y + dy) as usize, color);
                }
                // Bottom right
                for dy in (h / 2 + 2)..(h - 2) {
                    grid.set_dot_with_color((x + w - 2) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + w - 3) as usize, (y + dy) as usize, color);
                }
            }
            _ => {}
        }

        // Draw "X" symbol
        let x_pos = x + w + 5;
        let x_size = 12;
        for i in 0..x_size {
            // Diagonal \
            grid.set_dot_with_color((x_pos + i) as usize, (y + h / 2 - x_size / 2 + i) as usize, color);
            grid.set_dot_with_color((x_pos + i + 1) as usize, (y + h / 2 - x_size / 2 + i) as usize, color);
            // Diagonal /
            grid.set_dot_with_color((x_pos + i) as usize, (y + h / 2 + x_size / 2 - i) as usize, color);
            grid.set_dot_with_color((x_pos + i + 1) as usize, (y + h / 2 + x_size / 2 - i) as usize, color);
        }
    }
}

impl Animation for BonusMultiplierAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;

        if self.elapsed >= self.duration {
            return false;
        }

        // Update flash
        self.flash_timer += delta_time;
        if self.flash_timer >= Duration::from_millis(100) {
            self.flash_state = !self.flash_state;
            self.flash_timer = Duration::ZERO;
        }

        // Advance multiplier
        self.advance_timer += delta_time;
        if self.advance_timer >= Duration::from_millis(600) && self.current_multiplier < self.max_multiplier {
            self.current_multiplier += 1;
            self.advance_timer = Duration::ZERO;
        }

        // Update rays
        let dt = delta_time.as_secs_f32();
        for ray in &mut self.rays {
            ray.length += ray.speed * dt;
            if ray.length > 60.0 {
                ray.length = 20.0;
            }
        }

        true
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        // Draw rays
        let ray_color = Color::new(100, 100, 255);
        for ray in &self.rays {
            let end_x = center_x + (ray.angle.cos() * ray.length) as i32;
            let end_y = center_y + (ray.angle.sin() * ray.length * 0.6) as i32;

            if end_x >= 0 && end_x < grid.dot_width() as i32 && end_y >= 0 && end_y < grid.dot_height() as i32 {
                grid.draw_line_with_color(
                    center_x as usize,
                    center_y as usize,
                    end_x as usize,
                    end_y as usize,
                    ray_color,
                );
            }
        }

        // Draw the current multiplier
        let color = if self.flash_state {
            Color::new(255, 255, 100)
        } else {
            Color::new(255, 200, 0)
        };

        self.draw_digit(grid, self.current_multiplier, 0, color);

        // Draw "BONUS" text above
        let bonus_color = Color::new(255, 150, 0);
        let text = "BONUS";
        let char_w = 6;
        let spacing = 1;
        let total_w = (text.len() as i32 * char_w) + ((text.len() as i32 - 1) * spacing);
        let start_x = center_x - total_w / 2;
        let text_y = center_y - 35;

        for (i, ch) in text.chars().enumerate() {
            let x = start_x + (i as i32 * (char_w + spacing));

            for dy in 0..7 {
                for dx in 0..char_w {
                    let show = match ch {
                        'B' => dx == 0 || dy == 0 || dy == 3 || dy == 6 || (dx == char_w - 1 && dy != 3),
                        'O' => (dx == 0 || dx == char_w - 1) || (dy == 0 || dy == 6),
                        'N' => dx == 0 || dx == char_w - 1 || (dx == dy),
                        'U' => (dx == 0 || dx == char_w - 1) || dy == 6,
                        'S' => dy == 0 || dy == 3 || dy == 6 || (dx == 0 && dy < 3) || (dx == char_w - 1 && dy > 3),
                        _ => false,
                    };

                    if show {
                        let px = x + dx;
                        let py = text_y + dy;
                        if px >= 0 && px < grid.dot_width() as i32 && py >= 0 && py < grid.dot_height() as i32 {
                            grid.set_dot_with_color(px as usize, py as usize, bonus_color);
                        }
                    }
                }
            }
        }
    }

    fn name(&self) -> &str {
        "Bonus Multiplier"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.duration)
    }
}

impl Default for BonusMultiplierAnimation {
    fn default() -> Self {
        Self::new(Duration::from_secs(3), 5)
    }
}
