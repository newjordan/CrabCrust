// Download animation for git pull
use super::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

/// Downward arrows used for `git pull`.
pub struct DownloadAnimation {
    elapsed: Duration,
    total_duration: Duration,
    particles: Vec<Particle>,
}

struct Particle {
    x: f32,
    y: f32,
    speed: f32,
    color: Color,
}

impl DownloadAnimation {
    /// Drop arrows for `duration`, then stop.
    pub fn new(duration: Duration) -> Self {
        let mut particles = Vec::new();

        // Create downward-moving particles
        for i in 0..15 {
            particles.push(Particle {
                x: (i as f32 * 20.0) % 200.0 + 50.0,
                y: -(i as f32 * 10.0) % 100.0,
                speed: 80.0 + (i as f32 * 5.0),
                color: if i % 3 == 0 {
                    Color::new(100, 200, 255) // Light blue
                } else if i % 3 == 1 {
                    Color::new(50, 150, 255) // Blue
                } else {
                    Color::new(0, 100, 255) // Dark blue
                },
            });
        }

        Self {
            elapsed: Duration::ZERO,
            total_duration: duration,
            particles,
        }
    }
}

impl Default for DownloadAnimation {
    fn default() -> Self {
        Self::new(Duration::from_millis(1500))
    }
}

impl Animation for DownloadAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;

        let dt = delta_time.as_secs_f32();

        // Update particles
        for particle in &mut self.particles {
            particle.y += particle.speed * dt;

            // Reset particle when it goes off screen
            if particle.y > 300.0 {
                particle.y = -20.0;
            }
        }

        self.elapsed < self.total_duration
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = grid.dot_width() / 2;
        let center_y = grid.dot_height() / 2;

        // Draw download icon/arrow at top center
        let icon_x = center_x;
        let icon_y = center_y.saturating_sub(60);

        // Draw down arrow
        for y in 0..40i32 {
            let width = if y < 25 { 8 } else { (40 - y) * 2 };
            for x in -(width / 2)..(width / 2) {
                let px = (icon_x as i32 + x).max(0) as usize;
                let py = (icon_y as i32 + y).max(0) as usize;
                if px < grid.dot_width() && py < grid.dot_height() {
                    grid.set_dot_with_color(px, py, Color::new(0, 255, 150));
                }
            }
        }

        // Draw particles
        for particle in &self.particles {
            let x = particle.x as usize;
            let y = particle.y as usize;

            if x < grid.dot_width() && y < grid.dot_height() {
                // Draw arrow shape
                for dy in 0..8 {
                    for dx in 0..3 {
                        let px = x + dx;
                        let py = y + dy;
                        if px < grid.dot_width() && py < grid.dot_height() {
                            grid.set_dot_with_color(px, py, particle.color);
                        }
                    }
                }

                // Arrow head
                if y + 8 < grid.dot_height() {
                    for dx in 0..7i32 {
                        let px = (x as i32 - 2 + dx).max(0) as usize;
                        let py = y + 8;
                        if px < grid.dot_width() {
                            grid.set_dot_with_color(px, py, particle.color);
                        }
                    }
                }
            }
        }
    }

    fn name(&self) -> &str {
        "Download"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }
}
