//! Procedural animations and the terminal player.
//!
//! Implement [`Animation`] to draw into a [`BrailleGrid`](crate::BrailleGrid).
//! [`render_frame`] samples one frame without opening a terminal.
//! [`AnimationPlayer`] plays frames on the tty.
//!
//! `get_computer_fingers_frames` is exported when the `gif` or `video`
//! feature is enabled. It returns the embedded computer-fingers DMD clip.

mod baby;
mod confetti;
mod download;
#[cfg(any(feature = "gif", feature = "video"))]
mod embedded_computer_fingers;
mod fireworks;
mod framebased;
mod matrixrain;
mod merge;
mod rabbit;
mod rocket;
mod save;
mod spinner;
mod trophy;

pub use baby::BabyAnnouncementAnimation;
pub use confetti::ConfettiAnimation;
pub use download::DownloadAnimation;
#[cfg(any(feature = "gif", feature = "video"))]
pub use embedded_computer_fingers::get_computer_fingers_frames;
pub use fireworks::FireworksAnimation;
pub use framebased::FrameBasedAnimation;
pub use matrixrain::MatrixRainAnimation;
pub use merge::MergeAnimation;
pub use rabbit::RabbitAnimation;
pub use rocket::RocketAnimation;
pub use save::SaveAnimation;
pub use spinner::SpinnerAnimation;
pub use trophy::TrophyAnimation;

use crate::braille::BrailleGrid;
use crate::rendering::{RenderMode, TerminalRenderer};
use anyhow::Result;
use std::time::{Duration, Instant};

/// Draw `animation` into a new grid of `width` by `height` cells.
///
/// The animation is not advanced. Call [`Animation::update`] first when the
/// frame should reflect elapsed time.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use crabcrust::animation::{render_frame, RocketAnimation};
/// use crabcrust::Animation;
///
/// let mut rocket = RocketAnimation::new(Duration::from_secs(2));
/// rocket.update(Duration::from_millis(400));
/// let grid = render_frame(&rocket, 40, 16);
/// assert_eq!(grid.width(), 40);
/// ```
pub fn render_frame(animation: &impl Animation, width: usize, height: usize) -> BrailleGrid {
    let mut grid = BrailleGrid::new(width, height);
    animation.render(&mut grid);
    grid
}

/// A procedural terminal animation.
///
/// `update` advances state by `delta_time` and returns `true` while the
/// animation should keep playing. `render` paints the current state into a
/// grid. `name` is a stable label for logs and demos.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use crabcrust::{Animation, BrailleGrid, Color};
///
/// struct Blink { on: bool }
///
/// impl Animation for Blink {
///     fn update(&mut self, _: Duration) -> bool {
///         self.on = !self.on;
///         true
///     }
///
///     fn render(&self, grid: &mut BrailleGrid) {
///         if self.on {
///             grid.set_dot_with_color(1, 1, Color::GREEN);
///         }
///     }
///
///     fn name(&self) -> &str { "blink" }
/// }
///
/// let mut blink = Blink { on: false };
/// assert!(blink.update(Duration::from_millis(16)));
/// assert_eq!(blink.name(), "blink");
/// ```
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
        let height = (terminal_height / 3).clamp(15, 40);

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
    pub fn play_for<A: Animation>(&mut self, mut animation: A, duration: Duration) -> Result<()> {
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
