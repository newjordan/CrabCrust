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
    pub tags: &'static [&'static str],
    pub theme: &'static str,
}

/// Available DMD animations
pub enum DmdAnimation {
    Invader,
    Skull,
    Sword,
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
                tags: &["action", "horror", "monster"],
                theme: "Monster Bash",
            },
            DmdAnimation::Skull => DmdInfo {
                name: "skull",
                file_path: "ref/monster_bash_dmd.gif",
                description: "Monster Bash skull character (63 frames, action sequence)",
                frames: 63,
                tags: &["action", "horror", "monster"],
                theme: "Monster Bash",
            },
            DmdAnimation::Sword => DmdInfo {
                name: "sword",
                file_path: "ref/dmd_sword.gif",
                description: "Monster Bash sword character (42 frames, victory pose)",
                frames: 42,
                tags: &["celebration", "victory", "monster"],
                theme: "Monster Bash",
            },
        }
    }

    /// Get all available DMDs
    pub fn all() -> Vec<DmdAnimation> {
        vec![
            DmdAnimation::Invader,
            DmdAnimation::Skull,
            DmdAnimation::Sword,
        ]
    }
}

/// Git command to DMD animation mapping
pub fn git_command_to_dmd(command: &str) -> Option<DmdAnimation> {
    match command {
        "status" | "diff" | "log" => Some(DmdAnimation::Invader),   // Quick status check
        "pull" | "fetch" | "clone" => Some(DmdAnimation::Skull),    // Pulling in changes
        "push" | "merge" => Some(DmdAnimation::Sword),              // Victory/celebration
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

    let gif_path = project_root.join(info.file_path);

    // Check if file exists, fall back to relative path if not
    let gif_path = if gif_path.exists() {
        gif_path
    } else {
        PathBuf::from(info.file_path)
    };

    // Convert GIF to Braille frames
    // These Tenor GIFs are ~498x150, so we use larger cells to preserve detail
    // 124x19 cells = 248x76 dots (closer to source resolution, better quality)
    // Threshold 50: Lower value = more pixels visible (good for dark orange DMD colors)
    let frames = converter::gif_to_frames(&gif_path, 124, 19, 50)?;

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

/// Get all unique tags from all DMDs
pub fn get_all_tags() -> Vec<String> {
    let mut tags = std::collections::HashSet::new();
    for dmd in DmdAnimation::all() {
        for tag in dmd.info().tags {
            tags.insert(tag.to_string());
        }
    }
    let mut tags: Vec<String> = tags.into_iter().collect();
    tags.sort();
    tags
}

/// Get all unique themes
pub fn get_all_themes() -> Vec<String> {
    let mut themes = std::collections::HashSet::new();
    for dmd in DmdAnimation::all() {
        themes.insert(dmd.info().theme.to_string());
    }
    let mut themes: Vec<String> = themes.into_iter().collect();
    themes.sort();
    themes
}

/// Filter DMDs by tag
pub fn filter_by_tag(tag: &str) -> Vec<DmdInfo> {
    DmdAnimation::all()
        .iter()
        .map(|dmd| dmd.info())
        .filter(|info| info.tags.contains(&tag))
        .collect()
}

/// Filter DMDs by theme
pub fn filter_by_theme(theme: &str) -> Vec<DmdInfo> {
    DmdAnimation::all()
        .iter()
        .map(|dmd| dmd.info())
        .filter(|info| info.theme == theme)
        .collect()
}

/// Find DMD by name
pub fn find_by_name(name: &str) -> Option<DmdAnimation> {
    DmdAnimation::all()
        .into_iter()
        .find(|dmd| dmd.info().name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_command_mapping() {
        assert!(matches!(git_command_to_dmd("status"), Some(DmdAnimation::Invader)));
        assert!(matches!(git_command_to_dmd("push"), Some(DmdAnimation::Sword)));
        assert!(matches!(git_command_to_dmd("pull"), Some(DmdAnimation::Skull)));
        assert!(git_command_to_dmd("unknown").is_none());
    }

    #[test]
    fn test_list_dmds() {
        let dmds = list_dmds();
        assert_eq!(dmds.len(), 3);
        assert!(dmds.iter().any(|d| d.name == "invader"));
        assert!(dmds.iter().any(|d| d.name == "skull"));
        assert!(dmds.iter().any(|d| d.name == "sword"));
    }

    #[test]
    fn test_dmd_info() {
        let invader = DmdAnimation::Invader.info();
        assert_eq!(invader.name, "invader");
        assert_eq!(invader.frames, 38);
        assert!(invader.file_path.contains("invader"));
        assert_eq!(invader.theme, "Monster Bash");
        assert!(invader.tags.contains(&"action"));
    }

    #[test]
    fn test_filter_by_tag() {
        let action_dmds = filter_by_tag("action");
        assert!(action_dmds.len() >= 2); // invader and skull

        let celebration_dmds = filter_by_tag("celebration");
        assert!(celebration_dmds.len() >= 1); // sword
    }

    #[test]
    fn test_find_by_name() {
        assert!(find_by_name("invader").is_some());
        assert!(find_by_name("skull").is_some());
        assert!(find_by_name("nonexistent").is_none());
    }
}
