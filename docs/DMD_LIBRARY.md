# DMD Animation Library

The DMD (Dot Matrix Display) Library is a curated collection of pinball DMD animations that can be used with CrabCrust's git wrapper and other commands.

## Features

- **Browse** all available DMD animations with metadata
- **Filter** by tags (action, celebration, tech, etc.) or themes
- **Preview** animations before using them
- **Automatic** mapping to git commands

## Usage

### List all DMDs

```bash
crabcrust library list
```

### Filter by tag

```bash
crabcrust library list --tag celebration
crabcrust library list --tag tech
```

### Filter by theme

```bash
crabcrust library list --theme "Monster Bash"
```

### Preview an animation

```bash
crabcrust library preview skull
crabcrust library preview invader --loop  # Loop continuously
```

### Show detailed info

```bash
crabcrust library info skull
```

### Show all tags

```bash
crabcrust library tags
```

### Show all themes

```bash
crabcrust library themes
```

## Available Animations

Currently included DMDs:

| Name    | Theme        | Tags                        | Frames | Description                                      |
|---------|--------------|----------------------------|--------|--------------------------------------------------|
| invader | Monster Bash | action, horror, monster    | 38     | Quick & energetic invader character             |
| skull   | Monster Bash | action, horror, monster    | 63     | Action sequence with skull character            |
| sword   | Monster Bash | celebration, victory, monster | 42  | Victory pose with sword                         |

## Adding New DMDs

### 1. Find or Create DMD GIFs

Good sources for pinball DMD GIFs:
- **Tenor**: Search for "pinball DMD" + machine name
- **Giphy**: Search for specific pinball machines
- **Virtual Pinball Forums**: VPUniverse, VPForums (may require registration)
- **Record Your Own**: Use screen capture from virtual pinball games

Ideal specifications:
- **Resolution**: 128x32 pixels (standard DMD) or ~498x144 (Tenor GIFs work well)
- **Format**: Animated GIF
- **Colors**: Monochrome or single-color (orange, green, etc.)
- **Frame rate**: 15-30 FPS

### 2. Add GIF to Repository

Place the GIF file in the `ref/` directory:

```bash
# Example
cp ~/Downloads/t2_skull.gif ref/dmd_t2_skull.gif
```

### 3. Update dmd_library.rs

Edit `src/dmd_library.rs`:

**Step 3.1**: Add enum variant

```rust
pub enum DmdAnimation {
    Invader,
    Skull,
    Sword,
    T2Skull,  // <-- Add new variant
}
```

**Step 3.2**: Add info() match arm

```rust
DmdAnimation::T2Skull => DmdInfo {
    name: "t2-skull",
    file_path: "ref/dmd_t2_skull.gif",
    description: "Terminator 2 skull (56 frames, tech theme)",
    frames: 56,
    tags: &["action", "tech", "sci-fi"],
    theme: "Terminator 2",
},
```

**Step 3.3**: Add to all() vector

```rust
pub fn all() -> Vec<DmdAnimation> {
    vec![
        DmdAnimation::Invader,
        DmdAnimation::Skull,
        DmdAnimation::Sword,
        DmdAnimation::T2Skull,  // <-- Add here
    ]
}
```

**Step 3.4** (Optional): Map to git commands

```rust
pub fn git_command_to_dmd(command: &str) -> Option<DmdAnimation> {
    match command {
        "status" => Some(DmdAnimation::Invader),
        "push" => Some(DmdAnimation::T2Skull),  // <-- Map commands
        // ...
        _ => None,
    }
}
```

### 4. Test Your DMD

```bash
# Build with gif support
cargo build --features gif

# List to verify it appears
cargo run --features gif -- library list

# Preview it
cargo run --features gif -- library preview t2-skull
```

## Tag Guidelines

Use consistent, descriptive tags:

**Action/Mood**:
- `action` - Fast-paced, energetic animations
- `celebration` - Victory, success animations
- `suspense` - Tension, anticipation
- `humor` - Funny, lighthearted

**Genre/Theme**:
- `horror` - Monsters, scary themes
- `sci-fi` - Technology, futuristic
- `tech` - Computers, robots, cyborg
- `fantasy` - Medieval, magic
- `adventure` - Exploration, quests

**Character Type**:
- `monster` - Creatures, monsters
- `robot` - Mechanical beings
- `alien` - Extraterrestrial
- `hero` - Protagonists, champions

**Visual Style**:
- `retro` - Classic pinball era
- `modern` - Contemporary designs
- `colorful` - Vibrant, multi-color (if converted to color)

## Theme Guidelines

Themes should match the pinball machine name:

- "Monster Bash"
- "Terminator 2"
- "Attack from Mars"
- "Medieval Madness"
- "Star Trek: TNG"
- "The Addams Family"
- etc.

## Finding Cool DMDs

### Recommended Pinball Machines with Great DMDs

**Sci-Fi/Tech Theme** (perfect for terminal use):
- **Terminator 2** - Iconic tech/cyborg theme, "Hasta la vista, baby"
- **Star Trek: The Next Generation** - LCARS interface animations
- **Attack from Mars** - Alien invasion, flying saucers
- **Demolition Man** - Futuristic action

**Classic Action**:
- **Medieval Madness** - Castle battles, trolls
- **Monster Bash** - Classic monsters (already included)
- **The Addams Family** - Thing, mansion animations
- **Twilight Zone** - Surreal, mysterious

**Modern/Recent**:
- **Iron Maiden** - Heavy metal artwork
- **Avengers** - Superhero action
- **Jurassic Park** - Dinosaurs, adventure

## Conversion Tips

If you have video files instead of GIFs:

```bash
# Convert video to GIF using crabcrust
crabcrust convert path/to/dmd_video.mp4 \
  --width 124 \
  --height 19 \
  --threshold 50 \
  --play

# If you like it, the frames are in memory
# For best results, use external tools like ffmpeg to create the GIF first
```

## Examples

Preview a celebration animation when you need motivation:

```bash
crabcrust library preview sword
```

See all tech-themed DMDs:

```bash
crabcrust library list --tag tech
```

Browse what's available:

```bash
crabcrust library list
crabcrust library tags
```

## Git Integration

DMDs are automatically shown for git commands when using the git wrapper:

```bash
# Shows an appropriate DMD animation based on command
crabcrust git push    # Shows "sword" (victory)
crabcrust git status  # Shows "invader" (quick check)
crabcrust git pull    # Shows "skull" (pulling changes)
```

See `src/dmd_library.rs` function `git_command_to_dmd()` for current mappings.

## Contributing

Want to add DMDs to the library?

1. Find high-quality DMD GIFs (see "Finding Cool DMDs" above)
2. Follow the "Adding New DMDs" workflow
3. Test thoroughly
4. Consider submitting a PR with your additions!

The goal is to build a comprehensive library of pinball DMD animations that make terminal commands more fun and visually interesting.
