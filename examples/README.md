# CrabCrust Examples

This directory contains example code and demo materials for CrabCrust.

## Code Examples

- **`text_frames.rs`** - Print a text capture of every built-in animation (no tty)
- **`wrap_command.rs`** - Run a command with `CommandExecutor` and draw the result frame as text
- **`custom_animation.rs`** - Create your own custom Braille animation
- **`git_wrapper.rs`** - Use the GitWrapper to add animations to git commands
- **`test_braille_no_terminal.rs`** - Test Braille rendering without terminal dependency
- **`visualize_animations.rs`** - Print several frames of the spinner, rocket, and save animations
- **`pac_inspector.rs`** - Inspect Pin2DMD .pac files *(experimental, needs `--features video`)*

## Running Examples

```bash
# Headless text captures (no alternate screen)
cargo run --example text_frames

# Wrap a command and print the result frame
cargo run --example wrap_command

# Play a custom animation in the terminal
cargo run --example custom_animation

# Inspect a Pin2DMD package
cargo run --features video --example pac_inspector -- path/to/file.pac
```

## Demo Materials

**demo.gif / demo.mp4** - Demo video showing CrabCrust in action

Place your demo video here with the name `demo.gif` or `demo.mp4` and it will automatically appear in the main README.

## Creating Custom Animations

See `custom_animation.rs` for a template showing how to:
1. Implement the `Animation` trait
2. Render to a BrailleGrid
3. Update animation state
4. Use with AnimationPlayer

## Contributing Examples

Have a cool example? Submit a PR!
