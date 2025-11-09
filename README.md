# CrabCrust 🦀✨

**Add arcade-style animations to your CLI tools!**

Transform boring command-line interfaces into engaging, arcade-like experiences with stunning Braille-based terminal animations. Every git commit becomes a save animation, every push launches a rocket, and every command feels like a celebration!

## 🎬 See It In Action

https://github.com/newjordan/CrabCrust/assets/crabcrust_in_action.mp4

*CrabCrust adding delightful animations to everyday git commands*

## ✨ Features

- **High-Resolution Braille Graphics**: Uses Unicode Braille characters (⣿) for 8× terminal resolution (2×4 dots per cell)
- **Procedural Animations**: Hand-crafted animations including spinners, rockets, save disks, and more
- **Git Integration**: Themed animations for git commands (commit, push, pull, etc.)
- **Inline Rendering**: Animations display in 1/3 of terminal height - non-disruptive to your workflow
- **Terminal-Native**: Works in any modern terminal with Unicode support
- **Fast & Lightweight**: Written in Rust for blazing-fast performance
- **Experimental**: GIF/video conversion pipeline *(work in progress)*

## 🎮 Demo

```bash
# Test all animations
crabcrust demo all

# Test individual animations
crabcrust demo spinner
crabcrust demo rocket
crabcrust demo save
```

## 🚀 Installation

```bash
git clone https://github.com/newjordan/CrabCrust.git
cd CrabCrust
cargo build --release

# The binary will be at target/release/crabcrust
# Optionally install to your system:
cargo install --path .
```

## 📖 Usage

### Git Wrapper

The most common use case is wrapping git commands:

```bash
# Use crabcrust to run git commands with animations
crabcrust git commit -m "Add new feature"  # Shows save animation
crabcrust git push                          # Shows rocket launch animation
crabcrust git pull                          # Shows loading spinner
crabcrust git status                        # Shows quick spinner
```

### Shell Alias

For the ultimate experience, add these to your `.bashrc` or `.zshrc`:

```bash
alias git="crabcrust git"
```

Now every git command automatically gets animated!

```bash
git commit -m "This will show a floppy disk save animation! 💾"
git push  # 🚀 Rocket launch!
```

## 🎨 Animations

### Spinner Animation
A smooth rotating circle with a trailing effect - perfect for loading states.

**Used for**: Generic commands, status checks, pull operations

### Rocket Animation
A rocket ship launching upward with flame effects and stars - celebrating your code going live!

**Used for**: `git push`

**Duration**: 2 seconds

**Features**:
- Procedurally generated stars
- Animated flame with flickering effect
- Smooth easing animation

### Save Animation
A floppy disk icon with progress bar and checkmark - the classic save icon!

**Used for**: `git commit`

**Duration**: 1.5 seconds

**Phases**:
1. Disk appears
2. Progress bar fills
3. Checkmark appears
4. Success state

## 🏗️ Architecture

CrabCrust is built on a modular architecture:

```
crabcrust/
├── braille/          # High-res Braille grid rendering
├── rendering/        # Terminal management (Ratatui + Crossterm)
├── animation/        # Animation engine & procedural animations
│   ├── spinner.rs    # Rotating spinner
│   ├── rocket.rs     # Rocket launch
│   └── save.rs       # Floppy disk save
├── executor/         # Command execution & output capture
└── wrapper/          # CLI wrappers (git, cargo, etc.)
```

### Key Components

#### BrailleGrid
High-resolution terminal graphics using Unicode Braille patterns:
- Each terminal cell = 2×4 dots (8 possible dots)
- 256 unique patterns per cell (U+2800 to U+28FF)
- Full RGB color support per cell
- Bresenham's line algorithm for smooth curves
- Circle drawing with midpoint algorithm

#### Animation Trait
Simple trait for creating custom animations:
```rust
pub trait Animation {
    fn update(&mut self, delta_time: Duration) -> bool;
    fn render(&self, grid: &mut BrailleGrid);
    fn name(&self) -> &str;
}
```

#### Command Executor
Spawns subprocesses, captures output, and preserves exit codes:
```rust
let executor = CommandExecutor::new("git", &["status"]);
let result = executor.run()?;
assert_eq!(result.exit_code, 0);
```

## 🔧 Creating Custom Animations

Want to create your own animation? It's easy!

```rust
use crabcrust::{Animation, BrailleGrid, Color};
use std::time::Duration;

struct MyAnimation {
    elapsed: Duration,
}

impl Animation for MyAnimation {
    fn update(&mut self, delta_time: Duration) -> bool {
        self.elapsed += delta_time;
        self.elapsed < Duration::from_secs(2)  // Run for 2 seconds
    }

    fn render(&self, grid: &mut BrailleGrid) {
        let center_x = grid.dot_width() / 2;
        let center_y = grid.dot_height() / 2;

        // Draw something cool!
        grid.draw_circle(center_x, center_y, 20, Color::CYAN);
    }

    fn name(&self) -> &str {
        "MyAnimation"
    }
}
```

## 🧪 Experimental Features

> **Note**: These features are currently in development and may not be fully functional.

### GIF/Video Conversion *(Beta)*

Convert GIFs and videos to Braille animations:

```bash
# Build with experimental features
cargo build --release --features gif

# Convert a GIF
./target/release/crabcrust convert animation.gif --play
```

### DMD Animation Library

Browse and preview pinball Dot Matrix Display (DMD) animations:

```bash
# List all available DMD animations
crabcrust library list

# Filter by tag or theme
crabcrust library list --tag tech
crabcrust library list --theme "Monster Bash"

# Preview an animation
crabcrust library preview skull

# Show detailed info
crabcrust library info invader

# Browse tags and themes
crabcrust library tags
crabcrust library themes
```

The DMD library includes curated pinball animations that automatically play for git commands. See [docs/DMD_LIBRARY.md](docs/DMD_LIBRARY.md) for details on adding new DMDs.

See [docs/](docs/) for more details on experimental features.

## 🎯 Roadmap

- [x] Core Braille rendering engine
- [x] Git command wrapper with procedural animations
- [x] Inline rendering mode (1/3 terminal height)
- [ ] GIF/video conversion pipeline *(in progress)*
- [ ] More procedural animations (download, merge, error states)
- [ ] Cargo wrapper (`crabcrust cargo build`)
- [ ] Configuration file for custom animation mappings
- [ ] Plugin system for community animations

## 🤝 Contributing

Contributions welcome! Here's how you can help:

1. **Create new animations**: Add more procedural animations in `src/animation/`
2. **Add CLI wrappers**: Support more tools (cargo, npm, docker, etc.)
3. **Improve rendering**: Optimize BrailleGrid performance
4. **Fix bugs**: Check the issues page
5. **Add tests**: Expand test coverage

## 📝 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Credits

The BrailleGrid rendering system was originally developed for real-time audio visualization and has been adapted for general-purpose terminal animations.

## 🎪 Philosophy

Command-line tools don't have to be boring! We spend hours every day in the terminal - why not make it delightful?

CrabCrust believes that:
- **Feedback should be engaging**: Visual feedback makes commands more satisfying
- **CLI can be beautiful**: Terminal graphics can be stunning with the right techniques
- **Celebration matters**: Every `git push` is an achievement worth celebrating

Made with 🦀 and ✨ by the Rust community.

---

**Star this repo if you love making terminals fun!** ⭐
