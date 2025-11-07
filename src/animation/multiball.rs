// Classic pinball MULTIBALL animation
//
// Multiple balls bouncing around the playfield
// The holy grail of pinball - MULTIBALL MODE!

use crate::animation::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

const BALL_COUNT: usize = 5;

pub struct MultiballAnimation {
    elapsed: Duration,
    duration: Duration,
    balls: Vec<Ball>,
    flash_state: bool,
    flash_timer: Duration,
}

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    trail: Vec<(f32, f32)>,
}

impl MultiballAnimation {
    pub fn new(duration: Duration) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as f32;

        let colors = [
            Color::new(255, 255, 255), // White
            Color::new(255, 100, 100), // Red
            Color::new(100, 255, 100), // Green
            Color::new(100, 100, 255), // Blue
            Color::new(255, 255, 100), // Yellow
        ];

        let mut balls = Vec::new();
        for i in 0..BALL_COUNT {
            let offset = seed + i as f32 * 100.0;
            balls.push(Ball {
                x: (offset * 0.1).sin() * 30.0,
                y: (offset * 0.05).cos() * 20.0 - 30.0,
                vx: (offset * 0.02).sin() * 40.0 + 20.0,
                vy: (offset * 0.03).cos() * 20.0 + 30.0,
                color: colors[i % colors.len()],
                trail: Vec::new(),
            });
        }

        Self {
            elapsed: Duration::ZERO,
            duration,
            balls,
            flash_state: true,
            flash_timer: Duration::ZERO,
        }
    }

    fn draw_big_text(&self, grid: &mut BrailleGrid, text: &str, y_offset: i32, flash: bool) {
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        let color = if flash {
            Color::new(255, 100, 100) // Bright red
        } else {
            Color::new(200, 50, 50) // Dark red
        };

        // Draw block letters
        let char_width = 8;
        let spacing = 2;
        let total_width = (text.len() as i32 * char_width) + ((text.len() as i32 - 1) * spacing);
        let start_x = center_x - total_width / 2;

        for (i, ch) in text.chars().enumerate() {
            let x = start_x + (i as i32 * (char_width + spacing));
            let y = center_y + y_offset;

            // Draw each letter as a filled block
            for dy in 0..10 {
                for dx in 0..char_width {
                    let show = match ch {
                        'M' => {
                            dx == 0 || dx == char_width - 1 ||
                            (dy < 4 && (dx == 2 || dx == char_width - 3))
                        }
                        'U' => {
                            (dx == 0 || dx == char_width - 1) && dy < 9 ||
                            dy == 9 && dx > 0 && dx < char_width - 1
                        }
                        'L' => {
                            dx == 0 || dy == 9
                        }
                        'T' => {
                            dy == 0 || dx == char_width / 2
                        }
                        'I' => {
                            dx == char_width / 2 || dy == 0 || dy == 9
                        }
                        'B' => {
                            dx == 0 || dy == 0 || dy == 4 || dy == 9 ||
                            (dx == char_width - 1 && (dy < 4 || dy > 4))
                        }
                        'A' => {
                            (dx == 0 || dx == char_width - 1) ||
                            (dy == 0 && dx > 0 && dx < char_width - 1) ||
                            (dy == 5 && dx > 0 && dx < char_width - 1)
                        }
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

    fn draw_ball(&self, grid: &mut BrailleGrid, ball: &Ball) {
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        let x = center_x + ball.x as i32;
        let y = center_y + ball.y as i32;

        // Draw trail
        for (i, (tx, ty)) in ball.trail.iter().enumerate() {
            let alpha = (i as f32 / ball.trail.len() as f32) * 0.5;
            let trail_color = Color::new(
                (ball.color.r as f32 * alpha) as u8,
                (ball.color.g as f32 * alpha) as u8,
                (ball.color.b as f32 * alpha) as u8,
            );

            let px = center_x + *tx as i32;
            let py = center_y + *ty as i32;

            if px >= 0 && px < grid.dot_width() as i32 && py >= 0 && py < grid.dot_height() as i32 {
                grid.set_dot_with_color(px as usize, py as usize, trail_color);
            }
        }

        // Draw ball (circle)
        for dy in -4..4 {
            for dx in -4..4 {
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < 16 {
                    let px = x + dx;
                    let py = y + dy;
                    if px >= 0 && px < grid.dot_width() as i32 && py >= 0 && py < grid.dot_height() as i32 {
                        // Highlight on ball
                        let highlight = dx == -1 && dy == -1;
                        let color = if highlight {
                            Color::new(255, 255, 255)
                        } else {
                            ball.color
                        };
                        grid.set_dot_with_color(px as usize, py as usize, color);
                    }
                }
            }
        }
    }
}

impl Animation for MultiballAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;

        if self.elapsed >= self.duration {
            return false;
        }

        // Update flash
        self.flash_timer += delta_time;
        if self.flash_timer >= Duration::from_millis(200) {
            self.flash_state = !self.flash_state;
            self.flash_timer = Duration::ZERO;
        }

        // Update balls
        let dt = delta_time.as_secs_f32();
        for ball in &mut self.balls {
            // Store trail
            ball.trail.push((ball.x, ball.y));
            if ball.trail.len() > 8 {
                ball.trail.remove(0);
            }

            // Update position
            ball.x += ball.vx * dt;
            ball.y += ball.vy * dt;

            // Bounce off edges
            if ball.x < -60.0 || ball.x > 60.0 {
                ball.vx = -ball.vx;
                ball.x = ball.x.clamp(-60.0, 60.0);
            }

            if ball.y < -40.0 || ball.y > 40.0 {
                ball.vy = -ball.vy;
                ball.y = ball.y.clamp(-40.0, 40.0);
            }

            // Apply gravity
            ball.vy += 50.0 * dt;
        }

        // Ball-to-ball collision (simple)
        for i in 0..self.balls.len() {
            for j in (i + 1)..self.balls.len() {
                let dx = self.balls[j].x - self.balls[i].x;
                let dy = self.balls[j].y - self.balls[i].y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < 64.0 {
                    // Swap velocities (simplified)
                    let temp_vx = self.balls[i].vx;
                    let temp_vy = self.balls[i].vy;
                    self.balls[i].vx = self.balls[j].vx;
                    self.balls[i].vy = self.balls[j].vy;
                    self.balls[j].vx = temp_vx;
                    self.balls[j].vy = temp_vy;
                }
            }
        }

        true
    }

    fn render(&self, grid: &mut BrailleGrid) {
        // Draw balls
        for ball in &self.balls {
            self.draw_ball(grid, ball);
        }

        // Draw "MULTIBALL" text at top
        self.draw_big_text(grid, "MULTIBALL", -35, self.flash_state);

        // Draw ball count
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        // Small dots showing ball count
        for i in 0..BALL_COUNT {
            let x = center_x - 20 + (i as i32 * 10);
            let y = center_y + 30;

            for dy in -2..2 {
                for dx in -2..2 {
                    if dx * dx + dy * dy < 4 {
                        let px = x + dx;
                        let py = y + dy;
                        if px >= 0 && px < grid.dot_width() as i32 && py >= 0 && py < grid.dot_height() as i32 {
                            grid.set_dot_with_color(px as usize, py as usize, Color::new(255, 255, 255));
                        }
                    }
                }
            }
        }
    }

    fn name(&self) -> &str {
        "Multiball"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.duration)
    }
}

impl Default for MultiballAnimation {
    fn default() -> Self {
        Self::new(Duration::from_secs(3))
    }
}
