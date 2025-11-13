// Animation module for procedural terminal animations

mod spinner;
mod rocket;
mod save;
mod download;
mod merge;
mod rabbit;
mod fireworks;
mod baby;
mod confetti;
mod trophy;
mod framebased;

pub use spinner::SpinnerAnimation;
pub use rocket::RocketAnimation;
pub use save::SaveAnimation;
pub use download::DownloadAnimation;
pub use merge::MergeAnimation;
pub use rabbit::RabbitAnimation;
pub use fireworks::FireworksAnimation;
pub use baby::BabyAnnouncementAnimation;
pub use confetti::ConfettiAnimation;
pub use trophy::TrophyAnimation;
pub use framebased::FrameBasedAnimation;

use crate::braille::BrailleGrid;
use crate::rendering::{RenderMode, TerminalRenderer};
use anyhow::Result;
use std::time::{Duration, Instant};

/// Trait for animations
pub trait Animation {
    /// Update animation state
    /// Returns true if animation should continue, false if done
    fn update(&mut self, delta_time: Duration) -> bool;

    /// Render animation to braille grid
    fn render(&self, grid: &mut BrailleGrid);

    /// Get animation name
    fn name(&self) -> &str;

    /// Get total duration (if finite)
    fn duration(&self) -> Option<Duration> {
        None
    }
}

/// Animation player for running animations
pub struct AnimationPlayer {
    renderer: TerminalRenderer,
}

impl AnimationPlayer {
    /// Create a new animation player with fullscreen mode
    pub fn new() -> Result<Self> {
        Self::with_mode(RenderMode::Fullscreen)
    }

    /// Create an animation player with inline mode (for git commands)
    pub fn inline(height: u16) -> Result<Self> {
        Self::with_mode(RenderMode::Inline { height })
    }

    /// Create an animation player with inline mode using optimal height
    /// Automatically uses 1/3 of terminal height (min 15, max 40 lines)
    pub fn inline_auto() -> Result<Self> {
        let (_, terminal_height) = crossterm::terminal::size()?;

        // Use 1/3 of terminal height, with reasonable bounds
        let height = (terminal_height / 3).max(15).min(40);

        Self::with_mode(RenderMode::Inline { height })
    }

    /// Create an animation player with a specific render mode
    pub fn with_mode(mode: RenderMode) -> Result<Self> {
        Ok(Self {
            renderer: TerminalRenderer::with_mode(mode)?,
        })
    }

    /// Play an animation to completion
    pub fn play<A: Animation>(&mut self, mut animation: A) -> Result<()> {
        let (width, height) = self.renderer.size()?;
        let mut grid = BrailleGrid::new(width as usize, height as usize);

        let mut last_frame = Instant::now();
        let target_fps = 60;
        let frame_duration = Duration::from_millis(1000 / target_fps);

        loop {
            let now = Instant::now();
            let delta = now.duration_since(last_frame);

            // Update animation
            let should_continue = animation.update(delta);

            // Render
            grid.clear();
            animation.render(&mut grid);
            self.renderer.render_braille(&grid)?;

            // Check if done
            if !should_continue {
                break;
            }

            // Frame rate limiting
            let elapsed = now.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }

            last_frame = now;
        }

        Ok(())
    }

    /// Play animation for a specific duration
    pub fn play_for<A: Animation>(
        &mut self,
        mut animation: A,
        duration: Duration,
    ) -> Result<()> {
        let (width, height) = self.renderer.size()?;
        let mut grid = BrailleGrid::new(width as usize, height as usize);

        let start = Instant::now();
        let mut last_frame = start;
        let target_fps = 60;
        let frame_duration = Duration::from_millis(1000 / target_fps);

        while start.elapsed() < duration {
            let now = Instant::now();
            let delta = now.duration_since(last_frame);

            // Update animation
            animation.update(delta);

            // Render
            grid.clear();
            animation.render(&mut grid);
            self.renderer.render_braille(&grid)?;

            // Frame rate limiting
            let elapsed = now.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }

            last_frame = now;
        }

        Ok(())
    }

    /// Play animation with timeout while a predicate function returns false
    /// This allows showing animation during command execution with a max timeout
    ///
    /// # Arguments
    /// * `animation` - The animation to play
    /// * `timeout` - Maximum duration to show animation
    /// * `check_done` - Function that returns true when the operation is complete
    ///
    /// # Returns
    /// True if operation completed before timeout, false if timeout was reached
    pub fn play_until<A: Animation, F>(
        &mut self,
        mut animation: A,
        timeout: Duration,
        mut check_done: F,
    ) -> Result<bool>
    where
        F: FnMut() -> bool,
    {
        let (width, height) = self.renderer.size()?;
        let mut grid = BrailleGrid::new(width as usize, height as usize);

        let start = Instant::now();
        let mut last_frame = start;
        let target_fps = 60;
        let frame_duration = Duration::from_millis(1000 / target_fps);

        loop {
            let now = Instant::now();
            let delta = now.duration_since(last_frame);

            // Check if timeout reached
            if start.elapsed() >= timeout {
                return Ok(false); // Timeout reached
            }

            // Check if operation is done
            if check_done() {
                return Ok(true); // Completed before timeout
            }

            // Update animation
            animation.update(delta);

            // Render
            grid.clear();
            animation.render(&mut grid);
            self.renderer.render_braille(&grid)?;

            // Frame rate limiting
            let elapsed = now.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }

            last_frame = now;
        }
    }

    /// Get access to the terminal renderer
    pub fn renderer_mut(&mut self) -> &mut TerminalRenderer {
        &mut self.renderer
    }
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self::new().expect("Failed to create animation player")
    }
}
