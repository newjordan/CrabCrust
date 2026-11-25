//! Curated pinball DMD clips for git commands.
//!
//! [`list_dmds`] and [`git_command_to_dmd`] describe the library without
//! reading files. [`load_dmd_animation`] decodes a clip and needs the `gif`
//! or `video` feature.

#[cfg(any(feature = "gif", feature = "video"))]
use crate::animation::{get_computer_fingers_frames, FrameBasedAnimation};
#[cfg(any(feature = "gif", feature = "video"))]
use anyhow::Result;
#[cfg(any(feature = "gif", feature = "video"))]
use std::path::PathBuf;

#[cfg(any(feature = "gif", feature = "video"))]
use crate::video::converter;

/// Metadata for one DMD clip.
#[derive(Debug, Clone)]
pub struct DmdInfo {
    /// Short identifier, such as `"invader"`.
    pub name: &'static str,
    /// Path of the source asset, relative to the repository root.
    pub file_path: &'static str,
    /// Human-readable summary of the clip.
    pub description: &'static str,
    /// Frame count when it is known at compile time. Zero means "count at load".
    pub frames: usize,
}

/// DMD clips that git wrappers can play.
#[derive(Copy, Clone)]
pub enum DmdAnimation {
    /// Monster Bash invader.
    Invader,
    /// Monster Bash skull.
    Skull,
    /// Monster Bash sword.
    Sword,
    /// Eyes looking around, first take.
    EyesLook1,
    /// Eyes looking around, second take.
    EyesLook2,
    /// Chomping teeth.
    TeethChomp,
    /// Terminator lock-in startup.
    TerminatorStartup,
    /// Hands typing on a keyboard.
    ComputerFingers,
}

impl DmdAnimation {
    /// Get DMD info
    pub fn info(&self) -> DmdInfo {
        match self {
            DmdAnimation::Invader => DmdInfo {
                name: "invader",
                file_path: "ref/dmd_invader.gif",
                description: "Monster Bash invader character (38 frames, quick & energetic)",
                frames: 38,
            },
            DmdAnimation::Skull => DmdInfo {
                name: "skull",
                file_path: "ref/monster_bash_dmd.gif",
                description: "Monster Bash skull character (63 frames, action sequence)",
                frames: 63,
            },
            DmdAnimation::Sword => DmdInfo {
                name: "sword",
                file_path: "ref/dmd_sword.gif",
                description: "Monster Bash sword character (42 frames, victory pose)",
                frames: 42,
            },
            DmdAnimation::EyesLook1 => DmdInfo {
                name: "eyes_look_1",
                file_path: "ref/eyes_look_around_1.mp4",
                description: "Monster Bash eyes looking around animation",
                frames: 0, // Will be determined during conversion
            },
            DmdAnimation::EyesLook2 => DmdInfo {
                name: "eyes_look_2",
                file_path: "ref/eyes_look_2.mp4",
                description: "Monster Bash eyes looking animation",
                frames: 0, // Will be determined during conversion
            },
            DmdAnimation::TeethChomp => DmdInfo {
                name: "teeth_chomp",
                file_path: "ref/teeth_chomp.mp4",
                description: "Monster Bash teeth chomping animation",
                frames: 0, // Will be determined during conversion
            },
            DmdAnimation::TerminatorStartup => DmdInfo {
                name: "terminator_startup",
                file_path: "ref/terminator_lcok_in.gif",
                description: "Terminator lock-in startup animation",
                frames: 0, // Will be determined during conversion
            },
            DmdAnimation::ComputerFingers => DmdInfo {
                name: "computer_fingers",
                file_path: "ref/computer_fingers.mp4",
                description: "Computer fingers typing animation for git push",
                frames: 0, // Will be determined during conversion
            },
        }
    }

    /// Get all available DMDs
    pub fn all() -> Vec<DmdAnimation> {
        vec![
            DmdAnimation::Invader,
            DmdAnimation::Skull,
            DmdAnimation::Sword,
            DmdAnimation::EyesLook1,
            DmdAnimation::EyesLook2,
            DmdAnimation::TeethChomp,
            DmdAnimation::TerminatorStartup,
            DmdAnimation::ComputerFingers,
        ]
    }
}

/// Git command to DMD animation mapping
pub fn git_command_to_dmd(command: &str) -> Option<DmdAnimation> {
    match command {
        "status" | "diff" | "log" => Some(DmdAnimation::EyesLook1), // Eyes watching/looking at status
        "pull" | "fetch" | "clone" => Some(DmdAnimation::Invader),  // Quick & energetic invader!
        "push" => Some(DmdAnimation::ComputerFingers), // Computer fingers typing - push complete!
        "commit" => Some(DmdAnimation::TeethChomp),    // Chomping down on that commit
        "merge" => Some(DmdAnimation::EyesLook2),      // Eyes watching the merge
        _ => None,
    }
}

/// Load a DMD animation by name
#[cfg(any(feature = "gif", feature = "video"))]
pub fn load_dmd_animation(dmd: DmdAnimation, loop_animation: bool) -> Result<FrameBasedAnimation> {
    let info = dmd.info();

    // Check for pre-converted embedded animations first (works globally without FFmpeg)
    if let DmdAnimation::ComputerFingers = dmd {
        let frames = get_computer_fingers_frames();
        return Ok(FrameBasedAnimation::from_braille_frames(
            frames,
            loop_animation,
        ));
    }

    // Fall back to file-based loading for other animations
    let project_root = std::env::current_exe()?
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    let file_path = project_root.join(info.file_path);

    // Check if file exists, fall back to relative path if not
    let file_path = if file_path.exists() {
        file_path
    } else {
        PathBuf::from(info.file_path)
    };

    // Convert to Braille frames based on file extension
    // DMD animations: 124x19 cells = 248x76 dots (preserves detail)
    // Threshold 50: Lower value = more pixels visible (good for dark orange DMD colors)
    let frames = if info.file_path.ends_with(".gif") {
        converter::gif_to_frames(&file_path, 124, 19, 50)?
    } else if info.file_path.ends_with(".mp4") {
        #[cfg(feature = "video")]
        {
            converter::video_to_frames(&file_path, 124, 19, 50, None)?
        }
        #[cfg(not(feature = "video"))]
        {
            anyhow::bail!("Video support not enabled. Build with --features video")
        }
    } else {
        anyhow::bail!("Unsupported file format: {}", info.file_path)
    };

    Ok(FrameBasedAnimation::from_braille_frames(
        frames,
        loop_animation,
    ))
}

/// Load a DMD animation for a specific git command
#[cfg(any(feature = "gif", feature = "video"))]
pub fn load_dmd_for_git_command(
    command: &str,
    loop_animation: bool,
) -> Option<Result<FrameBasedAnimation>> {
    git_command_to_dmd(command).map(|dmd| load_dmd_animation(dmd, loop_animation))
}

/// List all available DMD animations
pub fn list_dmds() -> Vec<DmdInfo> {
    DmdAnimation::all().iter().map(|dmd| dmd.info()).collect()
}

/// Get DMD recommendation for git command
pub fn get_dmd_recommendation(command: &str) -> Option<DmdInfo> {
    git_command_to_dmd(command).map(|dmd| dmd.info())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_command_mapping() {
        assert!(matches!(
            git_command_to_dmd("status"),
            Some(DmdAnimation::EyesLook1)
        ));
        assert!(matches!(
            git_command_to_dmd("push"),
            Some(DmdAnimation::ComputerFingers)
        ));
        assert!(matches!(
            git_command_to_dmd("pull"),
            Some(DmdAnimation::Invader)
        ));
        assert!(matches!(
            git_command_to_dmd("commit"),
            Some(DmdAnimation::TeethChomp)
        ));
        assert!(matches!(
            git_command_to_dmd("merge"),
            Some(DmdAnimation::EyesLook2)
        ));
        assert!(git_command_to_dmd("unknown").is_none());
    }

    #[test]
    fn test_list_dmds() {
        let dmds = list_dmds();
        assert_eq!(dmds.len(), 8);
        assert!(dmds.iter().any(|d| d.name == "invader"));
        assert!(dmds.iter().any(|d| d.name == "skull"));
        assert!(dmds.iter().any(|d| d.name == "sword"));
        assert!(dmds.iter().any(|d| d.name == "eyes_look_1"));
        assert!(dmds.iter().any(|d| d.name == "eyes_look_2"));
        assert!(dmds.iter().any(|d| d.name == "teeth_chomp"));
        assert!(dmds.iter().any(|d| d.name == "terminator_startup"));
        assert!(dmds.iter().any(|d| d.name == "computer_fingers"));
    }

    #[test]
    fn test_dmd_info() {
        let invader = DmdAnimation::Invader.info();
        assert_eq!(invader.name, "invader");
        assert_eq!(invader.frames, 38);
        assert!(invader.file_path.contains("invader"));
    }
}
