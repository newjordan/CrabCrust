// Wrapper module for integrating animations with CLI commands

pub mod git;

use crate::animation::{AnimationPlayer, SaveAnimation, SpinnerAnimation};
use crate::executor::{CommandExecutor, CommandResult};
use anyhow::Result;
use std::time::Duration;

/// Wrapper for CLI commands with animations
pub struct CliWrapper;

impl CliWrapper {
    /// Create a new CLI wrapper
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Run a command with default animations based on success/failure
    pub fn run_with_default_animations(
        &mut self,
        executor: CommandExecutor,
    ) -> Result<CommandResult> {
        // Create inline player with 8 lines
        let mut player = AnimationPlayer::inline(8)?;

        // Show loading animation while command runs
        player.play_for(SpinnerAnimation::new(), Duration::from_millis(500))?;

        // Execute command
        let result = executor.run()?;

        // Show success or error animation
        if result.success {
            player.play(SaveAnimation::default())?;
        } else {
            player.play_for(SpinnerAnimation::new(), Duration::from_millis(500))?;
        }

        // Print output after animation
        drop(player);
        println!("{}", result.combined_output());

        Ok(result)
    }
}

impl Default for CliWrapper {
    fn default() -> Self {
        Self::new().expect("Failed to create CLI wrapper")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_wrapper_creation() {
        let wrapper = CliWrapper::new();
        assert!(wrapper.is_ok());
    }
}
