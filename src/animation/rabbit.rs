// White rabbit animation - "I'm late! I'm late!"
use super::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

/// White rabbit hopping across the screen
pub struct RabbitAnimation {
    elapsed: Duration,
    total_duration: Duration,
}

impl RabbitAnimation {
    pub fn new(duration: Duration) -> Self {
        Self {
            elapsed: Duration::ZERO,
            total_duration: duration,
        }
    }
}

impl Default for RabbitAnimation {
    fn default() -> Self {
        Self::new(Duration::from_millis(2000))
    }
}

impl Animation for RabbitAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;
        self.elapsed < self.total_duration
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let progress = (self.elapsed.as_secs_f32() / self.total_duration.as_secs_f32()).min(1.0);

        let center_y = grid.dot_height() / 2;

        // Rabbit hops from left to right
        let rabbit_x = (progress * grid.dot_width() as f32 * 1.2 - grid.dot_width() as f32 * 0.1) as i32;

        // Hopping motion (sine wave)
        let hop_height = ((progress * 10.0).sin().abs() * 30.0) as i32;
        let rabbit_y = center_y as i32 - hop_height;

        // Draw white rabbit
        let white = Color::new(255, 255, 255);
        let pink = Color::new(255, 192, 203);
        let black = Color::new(0, 0, 0);

        // Rabbit body (oval)
        for dy in -15..15 {
            for dx in -10..10 {
                let dist = ((dx * dx) as f32 / 100.0 + (dy * dy) as f32 / 225.0).sqrt();
                if dist < 1.0 {
                    let x = (rabbit_x + dx).max(0) as usize;
                    let y = (rabbit_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, white);
                    }
                }
            }
        }

        // Ears (two long ovals on top)
        for ear in 0..2 {
            let ear_x_offset = if ear == 0 { -6 } else { 6 };
            for dy in -25..-10 {
                for dx in -3..3 {
                    let x = (rabbit_x + ear_x_offset + dx).max(0) as usize;
                    let y = (rabbit_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, white);
                    }
                }
            }

            // Pink inner ear
            for dy in -23..-15 {
                let x = (rabbit_x + ear_x_offset).max(0) as usize;
                let y = (rabbit_y + dy).max(0) as usize;
                if x < grid.dot_width() && y < grid.dot_height() {
                    grid.set_dot_with_color(x, y, pink);
                }
            }
        }

        // Eyes (two black dots)
        for eye in 0..2 {
            let eye_x = rabbit_x + if eye == 0 { -4 } else { 4 };
            let eye_y = rabbit_y - 5;
            for dy in 0..3 {
                for dx in 0..3 {
                    let x = (eye_x + dx).max(0) as usize;
                    let y = (eye_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, black);
                    }
                }
            }
        }

        // Fluffy tail
        let tail_x = rabbit_x - 10;
        let tail_y = rabbit_y + 5;
        for dy in -5..5 {
            for dx in -5..5 {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist < 5.0 {
                    let x = (tail_x + dx).max(0) as usize;
                    let y = (tail_y + dy).max(0) as usize;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, white);
                    }
                }
            }
        }

        // Clock (rabbit is late!)
        if progress < 0.8 {
            let clock_x = rabbit_x + 25;
            let clock_y = rabbit_y - 10;

            // Clock face
            for angle in 0..360 {
                let rad = (angle as f32).to_radians();
                let x = (clock_x as f32 + rad.cos() * 8.0) as i32;
                let y = (clock_y as f32 + rad.sin() * 8.0) as i32;
                let px = x.max(0) as usize;
                let py = y.max(0) as usize;
                if px < grid.dot_width() && py < grid.dot_height() {
                    grid.set_dot_with_color(px, py, Color::new(255, 215, 0)); // Gold
                }
            }

            // Clock hands
            for i in 0..8 {
                let x = (clock_x + i * 1).max(0) as usize;
                let y = clock_y.max(0) as usize;
                if x < grid.dot_width() && y < grid.dot_height() {
                    grid.set_dot_with_color(x, y, black);
                }
            }
        }

        // "I'm late!" text trail
        if progress > 0.2 {
            let text_x = (rabbit_x - 40).max(0) as usize;
            let text_y = (rabbit_y - 30).max(0) as usize;

            if text_x < grid.dot_width() && text_y < grid.dot_height() {
                // Draw some motion lines/dust
                for i in 0..5 {
                    let x = text_x + i * 3;
                    let y = text_y + i;
                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, Color::new(200, 200, 200));
                    }
                }
            }
        }
    }

    fn name(&self) -> &str {
        "WhiteRabbit"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }
}
