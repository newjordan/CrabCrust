// CrabCrust: Add arcade-style animations to your CLI tools 🦀✨

use anyhow::Result;
use clap::{Parser, Subcommand};
use crabcrust::wrapper::git::GitWrapper;
use crabcrust::{
    AnimationPlayer, BabyAnnouncementAnimation, ConfettiAnimation, DownloadAnimation,
    FireworksAnimation, MergeAnimation, RabbitAnimation, RocketAnimation, SaveAnimation,
    SpinnerAnimation, TrophyAnimation, JackpotAnimation, MultiballAnimation, ExtraBallAnimation,
    BonusMultiplierAnimation,
};

#[cfg(feature = "video")]
use crabcrust::FrameBasedAnimation;
use std::time::Duration;

#[cfg(feature = "video")]
use std::path::PathBuf;

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
        /// Which animation to demo: spinner, rocket, save, download, merge, rabbit, fireworks, baby, confetti, trophy, jackpot, multiball, extraball, bonus, pinball, all
        #[arg(default_value = "all")]
        animation: String,

        /// Use fullscreen mode instead of inline (clears terminal)
        #[arg(short, long)]
        fullscreen: bool,
    },

    /// Convert video/GIF to Braille animation (requires 'video' feature)
    #[cfg(feature = "video")]
    Convert {
        /// Input file (video or GIF)
        input: PathBuf,

        /// Output width in terminal cells (default: 64 for DMD 128x32)
        #[arg(short, long, default_value = "64")]
        width: usize,

        /// Output height in terminal cells (default: 8 for DMD 128x32)
        #[arg(short = 'H', long, default_value = "8")]
        height: usize,

        /// Brightness threshold (0-255, default: 128)
        #[arg(short, long, default_value = "128")]
        threshold: u8,

        /// Play the animation after conversion
        #[arg(short, long)]
        play: bool,

        /// Loop the animation when playing
        #[arg(short, long)]
        loop_play: bool,

        /// Maximum frames to convert (useful for long videos)
        #[arg(short, long)]
        max_frames: Option<usize>,
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
                "jackpot" => {
                    println!("🎰 Jackpot Animation Demo - You hit the jackpot!");
                    player.play(JackpotAnimation::default())?;
                }
                "multiball" => {
                    println!("⚪ Multiball Animation Demo - MULTIBALL!");
                    player.play(MultiballAnimation::default())?;
                }
                "extraball" => {
                    println!("✨ Extra Ball Animation Demo - FREE BALL!");
                    player.play(ExtraBallAnimation::default())?;
                }
                "bonus" => {
                    println!("🔢 Bonus Multiplier Animation Demo - 2X... 3X... 4X...");
                    player.play(BonusMultiplierAnimation::default())?;
                }
                "pinball" => {
                    println!("🎰 Running all pinball animations...\n");

                    println!("1. JACKPOT!");
                    player.play(JackpotAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n2. MULTIBALL!");
                    player.play(MultiballAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n3. EXTRA BALL!");
                    player.play(ExtraBallAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n4. BONUS MULTIPLIER!");
                    player.play(BonusMultiplierAnimation::default())?;

                    println!("\n🎰 Pinball demos complete!");
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
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n11. Jackpot - JACKPOT!");
                    player.play(JackpotAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n12. Multiball - MULTIBALL!");
                    player.play(MultiballAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n13. Extra Ball - FREE BALL!");
                    player.play(ExtraBallAnimation::default())?;
                    std::thread::sleep(Duration::from_millis(500));

                    println!("\n14. Bonus Multiplier - 2X... 3X... 4X!");
                    player.play(BonusMultiplierAnimation::default())?;

                    println!("\n🎰✨ Demo complete! What a show!");
                }
            }
        }

        #[cfg(feature = "video")]
        Commands::Convert {
            input,
            width,
            height,
            threshold,
            play,
            loop_play,
            max_frames,
        } => {
            use crabcrust::video::converter;

            println!("🎬 Converting {} to Braille animation...", input.display());
            println!("   Target size: {}x{} cells ({}x{} dots)", width, height, width * 2, height * 4);
            println!("   Threshold: {}", threshold);

            // Detect file type and convert
            let frames = if input.extension().and_then(|s| s.to_str()) == Some("gif") {
                println!("   Detected: Animated GIF");
                converter::gif_to_frames(&input, width, height, threshold)?
            } else {
                println!("   Detected: Video file (using ffmpeg)");
                converter::video_to_frames(&input, width, height, threshold, max_frames)?
            };

            println!("✅ Converted {} frames!", frames.len());

            if play {
                println!("\n▶️  Playing animation...");
                let animation = FrameBasedAnimation::from_braille_frames(frames, loop_play);

                let mut player = AnimationPlayer::inline_auto()?;
                player.play(animation)?;

                println!("\n✨ Playback complete!");
            } else {
                println!("\n💡 Tip: Add --play to preview the animation");
                println!("   You can use these frames in your own code with FrameBasedAnimation");
            }
        }
    }

    Ok(())
}
