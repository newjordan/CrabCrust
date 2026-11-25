//! Arcade-style terminal animations for command-line tools.
//!
//! CrabCrust draws with Unicode braille, so each terminal cell holds a 2×4
//! dot grid. A command wrapper runs the real program, then plays a procedural
//! animation for that outcome. The process result is preserved:
//! [`CommandResult`] still carries stdout, stderr, and the exit code.
//!
//! # Render without a terminal
//!
//! Animations implement [`Animation`] and paint a [`BrailleGrid`]. Tests,
//! snapshots, and the `text_frames` example use this path, so they do not
//! need a tty.
//!
//! ```
//! use std::time::Duration;
//! use crabcrust::{Animation, BrailleGrid, SpinnerAnimation};
//!
//! let mut spinner = SpinnerAnimation::new();
//! spinner.update(Duration::from_millis(200));
//! let mut grid = BrailleGrid::new(24, 10);
//! spinner.render(&mut grid);
//! assert!(!grid.to_trimmed_text().is_empty());
//! ```
//!
//! # Play in a terminal
//!
//! [`AnimationPlayer`](animation::AnimationPlayer) owns the screen.
//! [`AnimationPlayer::inline`](animation::AnimationPlayer::inline) keeps
//! scrollback. [`AnimationPlayer::new`](animation::AnimationPlayer::new)
//! switches to the alternate screen.
//!
//! # Command wrappers
//!
//! [`CommandExecutor`] runs a program and captures its output.
//! [`CliWrapper`](wrapper::CliWrapper) and [`GitWrapper`](wrapper::git::GitWrapper)
//! pair that execution with an animation.

#![deny(missing_docs)]

/// Procedural animations and the player that runs them.
pub mod animation;
/// Braille dots, colors, and the grid animations draw into.
pub mod braille;
/// Process execution for command wrappers.
pub mod executor;
/// Fullscreen and inline terminal backends.
pub mod rendering;
/// Wrappers that pair a process with an animation.
pub mod wrapper;

/// Video, GIF, and Pin2DMD conversion.
///
/// Enabled with the `gif` or `video` feature. Video files also need `video`
/// and a system FFmpeg install.
#[cfg(any(feature = "gif", feature = "video"))]
pub mod video;

/// Curated pinball DMD clips for git commands.
///
/// Metadata is always available. Loading frames from disk needs the `gif`
/// or `video` feature.
#[cfg(any(feature = "gif", feature = "video"))]
pub mod dmd_library;

pub use animation::{
    render_frame, Animation, AnimationPlayer, BabyAnnouncementAnimation, ConfettiAnimation,
    DownloadAnimation, FireworksAnimation, FrameBasedAnimation, MatrixRainAnimation,
    MergeAnimation, RabbitAnimation, RocketAnimation, SaveAnimation, SpinnerAnimation,
    TrophyAnimation,
};
pub use braille::{BrailleGrid, Color};
pub use executor::{CommandExecutor, CommandResult};
pub use rendering::{RenderMode, TerminalRenderer};

#[cfg(any(feature = "gif", feature = "video"))]
pub use video::{blit_luma_to_braille, converter};

/// CrabCrust result type.
pub type Result<T> = std::result::Result<T, anyhow::Error>;
