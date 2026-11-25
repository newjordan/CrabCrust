//! Snapshot the braille frames animations actually draw.
//!
//! Each snapshot is the trimmed frame plus a color digest, so a shape change
//! or a recolor fails the test. Grids are fixed at 80×30 cells (160×120 dots),
//! which is large enough for the procedural coordinates these animations use.

use std::time::Duration;

use crabcrust::{
    render_frame, Animation, BabyAnnouncementAnimation, BrailleGrid, ConfettiAnimation,
    DownloadAnimation, FireworksAnimation, FrameBasedAnimation, MatrixRainAnimation,
    MergeAnimation, RabbitAnimation, RocketAnimation, SaveAnimation, SpinnerAnimation,
    TrophyAnimation,
};
use pretty_assertions::assert_eq;

const WIDTH: usize = 80;
const HEIGHT: usize = 30;

fn color_digest(grid: &BrailleGrid) -> u32 {
    let mut acc = 2_166_136_261u32;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if let Some(color) = grid.get_color(x, y) {
                acc = acc.wrapping_mul(16_777_619)
                    ^ (u32::from(color.r) << 16)
                    ^ (u32::from(color.g) << 8)
                    ^ u32::from(color.b);
                acc ^= (x as u32).wrapping_mul(131) ^ (y as u32).wrapping_mul(17);
            }
        }
    }
    acc
}

fn frame_text(animation: &impl Animation) -> String {
    let grid = render_frame(animation, WIDTH, HEIGHT);
    let art = grid.to_trimmed_text();
    let body = if art.is_empty() {
        "(blank)\n".to_string()
    } else {
        art
    };
    format!("digest {:08x}\n{body}", color_digest(&grid))
}

fn rollout(mut animation: impl Animation, steps_ms: &[u64]) -> String {
    let mut out = String::new();
    let mut elapsed = 0u64;
    out.push_str(&format!("--- {elapsed}ms ---\n"));
    out.push_str(&frame_text(&animation));

    for step in steps_ms {
        let keep_playing = animation.update(Duration::from_millis(*step));
        elapsed += step;
        out.push_str(&format!("--- {elapsed}ms keep={keep_playing} ---\n"));
        out.push_str(&frame_text(&animation));
    }
    out
}

#[test]
fn animation_names_stay_stable() {
    assert_eq!(SpinnerAnimation::new().name(), "Spinner");
    assert_eq!(RocketAnimation::default().name(), "Rocket");
    assert_eq!(SaveAnimation::default().name(), "Save");
    assert_eq!(DownloadAnimation::default().name(), "Download");
    assert_eq!(MergeAnimation::default().name(), "Merge");
    assert_eq!(RabbitAnimation::default().name(), "WhiteRabbit");
    assert_eq!(FireworksAnimation::default().name(), "Fireworks");
    assert_eq!(
        BabyAnnouncementAnimation::default().name(),
        "BabyAnnouncement"
    );
    assert_eq!(ConfettiAnimation::default().name(), "Confetti");
    assert_eq!(TrophyAnimation::default().name(), "Trophy");
    assert_eq!(
        MatrixRainAnimation::new("ok".to_string()).name(),
        "Matrix Rain"
    );
    assert_eq!(
        FrameBasedAnimation::default().name(),
        "Frame-Based Animation"
    );
}

#[test]
fn finite_animations_stop_and_spinner_loops() {
    let mut spinner = SpinnerAnimation::new();
    assert!(spinner.duration().is_none());
    assert!(spinner.update(Duration::from_secs(30)));

    let mut save = SaveAnimation::new(Duration::from_millis(1500));
    assert_eq!(save.duration(), Some(Duration::from_millis(1500)));
    assert!(save.update(Duration::from_millis(1499)));
    assert!(!save.update(Duration::from_millis(1)));

    let mut rocket = RocketAnimation::new(Duration::from_secs(2));
    assert!(!rocket.update(Duration::from_secs(2)));
}

#[test]
fn render_frame_uses_the_requested_grid() {
    let grid = render_frame(&SpinnerAnimation::new(), 12, 4);
    assert_eq!(grid.width(), 12);
    assert_eq!(grid.height(), 4);
    assert_eq!(grid.dot_width(), 24);
    assert!(!grid.to_trimmed_text().is_empty());
}

#[test]
fn frame_based_playback_advances_patterns() {
    let mut animation = FrameBasedAnimation::new(false);
    animation.add_frame(
        vec![0x01, 0x00, 0x00, 0x00],
        2,
        2,
        Duration::from_millis(100),
    );
    animation.add_frame(
        vec![0xFF, 0xFF, 0xFF, 0xFF],
        2,
        2,
        Duration::from_millis(100),
    );
    assert_eq!(animation.frame_count(), 2);
    assert_eq!(animation.duration(), Some(Duration::from_millis(200)));

    let first = render_frame(&animation, 4, 3).to_trimmed_text();
    assert!(animation.update(Duration::from_millis(100)));
    assert_eq!(animation.current_frame(), 1);
    let second = render_frame(&animation, 4, 3).to_trimmed_text();
    assert_ne!(first, second);
    assert!(!animation.update(Duration::from_millis(100)));

    let mut looping = FrameBasedAnimation::new(true);
    looping.add_frame(vec![0x01], 1, 1, Duration::from_millis(10));
    assert!(looping.duration().is_none());
    assert!(looping.update(Duration::from_millis(10)));
    assert_eq!(looping.current_frame(), 0);
}

#[test]
fn spinner_frames() {
    insta::assert_snapshot!(rollout(SpinnerAnimation::new(), &[150, 300]));
}

#[test]
fn rocket_frames() {
    insta::assert_snapshot!(rollout(
        RocketAnimation::new(Duration::from_secs(2)),
        &[500, 700, 700]
    ));
}

#[test]
fn save_frames() {
    insta::assert_snapshot!(rollout(
        SaveAnimation::new(Duration::from_millis(1500)),
        &[200, 500, 500, 250]
    ));
}

#[test]
fn download_frames() {
    insta::assert_snapshot!(rollout(
        DownloadAnimation::new(Duration::from_millis(1500)),
        &[200, 600, 600]
    ));
}

#[test]
fn merge_frames() {
    insta::assert_snapshot!(rollout(
        MergeAnimation::new(Duration::from_millis(1500)),
        &[200, 550, 550]
    ));
}

#[test]
fn rabbit_frames() {
    insta::assert_snapshot!(rollout(
        RabbitAnimation::new(Duration::from_millis(2000)),
        &[200, 700, 800]
    ));
}

#[test]
fn fireworks_frames() {
    insta::assert_snapshot!(rollout(
        FireworksAnimation::new(Duration::from_millis(3000)),
        &[400, 700, 700]
    ));
}

#[test]
fn baby_frames() {
    insta::assert_snapshot!(rollout(
        BabyAnnouncementAnimation::new(Duration::from_millis(2500)),
        &[300, 700, 1000]
    ));
}

#[test]
fn confetti_frames() {
    insta::assert_snapshot!(rollout(
        ConfettiAnimation::new(Duration::from_millis(2000)),
        &[200, 500, 700]
    ));
}

#[test]
fn trophy_frames() {
    insta::assert_snapshot!(rollout(
        TrophyAnimation::new(Duration::from_millis(2000)),
        &[300, 700, 800]
    ));
}

#[test]
fn matrix_frames() {
    insta::assert_snapshot!(rollout(
        MatrixRainAnimation::new("git status\nclean".to_string()),
        &[80, 200, 400]
    ));
}

#[test]
fn manual_frame_playback() {
    let mut animation = FrameBasedAnimation::new(false);
    animation.add_frame(
        vec![0x01, 0x08, 0x00, 0x80],
        2,
        2,
        Duration::from_millis(50),
    );
    animation.add_frame(vec![0xFF; 4], 2, 2, Duration::from_millis(50));
    insta::assert_snapshot!(rollout(animation, &[50, 50]));
}
