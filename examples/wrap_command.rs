//! Wrap a real command and draw the result frame as text.
//!
//! This is the headless shape of a CrabCrust wrapper: run the process with
//! [`CommandExecutor`](crabcrust::CommandExecutor), keep its exit code, and
//! render the animation that matches the outcome. [`CliWrapper`] and
//! [`GitWrapper`] do the same thing on a live terminal.
//!
//! ```text
//! cargo run --example wrap_command
//! ```

use std::time::Duration;

use crabcrust::{render_frame, Animation, CommandExecutor, SaveAnimation, SpinnerAnimation};

fn trimmed(animation: &impl Animation) -> String {
    render_frame(animation, 64, 20).to_trimmed_text()
}

fn main() {
    let executor = CommandExecutor::new("echo", &["wrapped by crabcrust"]);
    println!("$ {}", executor.command_string());

    let mut waiting = SpinnerAnimation::new();
    waiting.update(Duration::from_millis(180));
    println!("\nwhile it runs\n{}", trimmed(&waiting));

    let result = executor.run().expect("failed to run echo");

    if result.success {
        let mut saved = SaveAnimation::new(Duration::from_millis(1500));
        // Land in the checkmark phase of the 1.5s save animation.
        saved.update(Duration::from_millis(1200));
        println!("on success\n{}", trimmed(&saved));
    } else {
        println!("command failed with status {}", result.exit_code);
    }

    print!("{}", result.stdout);
    std::process::exit(result.exit_code);
}
