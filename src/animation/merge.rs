// Merge animation for git merge
use super::Animation;
use crate::braille::{BrailleGrid, Color};
use std::time::Duration;

/// Merge animation showing two branches coming together
pub struct MergeAnimation {
    elapsed: Duration,
    total_duration: Duration,
}

impl MergeAnimation {
    pub fn new(duration: Duration) -> Self {
        Self {
            elapsed: Duration::ZERO,
            total_duration: duration,
        }
    }
}

impl Default for MergeAnimation {
    fn default() -> Self {
        Self::new(Duration::from_millis(1500))
    }
}

impl Animation for MergeAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;
        self.elapsed < self.total_duration
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = grid.dot_width() / 2;
        let center_y = grid.dot_height() / 2;

        let progress = (self.elapsed.as_secs_f32() / self.total_duration.as_secs_f32()).min(1.0);

        // Phase 1: Show two separate branches (0.0 - 0.3)
        // Phase 2: Branches come together (0.3 - 0.7)
        // Phase 3: Merged branch (0.7 - 1.0)

        let branch1_color = Color::new(100, 200, 255); // Blue (main branch)
        let branch2_color = Color::new(255, 180, 50);  // Orange (feature branch)
        let merge_color = Color::new(150, 255, 150);   // Green (merged)

        if progress < 0.7 {
            // Draw main branch (straight line on left going right)
            let branch1_start_x = center_x - 80;
            let branch1_end_x = center_x + (progress * 100.0) as usize;
            let y1 = center_y - 20;

            for x in branch1_start_x..branch1_end_x {
                if x < grid.dot_width() && y1 < grid.dot_height() {
                    grid.set_dot_with_color(x, y1, branch1_color);
                    if y1 + 1 < grid.dot_height() {
                        grid.set_dot_with_color(x, y1 + 1, branch1_color);
                    }
                }
            }

            // Draw feature branch (curved line from top)
            let branch2_progress = progress.min(0.6) / 0.6;
            let branch2_start_x = center_x - 40;
            let branch2_start_y = center_y - 60;

            for t in 0..100 {
                let t_norm = t as f32 / 100.0;
                if t_norm > branch2_progress {
                    break;
                }

                // Cubic bezier curve
                let x = branch2_start_x as f32 + t_norm * 60.0;
                let y_offset = (t_norm * std::f32::consts::PI).sin() * 40.0;
                let y = branch2_start_y as f32 + y_offset + t_norm * 40.0;

                let xi = x as usize;
                let yi = y as usize;

                if xi < grid.dot_width() && yi < grid.dot_height() {
                    grid.set_dot_with_color(xi, yi, branch2_color);
                    if yi + 1 < grid.dot_height() {
                        grid.set_dot_with_color(xi, yi + 1, branch2_color);
                    }
                }
            }

            // Draw merge point circle when branches are coming together
            if progress > 0.5 {
                let merge_progress = ((progress - 0.5) / 0.2).min(1.0);
                let radius = (merge_progress * 8.0) as usize;

                for angle in 0..360 {
                    let rad = (angle as f32).to_radians();
                    let x = (center_x as i32 + (rad.cos() * radius as f32) as i32).max(0) as usize;
                    let y = (y1 as i32 + (rad.sin() * radius as f32) as i32).max(0) as usize;

                    if x < grid.dot_width() && y < grid.dot_height() {
                        grid.set_dot_with_color(x, y, merge_color);
                    }
                }
            }
        } else {
            // Phase 3: Show completed merge
            let merged_start_x = center_x - 80;
            let merged_end_x = center_x + 80;
            let y = center_y;

            // Draw merged branch as thick green line
            for x in merged_start_x..merged_end_x {
                if x < grid.dot_width() {
                    for dy in 0..3 {
                        let y_pos = y.saturating_sub(1) + dy;
                        if y_pos < grid.dot_height() {
                            grid.set_dot_with_color(x, y_pos, merge_color);
                        }
                    }
                }
            }

            // Draw checkmark at the end
            let check_x = merged_end_x + 10;
            let check_y = y;

            // Checkmark using line segments
            for i in 0..8 {
                let x1 = check_x + i;
                let y1 = check_y + i;
                if x1 < grid.dot_width() && y1 < grid.dot_height() {
                    grid.set_dot_with_color(x1, y1, Color::new(0, 255, 0));
                }
            }

            for i in 0..15 {
                let x2 = check_x + 8 + i;
                let y2 = (check_y + 8).saturating_sub(i);
                if x2 < grid.dot_width() && y2 < grid.dot_height() {
                    grid.set_dot_with_color(x2, y2, Color::new(0, 255, 0));
                }
            }
        }

        // Draw labels
        let label = if progress < 0.7 {
            "Merging branches..."
        } else {
            "Merge successful!"
        };

        // Simple label rendering (centered at bottom)
        let label_y = center_y + 70;
        let label_x = center_x.saturating_sub(label.len() * 4);

        // Note: Text rendering would need proper font support
        // For now, just skip the text rendering in the animation
        let _ = (label, label_x, label_y); // Suppress unused warnings
    }

    fn name(&self) -> &str {
        "Merge"
    }

    fn duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }
}
