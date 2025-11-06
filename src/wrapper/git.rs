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
            _ => self.wrapper.run_with_default_animations(executor),
        }
    }

    /// Run git commit with celebration animation
    fn run_commit(&mut self, executor: CommandExecutor) -> Result<CommandResult> {
        use crate::animation::AnimationPlayer;

        // Use inline mode with 12 lines height
        let mut player = AnimationPlayer::inline(12)?;

        // Show loading animation
        player.play_for(SpinnerAnimation::new(), Duration::from_millis(500))?;

        // Execute command
        let result = executor.run()?;

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

        // Use inline mode with 12 lines height
        let mut player = AnimationPlayer::inline(12)?;

        // Show loading animation
        player.play_for(SpinnerAnimation::new(), Duration::from_millis(500))?;

        // Execute command
        let result = executor.run()?;

        // Show success animation - EPIC CELEBRATION!
        if result.success {
            let random = Self::random_choice(4);
            match random {
                0 => player.play(RocketAnimation::new(Duration::from_secs(2)))?,
                1 => player.play(FireworksAnimation::default())?,
                2 => player.play(TrophyAnimation::default())?,
                _ => player.play(ConfettiAnimation::default())?,
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

        // Use inline mode with 10 lines height
        let mut player = AnimationPlayer::inline(10)?;

        // Show loading animation
        player.play_for(SpinnerAnimation::new(), Duration::from_millis(500))?;

        // Execute command
        let result = executor.run()?;

        // Show download or rabbit animation on success
        if result.success {
            let random = Self::random_choice(2);
            match random {
                0 => player.play(DownloadAnimation::default())?,
                _ => player.play(RabbitAnimation::default())?, // "I'm late! I'm late!"
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

        // Use inline mode with 10 lines height
        let mut player = AnimationPlayer::inline(10)?;

        // Show loading animation
        player.play_for(SpinnerAnimation::new(), Duration::from_millis(500))?;

        // Execute command
        let result = executor.run()?;

        // Show merge animation on success
        if result.success {
            player.play(MergeAnimation::default())?;
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
