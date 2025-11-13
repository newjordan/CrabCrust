// DMD Animation Library
//
// Pre-converted pinball Dot Matrix Display animations for git commands.
// This module provides curated DMD animations with lazy loading and caching.

use crate::animation::FrameBasedAnimation;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

#[cfg(any(feature = "gif", feature = "video"))]
use crate::video::converter;

/// DMD animation metadata
#[derive(Debug, Clone)]
pub struct DmdInfo {
    pub name: &'static str,
    pub file_path: &'static str,
    pub description: &'static str,
    pub frames: usize,
}

/// Available DMD animations
pub enum DmdAnimation {
    Invader,
    Skull,
    Sword,
    EyesLook1,
    EyesLook2,
    TeethChomp,
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
        ]
    }
}

/// Git command to DMD animation mapping
pub fn git_command_to_dmd(command: &str) -> Option<DmdAnimation> {
    match command {
        "status" | "diff" | "log" => Some(DmdAnimation::EyesLook1), // Eyes watching/looking at status
        "pull" | "fetch" | "clone" => Some(DmdAnimation::Skull),    // Pulling in changes
        "push" => Some(DmdAnimation::Sword),                         // Victory push!
        "commit" => Some(DmdAnimation::TeethChomp),                  // Chomping down on that commit
        "merge" => Some(DmdAnimation::EyesLook2),                    // Eyes watching the merge
        _ => None,
    }
}

/// DMD animation cache (lazy-loaded)
static DMD_CACHE: OnceLock<HashMap<String, Vec<u8>>> = OnceLock::new();

/// Load a DMD animation by name
#[cfg(any(feature = "gif", feature = "video"))]
pub fn load_dmd_animation(dmd: DmdAnimation, loop_animation: bool) -> Result<FrameBasedAnimation> {
    let info = dmd.info();

    // Construct path relative to project root
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

    Ok(FrameBasedAnimation::from_braille_frames(frames, loop_animation))
}

/// Load a DMD animation for a specific git command
#[cfg(any(feature = "gif", feature = "video"))]
pub fn load_dmd_for_git_command(command: &str, loop_animation: bool) -> Option<Result<FrameBasedAnimation>> {
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
        assert!(matches!(git_command_to_dmd("status"), Some(DmdAnimation::EyesLook1)));
        assert!(matches!(git_command_to_dmd("push"), Some(DmdAnimation::Sword)));
        assert!(matches!(git_command_to_dmd("pull"), Some(DmdAnimation::Skull)));
        assert!(matches!(git_command_to_dmd("commit"), Some(DmdAnimation::TeethChomp)));
        assert!(matches!(git_command_to_dmd("merge"), Some(DmdAnimation::EyesLook2)));
        assert!(git_command_to_dmd("unknown").is_none());
    }

    #[test]
    fn test_list_dmds() {
        let dmds = list_dmds();
        assert_eq!(dmds.len(), 6);
        assert!(dmds.iter().any(|d| d.name == "invader"));
        assert!(dmds.iter().any(|d| d.name == "skull"));
        assert!(dmds.iter().any(|d| d.name == "sword"));
        assert!(dmds.iter().any(|d| d.name == "eyes_look_1"));
        assert!(dmds.iter().any(|d| d.name == "eyes_look_2"));
        assert!(dmds.iter().any(|d| d.name == "teeth_chomp"));
    }

    #[test]
    fn test_dmd_info() {
        let invader = DmdAnimation::Invader.info();
        assert_eq!(invader.name, "invader");
        assert_eq!(invader.frames, 38);
        assert!(invader.file_path.contains("invader"));
    }
}
