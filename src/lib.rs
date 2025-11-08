// CrabCrust: Add arcade-style animations to your CLI tools 🦀✨

pub mod braille;
pub mod rendering;
pub mod animation;
pub mod executor;
pub mod wrapper;

// Video conversion (optional feature)
#[cfg(any(feature = "gif", feature = "video"))]
pub mod video;

// Re-export commonly used types
pub use braille::{BrailleGrid, Color};
pub use rendering::{RenderMode, TerminalRenderer};
pub use animation::{
    Animation, AnimationPlayer,
    SpinnerAnimation, RocketAnimation, SaveAnimation, DownloadAnimation, MergeAnimation,
    RabbitAnimation, FireworksAnimation, BabyAnnouncementAnimation, ConfettiAnimation, TrophyAnimation,
    FrameBasedAnimation
};
pub use executor::{CommandExecutor, CommandResult};

// Video conversion utilities (optional feature)
#[cfg(any(feature = "gif", feature = "video"))]
pub use video::{blit_luma_to_braille, converter};

/// CrabCrust result type
pub type Result<T> = std::result::Result<T, anyhow::Error>;
