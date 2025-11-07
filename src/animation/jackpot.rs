// Classic pinball JACKPOT animation
//
// Big flashing text with coins raining down and dollar signs
// Just like hitting that sweet jackpot on a pinball machine!

use crate::animation::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

const COIN_COUNT: usize = 30;

pub struct JackpotAnimation {
    elapsed: Duration,
    duration: Duration,
    coins: Vec<Coin>,
    flash_state: bool,
    flash_timer: Duration,
}

struct Coin {
    x: i32,
    y: i32,
    vy: f32,
    rotation: f32,
    rotation_speed: f32,
}

impl JackpotAnimation {
    pub fn new(duration: Duration) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut coins = Vec::new();
        for i in 0..COIN_COUNT {
            let offset = (seed + i as u128 * 1000) as f32;
            coins.push(Coin {
                x: ((offset * 0.1).sin() * 200.0) as i32,
                y: -50 - (i as i32 * 10),
                vy: (offset * 0.01).sin() * 2.0 + 3.0,
                rotation: offset * 0.1,
                rotation_speed: (offset * 0.05).sin() * 0.2 + 0.15,
            });
        }

        Self {
            elapsed: Duration::ZERO,
            duration,
            coins,
            flash_state: true,
            flash_timer: Duration::ZERO,
        }
    }

    fn draw_big_text(&self, grid: &mut BrailleGrid, text: &str, y_offset: i32, flash: bool) {
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        let color = if flash {
            Color::new(255, 215, 0) // Gold
        } else {
            Color::new(255, 255, 100) // Lighter yellow
        };

        // Draw large blocky text
        let char_width = 12;
        let char_height = 16;
        let text_width = (text.len() as i32 * char_width) + ((text.len() as i32 - 1) * 3);
        let start_x = center_x - text_width / 2;

        for (i, ch) in text.chars().enumerate() {
            let x = start_x + (i as i32 * (char_width + 3));
            let y = center_y + y_offset;

            self.draw_char(grid, ch, x, y, char_width, char_height, color);
        }
    }

    fn draw_char(&self, grid: &mut BrailleGrid, ch: char, x: i32, y: i32, w: i32, h: i32, color: Color) {
        match ch {
            'J' => {
                // Top bar
                for dx in 0..w {
                    grid.set_dot_with_color((x + dx) as usize, y as usize, color);
                }
                // Vertical line on right
                for dy in 0..h {
                    grid.set_dot_with_color((x + w - 2) as usize, (y + dy) as usize, color);
                }
                // Bottom curve
                for dx in 0..4 {
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 1) as usize, color);
                }
                for dy in (h - 4)..h {
                    grid.set_dot_with_color(x as usize, (y + dy) as usize, color);
                }
            }
            'A' => {
                // Top point
                for dy in 0..h {
                    let offset = (h - dy) / 2;
                    grid.set_dot_with_color((x + offset) as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + w - offset) as usize, (y + dy) as usize, color);
                }
                // Middle bar
                for dx in 3..(w - 3) {
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2) as usize, color);
                }
            }
            'C' => {
                // Left vertical
                for dy in 2..(h - 2) {
                    grid.set_dot_with_color(x as usize, (y + dy) as usize, color);
                }
                // Top and bottom arcs
                for dx in 1..w {
                    grid.set_dot_with_color((x + dx) as usize, y as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 1) as usize, color);
                }
            }
            'K' => {
                // Left vertical
                for dy in 0..h {
                    grid.set_dot_with_color(x as usize, (y + dy) as usize, color);
                }
                // Diagonal up
                for i in 0..(h / 2) {
                    let dx = (i * 2).min(w - 1);
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2 - i) as usize, color);
                }
                // Diagonal down
                for i in 0..(h / 2) {
                    let dx = (i * 2).min(w - 1);
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2 + i) as usize, color);
                }
            }
            'P' => {
                // Left vertical
                for dy in 0..h {
                    grid.set_dot_with_color(x as usize, (y + dy) as usize, color);
                }
                // Top horizontal
                for dx in 0..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, y as usize, color);
                }
                // Right vertical (top half)
                for dy in 0..(h / 2) {
                    grid.set_dot_with_color((x + w - 2) as usize, (y + dy) as usize, color);
                }
                // Middle horizontal
                for dx in 0..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, (y + h / 2) as usize, color);
                }
            }
            'O' => {
                // Top and bottom
                for dx in 2..(w - 2) {
                    grid.set_dot_with_color((x + dx) as usize, y as usize, color);
                    grid.set_dot_with_color((x + dx) as usize, (y + h - 1) as usize, color);
                }
                // Sides
                for dy in 2..(h - 2) {
                    grid.set_dot_with_color(x as usize, (y + dy) as usize, color);
                    grid.set_dot_with_color((x + w - 1) as usize, (y + dy) as usize, color);
                }
            }
            'T' => {
                // Top bar
                for dx in 0..w {
                    grid.set_dot_with_color((x + dx) as usize, y as usize, color);
                }
                // Center vertical
                for dy in 0..h {
                    grid.set_dot_with_color((x + w / 2) as usize, (y + dy) as usize, color);
                }
            }
            _ => {}
        }
    }

    fn draw_coin(&self, grid: &mut BrailleGrid, coin: &Coin) {
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        let x = center_x + coin.x;
        let y = center_y + coin.y;

        if y < 0 || y >= grid.dot_height() as i32 {
            return;
        }

        let color = Color::new(255, 215, 0); // Gold

        // Coin shape (changes with rotation)
        let width = (6.0 * coin.rotation.cos().abs() + 2.0) as i32;

        for dy in -3..3 {
            for dx in -width..width {
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < 12 {
                    let px = x + dx;
                    let py = y + dy;
                    if px >= 0 && px < grid.dot_width() as i32 && py >= 0 && py < grid.dot_height() as i32 {
                        grid.set_dot_with_color(px as usize, py as usize, color);
                    }
                }
            }
        }

        // Dollar sign
        if width > 3 {
            for dy in -2..2 {
                grid.set_dot_with_color(x as usize, (y + dy) as usize, Color::new(100, 50, 0));
            }
        }
    }
}

impl Animation for JackpotAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;

        if self.elapsed >= self.duration {
            return false;
        }

        // Update flash
        self.flash_timer += delta_time;
        if self.flash_timer >= Duration::from_millis(150) {
            self.flash_state = !self.flash_state;
            self.flash_timer = Duration::ZERO;
        }

        // Update coins
        let dt = delta_time.as_secs_f32();
        for coin in &mut self.coins {
            coin.y += (coin.vy * 60.0 * dt) as i32;
            coin.vy += 9.8 * dt; // Gravity
            coin.rotation += coin.rotation_speed * dt * 10.0;

            // Reset coin if it falls too far
            if coin.y > 100 {
                coin.y = -50;
                coin.vy = 3.0;
            }
        }

        true
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = (grid.dot_width() / 2) as i32;
        let center_y = (grid.dot_height() / 2) as i32;

        // Draw raining coins
        for coin in &self.coins {
            self.draw_coin(grid, coin);
        }

        // Draw "JACKPOT!" text
        self.draw_big_text(grid, "JACKPOT", -8, self.flash_state);

        // Draw sparkles
        let sparkle_color = Color::new(255, 255, 255);
        let time = self.elapsed.as_secs_f32();
        for i in 0..20 {
            let angle = (i as f32 * 0.314 + time * 2.0) % 6.28;
            let radius = 50.0 + (time * 10.0).sin() * 10.0;
            let x = center_x + (angle.cos() * radius) as i32;
            let y = center_y + (angle.sin() * radius * 0.5) as i32;

            if x >= 0 && x < grid.dot_width() as i32 && y >= 0 && y < grid.dot_height() as i32 {
                grid.set_dot_with_color(x as usize, y as usize, sparkle_color);
            }
        }
    }

    fn name(&self) -> &str {
        "Jackpot"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.duration)
    }
}

impl Default for JackpotAnimation {
    fn default() -> Self {
        Self::new(Duration::from_secs(3))
    }
}
