// Baby announcement animation - "Congratulations, you're the father of a new commit!"
use super::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

struct Confetti {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    rotation: f32,
}

/// Baby announcement celebration animation
pub struct BabyAnnouncementAnimation {
    elapsed: Duration,
    total_duration: Duration,
    confetti: Vec<Confetti>,
}

impl BabyAnnouncementAnimation {
    pub fn new(duration: Duration) -> Self {
        let mut confetti = Vec::new();

        // Create confetti particles
        for i in 0..50 {
            let x = (i as f32 * 10.0) % 300.0;
            confetti.push(Confetti {
                x,
                y: -20.0 - (i as f32 * 5.0) % 50.0,
                vx: (i as f32 * 0.5).sin() * 10.0,
                vy: 30.0 + (i as f32 % 20.0),
                color: [
                    Color::new(255, 182, 193), // Light pink
                    Color::new(173, 216, 230), // Light blue
                    Color::new(255, 255, 0),   // Yellow
                    Color::new(255, 192, 203), // Pink
                    Color::new(135, 206, 250), // Sky blue
                ][i % 5],
                rotation: 0.0,
            });
        }

        Self {
            elapsed: Duration::ZERO,
            total_duration: duration,
            confetti,
        }
    }
}

impl Default for BabyAnnouncementAnimation {
    fn default() -> Self {
        Self::new(Duration::from_millis(2500))
    }
}

impl Animation for BabyAnnouncementAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;
        let dt = delta_time.as_secs_f32();

        // Update confetti
        for particle in &mut self.confetti {
            particle.x += particle.vx * dt;
            particle.y += particle.vy * dt;
            particle.rotation += dt * 2.0;

            // Reset if off screen
            if particle.y > 250.0 {
                particle.y = -10.0;
                particle.x = (particle.x + 50.0) % 300.0;
            }
        }

        self.elapsed < self.total_duration
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = grid.dot_width() / 2;
        let center_y = grid.dot_height() / 2;
        let progress = (self.elapsed.as_secs_f32() / self.total_duration.as_secs_f32()).min(1.0);

        // Draw confetti
        for particle in &self.confetti {
            let x = particle.x as usize;
            let y = particle.y as usize;

            if x < grid.dot_width() && y < grid.dot_height() {
                // Draw confetti piece (small rectangle)
                for dy in 0..3 {
                    for dx in 0..2 {
                        let px = x + dx;
                        let py = y + dy;
                        if px < grid.dot_width() && py < grid.dot_height() {
                            grid.set_dot_with_color(px, py, particle.color);
                        }
                    }
                }
            }
        }

        // Draw baby stork (appears in phases)
        if progress > 0.2 {
            let stork_x = center_x as i32 - 30;
            let stork_y = center_y as i32 - 20;

            // Stork body (white)
            let white = Color::new(255, 255, 255);
            let orange = Color::new(255, 165, 0);
            let pink = Color::new(255, 182, 193);

            // Body oval
            for dy in -10..10 {
                for dx in -8..8 {
                    let dist = ((dx * dx) as f32 / 64.0 + (dy * dy) as f32 / 100.0).sqrt();
                    if dist < 1.0 {
                        let x = (stork_x + dx).max(0) as usize;
                        let y = (stork_y + dy).max(0) as usize;
                        if x < grid.dot_width() && y < grid.dot_height() {
                            grid.set_dot_with_color(x, y, white);
                        }
                    }
                }
            }

            // Long neck
            for dy in -20..-10 {
                for dx in -2..2 {
                    let x = (stork_x + dx).max(0) as usize;
                    let y = (stork_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, white);
                    }
                }
            }

            // Head
            for dy in -25..-18 {
                for dx in -4..4 {
                    let x = (stork_x + dx).max(0) as usize;
                    let y = (stork_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, white);
                    }
                }
            }

            // Orange beak
            for dx in 0..10 {
                let x = (stork_x + 4 + dx).max(0) as usize;
                let y = (stork_y - 22).max(0) as usize;
                if x < grid.dot_width() && y < grid.dot_height() {
                    grid.set_dot_with_color(x, y, orange);
                }
            }

            // Wings (flapping)
            let wing_angle = (self.elapsed.as_secs_f32() * 8.0).sin() * 15.0;
            for dx in -15..-8 {
                for dy in -5..5 {
                    let y_offset = (dy as f32 + wing_angle) as i32;
                    let x = (stork_x + dx).max(0) as usize;
                    let y = (stork_y + y_offset).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, white);
                    }
                }
            }

            // Bundle (baby in blanket)
            if progress > 0.4 {
                let bundle_x = stork_x + 10;
                let bundle_y = stork_y + 15;

                // Pink blanket
                for dy in -8..8 {
                    for dx in -6..6 {
                        let dist = ((dx * dx) as f32 / 36.0 + (dy * dy) as f32 / 64.0).sqrt();
                        if dist < 1.0 {
                            let x = (bundle_x + dx).max(0) as usize;
                            let y = (bundle_y + dy).max(0) as usize;
                            if x < grid.dot_width() && y < grid.dot_height() {
                                grid.set_dot_with_color(x, y, pink);
                            }
                        }
                    }
                }
            }
        }

        // Draw "IT'S A COMMIT!" sign
        if progress > 0.6 {
            let sign_x = center_x + 40;
            let sign_y = center_y - 30;

            // Sign board
            let sign_color = Color::new(255, 255, 200);
            for dy in 0..30 {
                for dx in 0..60 {
                    let x = sign_x + dx;
                    let y = sign_y + dy;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        if dy < 3 || dy > 27 || dx < 3 || dx > 57 {
                            grid.set_dot_with_color(x, y, Color::new(139, 69, 19)); // Brown border
                        } else {
                            grid.set_dot_with_color(x, y, sign_color);
                        }
                    }
                }
            }
        }

        // Draw balloons
        if progress > 0.5 {
            let balloon_colors = [
                Color::new(255, 0, 0),
                Color::new(0, 0, 255),
                Color::new(255, 255, 0),
            ];

            for (i, color) in balloon_colors.iter().enumerate() {
                let balloon_x = center_x as i32 + 50 + i as i32 * 25;
                let balloon_y = center_y as i32 - 50 + (((self.elapsed.as_secs_f32() + i as f32) * 2.0).sin() * 5.0) as i32;

                // Balloon (oval)
                for dy in -12..0 {
                    for dx in -8..8 {
                        let dist = ((dx * dx) as f32 / 64.0 + (dy * dy) as f32 / 144.0).sqrt();
                        if dist < 1.0 {
                            let x = (balloon_x + dx).max(0) as usize;
                            let y = (balloon_y + dy).max(0) as usize;
                            if x < grid.dot_width() && y < grid.dot_height() {
                                grid.set_dot_with_color(x, y, *color);
                            }
                        }
                    }
                }

                // String
                for dy in 0..20 {
                    let x = balloon_x.max(0) as usize;
                    let y = (balloon_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, Color::new(100, 100, 100));
                    }
                }
            }
        }

        // Hearts floating up
        for i in 0..5 {
            let heart_time = self.elapsed.as_secs_f32() - i as f32 * 0.2;
            if heart_time > 0.0 {
                let heart_x = center_x as i32 + (i as i32 - 2) * 40;
                let heart_y = center_y as i32 + 40 - (heart_time * 30.0) as i32;

                let red = Color::new(255, 0, 100);

                // Simple heart shape
                for dy in -3i32..3i32 {
                    for dx in -4i32..4i32 {
                        let is_heart = (dy <= 0 && (dx.abs() < 4 - dy.abs())) ||
                                      (dy > 0 && dy < 3 && dx.abs() < 2);
                        if is_heart {
                            let x = (heart_x + dx).max(0) as usize;
                            let y = (heart_y + dy).max(0) as usize;
                            if x < grid.dot_width() && y < grid.dot_height() {
                                grid.set_dot_with_color(x, y, red);
                            }
                        }
                    }
                }
            }
        }
    }

    fn name(&self) -> &str {
        "BabyAnnouncement"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }
}
