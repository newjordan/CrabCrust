//! Print text captures of every built-in animation.
//!
//! No terminal takeover: frames are braille characters on stdout.
//!
//! ```text
//! cargo run --example text_frames
//! ```

use std::time::Duration;

use crabcrust::{
    render_frame, Animation, BabyAnnouncementAnimation, ConfettiAnimation, DownloadAnimation,
    FireworksAnimation, MatrixRainAnimation, MergeAnimation, RabbitAnimation, RocketAnimation,
    SaveAnimation, SpinnerAnimation, TrophyAnimation,
};

fn show(title: &str, animation: &impl Animation, width: usize, height: usize) {
    let grid = render_frame(animation, width, height);
    println!("{title}");
    println!("{}", grid.to_trimmed_text());
}

fn at(title: &str, animation: &mut impl Animation, elapsed_ms: u64, width: usize, height: usize) {
    // Step in small slices so threshold-based motion (fireworks) matches playback.
    let mut left = elapsed_ms;
    while left > 0 {
        let step = left.min(50);
        animation.update(Duration::from_millis(step));
        left -= step;
    }
    show(title, animation, width, height);
}

fn main() {
    let (width, height) = (72, 24);
    at(
        "spinner @ 200ms",
        &mut SpinnerAnimation::new(),
        200,
        width,
        height,
    );
    at(
        "rocket @ 900ms",
        &mut RocketAnimation::new(Duration::from_secs(2)),
        900,
        width,
        height,
    );
    at(
        "save @ 1200ms",
        &mut SaveAnimation::new(Duration::from_millis(1500)),
        1200,
        width,
        height,
    );
    at(
        "download @ 800ms",
        &mut DownloadAnimation::new(Duration::from_millis(1500)),
        800,
        width,
        height,
    );
    at(
        "merge @ 400ms",
        &mut MergeAnimation::new(Duration::from_millis(1500)),
        400,
        width,
        height,
    );
    at(
        "rabbit @ 900ms",
        &mut RabbitAnimation::new(Duration::from_millis(2000)),
        900,
        width,
        height,
    );
    at(
        "fireworks @ 1800ms",
        &mut FireworksAnimation::new(Duration::from_millis(3000)),
        1800,
        100,
        56,
    );
    at(
        "baby @ 1600ms",
        &mut BabyAnnouncementAnimation::new(Duration::from_millis(2500)),
        1600,
        width,
        height,
    );
    at(
        "confetti @ 700ms",
        &mut ConfettiAnimation::new(Duration::from_millis(2000)),
        700,
        width,
        height,
    );
    at(
        "trophy @ 1600ms",
        &mut TrophyAnimation::new(Duration::from_millis(2000)),
        1600,
        width,
        height,
    );
    at(
        "matrix @ 500ms",
        &mut MatrixRainAnimation::new("git status\nclean".to_string()),
        500,
        width,
        height,
    );
}
