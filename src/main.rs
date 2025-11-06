// CrabCrust: Add arcade-style animations to your CLI tools 🦀✨

use anyhow::Result;
use clap::{Parser, Subcommand};
use crabcrust::wrapper::git::GitWrapper;
use crabcrust::{
    AnimationPlayer, BabyAnnouncementAnimation, ConfettiAnimation, DownloadAnimation,
    FireworksAnimation, MergeAnimation, RabbitAnimation, RocketAnimation, SaveAnimation,
    SpinnerAnimation, TrophyAnimation,
};
use std::time::Duration;

#[derive(Parser)]
#[command(name = "crabcrust")]
#[command(about = "Add arcade-style animations to your CLI tools 🦀✨", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Git wrapper with animations
    Git {
        /// Git arguments (e.g., commit -m "message")
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Test animations
    Demo {
        /// Which animation to demo: spinner, rocket, save, download, merge, rabbit, fireworks, baby, confetti, trophy, all
        #[arg(default_value = "all")]
        animation: String,

        /// Use fullscreen mode instead of inline (clears terminal)
        #[arg(short, long)]
        fullscreen: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Git { args } => {
            let mut wrapper = GitWrapper::new()?;
            let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            let result = wrapper.run(&args_refs)?;

            // Exit with the same code as git
            std::process::exit(result.exit_code);
        }

        Commands::Demo { animation, fullscreen } => {
            // Use inline mode by default for non-disruptive demos
            // Only use fullscreen if explicitly requested
            let mut player = if fullscreen {
                AnimationPlayer::new()? // Fullscreen mode
            } else {
                println!("💡 Tip: Add --fullscreen flag for immersive fullscreen mode\n");
                AnimationPlayer::inline_auto()? // Inline mode with 1/3 terminal height
            };

            match animation.as_str() {
                "spinner" => {
                    println!("🌀 Spinner Animation Demo");
                    player.play_for(SpinnerAnimation::new(), Duration::from_secs(3))?;
                }
                "rocket" => {
                    println!("🚀 Rocket Animation Demo");
                    player.play(RocketAnimation::new(Duration::from_secs(2)))?;
                }
                "save" => {
                    println!("💾 Save Animation Demo");
                    player.play(SaveAnimation::default())?;
                }
                "download" => {
                    println!("⬇️  Download Animation Demo");
                    player.play(DownloadAnimation::default())?;
                }
                "merge" => {
                    println!("🔀 Merge Animation Demo");
                    player.play(MergeAnimation::default())?;
                }
                "rabbit" => {
                    println!("🐰 White Rabbit Animation Demo - I'm late! I'm late!");
                    player.play(RabbitAnimation::default())?;
                }
                "fireworks" => {
                    println!("🎆 Fireworks Animation Demo");
                    player.play(FireworksAnimation::default())?;
                }
                "baby" => {
                    println!("👶 Baby Announcement Demo - Congratulations, you're the father!");
                    player.play(BabyAnnouncementAnimation::default())?;
                }
                "confetti" => {
                    println!("🎊 Confetti Animation Demo");
                    player.play(ConfettiAnimation::default())?;
                }
                "trophy" => {
                    println!("🏆 Trophy Animation Demo - You're a champion!");
                    player.play(TrophyAnimation::default())?;
                }
                "all" | _ => {
                    println!("🎮 Running all animations...\n");

                    println!("1. Spinner Animation");
                    player.play_for(SpinnerAnimation::new(), Duration::from_secs(2))?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n2. Save Animation");
                    player.play(SaveAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n3. Rocket Animation");
                    player.play(RocketAnimation::new(Duration::from_secs(2)))?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n4. Download Animation");
                    player.play(DownloadAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n5. Merge Animation");
                    player.play(MergeAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n6. White Rabbit - I'm late!");
                    player.play(RabbitAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n7. Fireworks Celebration!");
                    player.play(FireworksAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n8. Baby Announcement - It's a Commit!");
                    player.play(BabyAnnouncementAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n9. Confetti Party!");
                    player.play(ConfettiAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n10. Trophy - You're a Winner!");
                    player.play(TrophyAnimation::default())?;

                    println!("\n✨ Demo complete! What a show!");
                }
            }
        }
    }

    Ok(())
}
