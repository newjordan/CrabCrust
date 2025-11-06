// Confetti animation - simple celebration!
use super::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

struct ConfettiPiece {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    rotation: f32,
    rotation_speed: f32,
    color: Color,
    shape: u8, // 0 = square, 1 = circle, 2 = triangle
}

/// Confetti celebration animation
pub struct ConfettiAnimation {
    elapsed: Duration,
    total_duration: Duration,
    pieces: Vec<ConfettiPiece>,
}

impl ConfettiAnimation {
    pub fn new(duration: Duration) -> Self {
        let mut pieces = Vec::new();

        let colors = [
            Color::new(255, 0, 0),      // Red
            Color::new(255, 165, 0),    // Orange
            Color::new(255, 255, 0),    // Yellow
            Color::new(0, 255, 0),      // Green
            Color::new(0, 0, 255),      // Blue
            Color::new(128, 0, 128),    // Purple
            Color::new(255, 192, 203),  // Pink
            Color::new(255, 215, 0),    // Gold
        ];

        // Create confetti pieces
        for i in 0..100 {
            let x = (i as f32 * 5.0) % 300.0;
            pieces.push(ConfettiPiece {
                x,
                y: -30.0 - (i as f32 * 2.0) % 80.0,
                vx: ((i as f32 * 0.7).sin() * 20.0),
                vy: 40.0 + (i as f32 % 30.0),
                rotation: (i as f32 * 0.5) % 360.0,
                rotation_speed: ((i as f32 * 0.3).sin() + 1.0) * 3.0,
                color: colors[i % colors.len()],
                shape: (i % 3) as u8,
            });
        }

        Self {
            elapsed: Duration::ZERO,
            total_duration: duration,
            pieces,
        }
    }
}

impl Default for ConfettiAnimation {
    fn default() -> Self {
        Self::new(Duration::from_millis(2000))
    }
}

impl Animation for ConfettiAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;
        let dt = delta_time.as_secs_f32();

        // Update each confetti piece
        for piece in &mut self.pieces {
            piece.x += piece.vx * dt;
            piece.y += piece.vy * dt;
            piece.vy += 20.0 * dt; // Gravity
            piece.rotation += piece.rotation_speed * dt * 60.0;

            // Add some wobble
            piece.vx += ((piece.y * 0.1).sin() * 5.0) * dt;

            // Reset if off screen
            if piece.y > 250.0 {
                piece.y = -10.0;
                piece.vy = 40.0 + ((piece.x * 0.1) % 30.0);
            }

            // Wrap horizontally
            if piece.x < -10.0 {
                piece.x += 310.0;
            } else if piece.x > 300.0 {
                piece.x -= 310.0;
            }
        }

        self.elapsed < self.total_duration
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = grid.dot_width() / 2;
        let center_y = grid.dot_height() / 2;

        // Draw each confetti piece
        for piece in &self.pieces {
            let x = piece.x as i32;
            let y = piece.y as i32;

            if x >= 0 && y >= 0 && (x as usize) < grid.dot_width() && (y as usize) < grid.dot_height() {
                match piece.shape {
                    0 => {
                        // Square
                        for dy in 0..4 {
                            for dx in 0..4 {
                                let px = (x + dx).max(0) as usize;
                                let py = (y + dy).max(0) as usize;
                                if px < grid.dot_width() && py < grid.dot_height() {
                                    grid.set_dot_with_color(px, py, piece.color);
                                }
                            }
                        }
                    }
                    1 => {
                        // Circle
                        for dy in -2..3 {
                            for dx in -2..3 {
                                if dx * dx + dy * dy <= 4 {
                                    let px = (x + dx).max(0) as usize;
                                    let py = (y + dy).max(0) as usize;
                                    if px < grid.dot_width() && py < grid.dot_height() {
                                        grid.set_dot_with_color(px, py, piece.color);
                                    }
                                }
                            }
                        }
                    }
                    2 => {
                        // Triangle
                        for dy in 0..5 {
                            for dx in 0..(5 - dy) {
                                let px = (x + dx).max(0) as usize;
                                let py = (y + dy).max(0) as usize;
                                if px < grid.dot_width() && py < grid.dot_height() {
                                    grid.set_dot_with_color(px, py, piece.color);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Draw "YAY!" in the center with stars
        let progress = (self.elapsed.as_secs_f32() / self.total_duration.as_secs_f32()).min(1.0);

        if progress > 0.3 && progress < 0.8 {
            // Draw stars around
            for i in 0..8 {
                let angle = (i as f32 * 45.0 + progress * 360.0).to_radians();
                let radius = 40.0 + (progress * 10.0).sin() * 5.0;
                let star_x = (center_x as f32 + angle.cos() * radius) as i32;
                let star_y = (center_y as f32 + angle.sin() * radius * 0.6) as i32;

                // Draw 5-pointed star
                for point in 0..5 {
                    let point_angle = (point as f32 * 72.0).to_radians();
                    for r in 0..6 {
                        let px = (star_x + (point_angle.cos() * r as f32) as i32).max(0) as usize;
                        let py = (star_y + (point_angle.sin() * r as f32) as i32).max(0) as usize;
                        if px < grid.dot_width() && py < grid.dot_height() {
                            grid.set_dot_with_color(px, py, Color::new(255, 255, 0));
                        }
                    }
                }
            }
        }

        // Sparkles
        let sparkle_count = ((progress * 20.0) as usize).min(20);
        for i in 0..sparkle_count {
            let angle = (i as f32 * 30.0 + self.elapsed.as_secs_f32() * 100.0).to_radians();
            let radius = 30.0 + (i as f32 * 2.0);
            let sx = (center_x as f32 + angle.cos() * radius) as usize;
            let sy = (center_y as f32 + angle.sin() * radius * 0.5) as usize;

            if sx < grid.dot_width() && sy < grid.dot_height() {
                grid.set_dot_with_color(sx, sy, Color::new(255, 255, 255));
            }
        }
    }

    fn name(&self) -> &str {
        "Confetti"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }
}
