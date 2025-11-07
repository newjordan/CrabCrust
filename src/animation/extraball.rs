// Classic pinball EXTRA BALL animation
//
// Shiny ball with sparkles and fanfare
// One of the best moments in pinball!

use crate::animation::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

pub struct ExtraBallAnimation {
    elapsed: Duration,
    duration: Duration,
    flash_state: bool,
    flash_timer: Duration,
    sparkles: Vec<Sparkle>,
}

struct Sparkle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
}

impl ExtraBallAnimation {
    pub fn new(duration: Duration) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as f32;

        let mut sparkles = Vec::new();
        for i in 0..40 {
            let angle = (i as f32 / 40.0) * 6.28 + seed * 0.001;
            let speed = 30.0 + (seed * 0.01 + i as f32).sin() * 20.0;
            sparkles.push(Sparkle {
                x: 0.0,
                y: 0.0,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life: 0.0,
                max_life: 2.0 + (i as f32 * 0.05),
            });
        }

        Self {
            elapsed: Duration::ZERO,
            duration,
            flash_state: true,
            flash_timer: Duration::ZERO,
            sparkles,
        }
    }

    fn draw_ball(&self, grid: &mut BrailleGrid) {
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        let pulse = (self.elapsed.as_secs_f32() * 3.0).sin() * 0.2 + 1.0;
        let radius = (15.0 * pulse) as i32;

        // Draw shiny metallic ball
        for dy in -radius..radius {
            for dx in -radius..radius {
                let dist_sq = dx * dx + dy * dy;
                let r_sq = radius * radius;

                if dist_sq < r_sq {
                    let x = center_x + dx;
                    let y = center_y + dy;

                    if x >= 0 && x < grid.dot_width() as i32 && y >= 0 && y < grid.dot_height() as i32 {
                        // Gradient from center (lighter) to edge (darker)
                        let dist_ratio = (dist_sq as f32 / r_sq as f32).sqrt();
                        let brightness = (255.0 * (1.0 - dist_ratio * 0.5)) as u8;

                        // Chrome highlight
                        let is_highlight = dx < 0 && dy < 0 && dist_sq < (r_sq / 4);

                        let color = if is_highlight {
                            Color::new(255, 255, 255)
                        } else {
                            Color::new(brightness, brightness, brightness)
                        };

                        grid.set_dot_with_color(x as usize, y as usize, color);
                    }
                }
            }
        }
    }

    fn draw_text(&self, grid: &mut BrailleGrid, text: &str, y_offset: i32, flash: bool) {
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        let color = if flash {
            Color::new(100, 255, 100) // Bright green
        } else {
            Color::new(50, 200, 50) // Darker green
        };

        let char_width = 7;
        let spacing = 2;
        let total_width = (text.len() as i32 * char_width) + ((text.len() as i32 - 1) * spacing);
        let start_x = center_x - total_width / 2;

        for (i, ch) in text.chars().enumerate() {
            let x = start_x + (i as i32 * (char_width + spacing));
            let y = center_y + y_offset;

            for dy in 0..9 {
                for dx in 0..char_width {
                    let show = match ch {
                        'E' => dx == 0 || dy == 0 || dy == 4 || dy == 8,
                        'X' => (dx == dy && dx < 4) || (dx + dy == char_width - 1 && dx < 4) ||
                               (dx == char_width - 1 - dy && dx >= char_width - 4) ||
                               (dx + dy == 2 * char_width - 2 && dx >= char_width - 4),
                        'T' => dy == 0 || dx == char_width / 2,
                        'R' => dx == 0 || dy == 0 || dy == 4 ||
                               (dx == char_width - 1 && dy < 4) ||
                               (dy > 4 && dx == dy - 4),
                        'A' => (dx == 0 || dx == char_width - 1) ||
                               (dy == 0 && dx > 0 && dx < char_width - 1) ||
                               (dy == 4 && dx > 0 && dx < char_width - 1),
                        'B' => dx == 0 || dy == 0 || dy == 4 || dy == 8 ||
                               (dx == char_width - 1 && dy != 4),
                        'L' => dx == 0 || dy == 8,
                        _ => false,
                    };

                    if show {
                        let px = x + dx;
                        let py = y + dy;
                        if px >= 0 && px < grid.dot_width() as i32 && py >= 0 && py < grid.dot_height() as i32 {
                            grid.set_dot_with_color(px as usize, py as usize, color);
                        }
                    }
                }
            }
        }
    }
}

impl Animation for ExtraBallAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;

        if self.elapsed >= self.duration {
            return false;
        }

        // Update flash
        self.flash_timer += delta_time;
        if self.flash_timer >= Duration::from_millis(180) {
            self.flash_state = !self.flash_state;
            self.flash_timer = Duration::ZERO;
        }

        // Update sparkles
        let dt = delta_time.as_secs_f32();
        for sparkle in &mut self.sparkles {
            sparkle.life += dt;
            if sparkle.life < sparkle.max_life {
                sparkle.x += sparkle.vx * dt;
                sparkle.y += sparkle.vy * dt;
                sparkle.vy += 50.0 * dt; // Gravity
            }
        }

        true
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        // Draw sparkles
        for sparkle in &self.sparkles {
            if sparkle.life < sparkle.max_life {
                let alpha = 1.0 - (sparkle.life / sparkle.max_life);
                let brightness = (255.0 * alpha) as u8;

                let x = center_x + sparkle.x as i32;
                let y = center_y + sparkle.y as i32;

                if x >= 0 && x < grid.dot_width() as i32 && y >= 0 && y < grid.dot_height() as i32 {
                    let color = Color::new(brightness, brightness, 100);
                    grid.set_dot_with_color(x as usize, y as usize, color);

                    // Draw star shape
                    if x + 1 < grid.dot_width() as i32 {
                        grid.set_dot_with_color((x + 1) as usize, y as usize, color);
                    }
                    if x > 0 {
                        grid.set_dot_with_color((x - 1) as usize, y as usize, color);
                    }
                    if y + 1 < grid.dot_height() as i32 {
                        grid.set_dot_with_color(x as usize, (y + 1) as usize, color);
                    }
                    if y > 0 {
                        grid.set_dot_with_color(x as usize, (y - 1) as usize, color);
                    }
                }
            }
        }

        // Draw the ball
        self.draw_ball(grid);

        // Draw "EXTRA BALL" text
        self.draw_text(grid, "EXTRA", -35, self.flash_state);
        self.draw_text(grid, "BALL", 35, self.flash_state);

        // Draw arrow pointing to ball
        if self.flash_state {
            let arrow_color = Color::new(255, 200, 0);
            for i in 0..10 {
                let x = center_x - 30;
                let y = center_y - 5 + i;
                if y >= 0 && y < grid.dot_height() as i32 {
                    grid.set_dot_with_color(x as usize, y as usize, arrow_color);
                }
                // Arrow head
                if i < 5 {
                    grid.set_dot_with_color((x + i) as usize, (center_y + i) as usize, arrow_color);
                }
            }
        }
    }

    fn name(&self) -> &str {
        "Extra Ball"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.duration)
    }
}

impl Default for ExtraBallAnimation {
    fn default() -> Self {
        Self::new(Duration::from_secs(3))
    }
}
