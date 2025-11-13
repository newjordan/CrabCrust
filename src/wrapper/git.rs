// Git-specific wrapper with custom animations

use super::CliWrapper;
use crate::animation::{
    BabyAnnouncementAnimation, ConfettiAnimation, DownloadAnimation, FireworksAnimation,
    MergeAnimation, RabbitAnimation, RocketAnimation, SaveAnimation, SpinnerAnimation,
    TrophyAnimation,
};
use crate::executor::{CommandExecutor, CommandResult};
use anyhow::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(any(feature = "gif", feature = "video"))]
use crate::dmd_library;

/// Git command wrapper with themed animations
pub struct GitWrapper {
    wrapper: CliWrapper,
}

impl GitWrapper {
    /// Create a new Git wrapper
    pub fn new() -> Result<Self> {
        Ok(Self {
            wrapper: CliWrapper::new()?,
        })
    }

    /// Execute a git command with appropriate animation
    pub fn run(&mut self, args: &[&str]) -> Result<CommandResult> {
        let executor = CommandExecutor::new("git", args);

        // Determine animation based on git subcommand
        let subcommand = args.first().copied().unwrap_or("");

        match subcommand {
            "commit" => self.run_commit(executor),
            "push" => self.run_push(executor),
            "pull" => self.run_pull(executor),
            "merge" => self.run_merge(executor),
            "status" | "diff" | "log" => self.run_status(executor),
            _ => self.wrapper.run_with_default_animations(executor),
        }
    }

    /// Run git commit with celebration animation
    fn run_commit(&mut self, executor: CommandExecutor) -> Result<CommandResult> {
        use crate::animation::AnimationPlayer;

        // Use inline mode with 1/3 terminal height for proper visibility
        let mut player = AnimationPlayer::inline_auto()?;

        // Execute command in background thread
        let handle = executor.run_concurrent();

        // Show loading animation with timeout (10 seconds)
        let completed_before_timeout = player.play_until(
            SpinnerAnimation::new(),
            Duration::from_secs(10),
            || handle.is_done(),
        )?;

        // If timeout was reached, inform user and clean up animation
        if !completed_before_timeout {
            drop(player);
            println!("Command is taking longer than expected, still running...\n");
            player = AnimationPlayer::inline_auto()?;
        }

        // Wait for command to complete
        let result = handle.wait()?;

        // Show success animation - CONGRATULATIONS, YOU'RE THE FATHER!
        if result.success {
            let random = Self::random_choice(3);
            match random {
                0 => player.play(BabyAnnouncementAnimation::default())?,
                1 => player.play(ConfettiAnimation::default())?,
                _ => player.play(SaveAnimation::default())?,
            }
        }

        // Print output after animation completes
        drop(player); // Clean up renderer
        println!("{}", result.combined_output());

        Ok(result)
    }

    /// Run git push with epic celebration animation
    fn run_push(&mut self, executor: CommandExecutor) -> Result<CommandResult> {
        use crate::animation::AnimationPlayer;

        // Use inline mode with 1/3 terminal height for epic animations
        let mut player = AnimationPlayer::inline_auto()?;

        // Execute command in background thread
        let handle = executor.run_concurrent();

        // Show loading animation with timeout (15 seconds)
        // Animation will stop after timeout even if command is still running
        let completed_before_timeout = player.play_until(
            SpinnerAnimation::new(),
            Duration::from_secs(15),
            || handle.is_done(),
        )?;

        // If timeout was reached, inform user and clean up animation
        if !completed_before_timeout {
            drop(player); // Clean up renderer to reveal terminal
            println!("Command is taking longer than expected, still running...\n");

            // Re-create player for after command completes
            player = AnimationPlayer::inline_auto()?;
        }

        // Wait for command to complete
        let result = handle.wait()?;

        // Show success animation - DMD VICTORY or fallback to procedural
        if result.success {
            #[cfg(any(feature = "gif", feature = "video"))]
            {
                // Try to load DMD animation, fallback to procedural if it fails
                match dmd_library::load_dmd_for_git_command("push", false) {
                    Some(Ok(dmd_anim)) => {
                        player.play(dmd_anim)?;
                    }
                    _ => {
                        // Fallback to procedural animations
                        let random = Self::random_choice(4);
                        match random {
                            0 => player.play(RocketAnimation::new(Duration::from_secs(2)))?,
                            1 => player.play(FireworksAnimation::default())?,
                            2 => player.play(TrophyAnimation::default())?,
                            _ => player.play(ConfettiAnimation::default())?,
                        }
                    }
                }
            }
            #[cfg(not(any(feature = "gif", feature = "video")))]
            {
                let random = Self::random_choice(4);
                match random {
                    0 => player.play(RocketAnimation::new(Duration::from_secs(2)))?,
                    1 => player.play(FireworksAnimation::default())?,
                    2 => player.play(TrophyAnimation::default())?,
                    _ => player.play(ConfettiAnimation::default())?,
                }
            }
        }

        // Print output after animation completes
        drop(player); // Clean up renderer
        println!("{}", result.combined_output());

        Ok(result)
    }

    /// Run git pull with download/rabbit animation
    fn run_pull(&mut self, executor: CommandExecutor) -> Result<CommandResult> {
        use crate::animation::AnimationPlayer;

        // Use inline mode with 1/3 terminal height
        let mut player = AnimationPlayer::inline_auto()?;

        // Execute command in background thread
        let handle = executor.run_concurrent();

        // Show loading animation with timeout (12 seconds)
        let completed_before_timeout = player.play_until(
            SpinnerAnimation::new(),
            Duration::from_secs(12),
            || handle.is_done(),
        )?;

        // If timeout was reached, inform user and clean up animation
        if !completed_before_timeout {
            drop(player);
            println!("Command is taking longer than expected, still running...\n");
            player = AnimationPlayer::inline_auto()?;
        }

        // Wait for command to complete
        let result = handle.wait()?;

        // Show DMD or fallback animation on success
        if result.success {
            #[cfg(any(feature = "gif", feature = "video"))]
            {
                match dmd_library::load_dmd_for_git_command("pull", false) {
                    Some(Ok(dmd_anim)) => {
                        player.play(dmd_anim)?;
                    }
                    _ => {
                        let random = Self::random_choice(2);
                        match random {
                            0 => player.play(DownloadAnimation::default())?,
                            _ => player.play(RabbitAnimation::default())?,
                        }
                    }
                }
            }
            #[cfg(not(any(feature = "gif", feature = "video")))]
            {
                let random = Self::random_choice(2);
                match random {
                    0 => player.play(DownloadAnimation::default())?,
                    _ => player.play(RabbitAnimation::default())?,
                }
            }
        }

        // Print output after animation completes
        drop(player);
        println!("{}", result.combined_output());

        Ok(result)
    }

    /// Run git merge with merge animation
    fn run_merge(&mut self, executor: CommandExecutor) -> Result<CommandResult> {
        use crate::animation::AnimationPlayer;

        // Use inline mode with 1/3 terminal height
        let mut player = AnimationPlayer::inline_auto()?;

        // Execute command in background thread
        let handle = executor.run_concurrent();

        // Show loading animation with timeout (10 seconds)
        let completed_before_timeout = player.play_until(
            SpinnerAnimation::new(),
            Duration::from_secs(10),
            || handle.is_done(),
        )?;

        // If timeout was reached, inform user and clean up animation
        if !completed_before_timeout {
            drop(player);
            println!("Command is taking longer than expected, still running...\n");
            player = AnimationPlayer::inline_auto()?;
        }

        // Wait for command to complete
        let result = handle.wait()?;

        // Show DMD or fallback animation on success
        if result.success {
            #[cfg(any(feature = "gif", feature = "video"))]
            {
                match dmd_library::load_dmd_for_git_command("merge", false) {
                    Some(Ok(dmd_anim)) => {
                        player.play(dmd_anim)?;
                    }
                    _ => {
                        player.play(MergeAnimation::default())?;
                    }
                }
            }
            #[cfg(not(any(feature = "gif", feature = "video")))]
            {
                player.play(MergeAnimation::default())?;
            }
        }

        // Print output after animation completes
        drop(player);
        println!("{}", result.combined_output());

        Ok(result)
    }

    /// Run git status/diff/log with quick DMD animation
    fn run_status(&mut self, executor: CommandExecutor) -> Result<CommandResult> {
        use crate::animation::AnimationPlayer;

        // Use inline mode with 1/3 terminal height
        let mut player = AnimationPlayer::inline_auto()?;

        // Execute command in background thread
        let handle = executor.run_concurrent();

        // Show loading animation with timeout (5 seconds - shorter for status commands)
        let completed_before_timeout = player.play_until(
            SpinnerAnimation::new(),
            Duration::from_secs(5),
            || handle.is_done(),
        )?;

        // If timeout was reached, inform user and clean up animation
        if !completed_before_timeout {
            drop(player);
            println!("Command is taking longer than expected, still running...\n");
            player = AnimationPlayer::inline_auto()?;
        }

        // Wait for command to complete
        let result = handle.wait()?;

        // Show quick DMD animation on success
        if result.success {
            #[cfg(any(feature = "gif", feature = "video"))]
            {
                match dmd_library::load_dmd_for_git_command("status", false) {
                    Some(Ok(dmd_anim)) => {
                        player.play(dmd_anim)?;
                    }
                    _ => {
                        // No fallback animation for status - just return
                    }
                }
            }
        }

        // Print output after animation completes
        drop(player);
        println!("{}", result.combined_output());

        Ok(result)
    }

    /// Get a pseudo-random choice from 0 to max (exclusive)
    fn random_choice(max: usize) -> usize {
        // Simple pseudo-random based on system time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        (now as usize) % max
    }

    /// Execute git command directly (for convenience)
    pub fn commit(&mut self, message: &str) -> Result<CommandResult> {
        self.run(&["commit", "-m", message])
    }

    pub fn push(&mut self) -> Result<CommandResult> {
        self.run(&["push"])
    }

    pub fn pull(&mut self) -> Result<CommandResult> {
        self.run(&["pull"])
    }

    pub fn status(&mut self) -> Result<CommandResult> {
        self.run(&["status"])
    }
}

impl Default for GitWrapper {
    fn default() -> Self {
        Self::new().expect("Failed to create Git wrapper")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_wrapper_creation() {
        let wrapper = GitWrapper::new();
        assert!(wrapper.is_ok());
    }
}
