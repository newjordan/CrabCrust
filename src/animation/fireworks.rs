// Fireworks animation - celebration time!
use super::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

struct Firework {
    x: f32,
    y: f32,
    particles: Vec<Particle>,
    exploded: bool,
    launch_time: f32,
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    lifetime: f32,
}

/// Fireworks explosion animation
pub struct FireworksAnimation {
    elapsed: Duration,
    total_duration: Duration,
    fireworks: Vec<Firework>,
}

impl FireworksAnimation {
    pub fn new(duration: Duration) -> Self {
        let mut fireworks = Vec::new();

        // Create 5 fireworks at different times
        for i in 0..5 {
            let launch_time = i as f32 * 0.3;
            fireworks.push(Firework {
                x: 50.0 + i as f32 * 50.0,
                y: 200.0, // Start at bottom
                particles: Vec::new(),
                exploded: false,
                launch_time,
            });
        }

        Self {
            elapsed: Duration::ZERO,
            total_duration: duration,
            fireworks,
        }
    }

    fn create_explosion(x: f32, y: f32, color_index: usize) -> Vec<Particle> {
        let mut particles = Vec::new();

        let colors = [
            Color::new(255, 0, 0),     // Red
            Color::new(255, 215, 0),   // Gold
            Color::new(0, 255, 0),     // Green
            Color::new(0, 255, 255),   // Cyan
            Color::new(255, 0, 255),   // Magenta
        ];

        let color = colors[color_index % colors.len()];

        // Create particles in all directions
        for angle in 0..36 {
            let angle_rad = (angle as f32 * 10.0).to_radians();
            for speed in 1..4 {
                particles.push(Particle {
                    x,
                    y,
                    vx: angle_rad.cos() * speed as f32 * 15.0,
                    vy: angle_rad.sin() * speed as f32 * 15.0,
                    color,
                    lifetime: 1.0,
                });
            }
        }

        particles
    }
}

impl Default for FireworksAnimation {
    fn default() -> Self {
        Self::new(Duration::from_millis(3000))
    }
}

impl Animation for FireworksAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;
        let dt = delta_time.as_secs_f32();
        let time = self.elapsed.as_secs_f32();

        // Update fireworks
        for (i, firework) in self.fireworks.iter_mut().enumerate() {
            if time >= firework.launch_time && !firework.exploded {
                // Launch firework upward
                if firework.y > 80.0 {
                    firework.y -= 150.0 * dt;
                } else {
                    // Explode!
                    firework.exploded = true;
                    firework.particles = Self::create_explosion(firework.x, firework.y, i);
                }
            }

            if firework.exploded {
                // Update particles
                for particle in &mut firework.particles {
                    particle.x += particle.vx * dt;
                    particle.y += particle.vy * dt;
                    particle.vy += 50.0 * dt; // Gravity
                    particle.lifetime -= dt * 0.5;
                }

                // Remove dead particles
                firework.particles.retain(|p| p.lifetime > 0.0);
            }
        }

        self.elapsed < self.total_duration
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = grid.dot_width() / 2;
        let center_y = grid.dot_height() / 2;

        // Draw each firework
        for firework in &self.fireworks {
            if !firework.exploded {
                // Draw launching rocket
                let x = (firework.x) as usize;
                let y = (firework.y) as usize;

                if x < grid.dot_width() && y < grid.dot_height() {
                    // Rocket body
                    for dy in 0..8 {
                        if y + dy < grid.dot_height() {
                            grid.set_dot_with_color(x, y + dy, Color::new(255, 255, 255));
                        }
                    }

                    // Flame trail
                    for dy in 8..15 {
                        if y + dy < grid.dot_height() {
                            let color = if dy % 2 == 0 {
                                Color::new(255, 100, 0)
                            } else {
                                Color::new(255, 200, 0)
                            };
                            grid.set_dot_with_color(x, y + dy, color);
                        }
                    }
                }
            } else {
                // Draw explosion particles
                for particle in &firework.particles {
                    let x = particle.x as usize;
                    let y = particle.y as usize;

                    if x < grid.dot_width() && y < grid.dot_height() {
                        // Fade color based on lifetime
                        let alpha = particle.lifetime.max(0.0).min(1.0);
                        let color = Color::new(
                            (particle.color.r as f32 * alpha) as u8,
                            (particle.color.g as f32 * alpha) as u8,
                            (particle.color.b as f32 * alpha) as u8,
                        );
                        grid.set_dot_with_color(x, y, color);

                        // Larger particles for effect
                        if x + 1 < grid.dot_width() {
                            grid.set_dot_with_color(x + 1, y, color);
                        }
                        if y + 1 < grid.dot_height() {
                            grid.set_dot_with_color(x, y + 1, color);
                        }
                    }
                }
            }
        }

        // Draw "BOOM!" text when explosions happen
        let explosions_count = self.fireworks.iter().filter(|f| f.exploded).count();
        if explosions_count > 0 {
            // Draw stars around the edges for extra celebration
            for i in 0..20 {
                let angle = (self.elapsed.as_secs_f32() * 2.0 + i as f32 * 18.0).to_radians();
                let radius = 60.0;
                let x = (center_x as f32 + angle.cos() * radius) as usize;
                let y = (center_y as f32 + angle.sin() * radius / 2.0) as usize;

                if x < grid.dot_width() && y < grid.dot_height() {
                    grid.set_dot_with_color(x, y, Color::new(255, 255, 0));
                }
            }
        }
    }

    fn name(&self) -> &str {
        "Fireworks"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }
}
