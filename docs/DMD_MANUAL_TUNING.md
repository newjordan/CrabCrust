# Manual DMD Tuning Guide

This guide shows how to manually test and tune DMD GIF conversions to get the best quality.

## The Problem

Not all DMD GIFs convert well automatically. Issues:
- **Too dark/faint**: Threshold too high, pixels filtered out
- **Too bright/bloated**: Threshold too low, noise shows up
- **Pixelated**: Size too small, detail lost
- **Wrong colors**: GIF not optimized for terminal Braille

## Quick Testing Tool

Use the quality tester to find the right settings:

```bash
python tools/test_dmd_quality.py ref/your_dmd.gif
```

This will test 7 different combinations and let you see which looks best!

## Manual Testing

Test conversions manually:

```bash
# Try different thresholds (0-255)
crabcrust convert your.gif --threshold 50 --play   # Low (more visible)
crabcrust convert your.gif --threshold 80 --play   # Medium
crabcrust convert your.gif --threshold 128 --play  # High (less visible)

# Try different sizes (width x height in cells)
crabcrust convert your.gif --width 64 --height 8 --play    # Small (128x32 dots)
crabcrust convert your.gif --width 100 --height 15 --play  # Medium (200x60 dots)
crabcrust convert your.gif --width 124 --height 19 --play  # Large (248x76 dots)

# Combine both
crabcrust convert your.gif --width 124 --height 19 --threshold 50 --play
```

## Understanding the Parameters

### Threshold (0-255)

Controls which pixels become dots:
- **Low (30-60)**: More pixels visible, good for dark/orange DMDs
- **Medium (70-100)**: Balanced, works for most
- **High (110-150)**: Only bright pixels, cleaner but may lose detail

**Rule of thumb:**
- If GIF looks **mostly black** → LOWER threshold
- If GIF looks **too bright/noisy** → RAISE threshold

### Size (width × height cells)

Controls detail level:
- **64×8 cells** (128×32 dots): Standard DMD size, compact
- **100×15 cells** (200×60 dots): Medium, good balance
- **124×19 cells** (248×76 dots): Large, preserves detail

**Rule of thumb:**
- If GIF looks **pixelated/blocky** → INCREASE size
- If GIF looks **too detailed/busy** → DECREASE size

### Source GIF Quality

Best source GIFs:
- ✅ **128×32 pixels** (native DMD resolution)
- ✅ **High contrast** (bright on black)
- ✅ **Monochrome or 2-color** (orange/white on black)
- ✅ **Clean pixels** (no anti-aliasing artifacts)

Problematic GIFs:
- ❌ **Large upscaled** (498×150 like Tenor GIFs)
- ❌ **Low contrast** (gray on gray)
- ❌ **Multi-color gradients** (lose detail in grayscale)
- ❌ **Heavy compression** (JPEG artifacts)

## Current DMD Settings

Our Tenor GIFs (498×150) currently use:
```rust
width: 124 cells
height: 19 cells
threshold: 50
```

This gives us **248×76 dots** with **low threshold** for better visibility.

## Adding a New DMD to the Library

Once you find settings that work:

### 1. Test the GIF
```bash
crabcrust convert my_new_dmd.gif --width 124 --height 19 --threshold 50 --play
```

### 2. Add to `ref/` folder
```bash
cp my_new_dmd.gif ref/dmd_my_animation.gif
```

### 3. Update `src/dmd_library.rs`

Add to the `DmdAnimation` enum:
```rust
pub enum DmdAnimation {
    Invader,
    Skull,
    Sword,
    MyAnimation,  // Add your new one
}
```

Add to the `info()` method:
```rust
DmdAnimation::MyAnimation => DmdInfo {
    name: "myanimation",
    file_path: "ref/dmd_my_animation.gif",
    description: "Description here (X frames, purpose)",
    frames: 42,  // Count from conversion output
},
```

Add to git command mapping:
```rust
pub fn git_command_to_dmd(command: &str) -> Option<DmdAnimation> {
    match command {
        "status" | "diff" | "log" => Some(DmdAnimation::Invader),
        "pull" | "fetch" | "clone" => Some(DmdAnimation::Skull),
        "push" | "merge" => Some(DmdAnimation::Sword),
        "rebase" => Some(DmdAnimation::MyAnimation),  // Add your mapping
        _ => None,
    }
}
```

### 4. Rebuild and test
```bash
cargo build --release --features gif
crabcrust git rebase  # Test your new command
```

## Per-DMD Custom Settings (Advanced)

If different DMDs need different settings, update `load_dmd_animation()`:

```rust
pub fn load_dmd_animation(dmd: DmdAnimation, loop_animation: bool) -> Result<FrameBasedAnimation> {
    let info = dmd.info();
    let gif_path = /* ... path resolution ... */;

    // Custom settings per DMD
    let (width, height, threshold) = match dmd {
        DmdAnimation::Invader => (124, 19, 50),   // Large, low threshold
        DmdAnimation::Skull => (124, 19, 50),     // Large, low threshold
        DmdAnimation::Sword => (124, 19, 50),     // Large, low threshold
        DmdAnimation::MyAnimation => (64, 8, 100), // Small, high threshold
    };

    let frames = converter::gif_to_frames(&gif_path, width, height, threshold)?;
    Ok(FrameBasedAnimation::from_braille_frames(frames, loop_animation))
}
```

## Finding Better DMD GIFs

### Ideal: Native 128×32 DMDs

Search for:
- "pinball DMD 128x32 GIF"
- VPUniverse DMD exports (128×32 native)
- Pin2DMD editor exports (actual DMD resolution)

### Converting Large GIFs

If you only have large GIFs (like Tenor 498×150):
1. Use larger cell sizes (124×19) to preserve detail
2. Lower threshold (40-60) to pick up darker pixels
3. Or resize GIF first to 128×32 with a tool

### Creating Custom DMDs

Best approach for perfect quality:
1. Create 128×32 pixel GIF in Aseprite/GIMP
2. Use monochrome (white on black)
3. Export as GIF
4. Convert with: `--width 64 --height 8 --threshold 100`

## Troubleshooting

### "I only see a black screen"
- **Threshold too high** → Try `--threshold 40`
- **GIF is all dark** → Check GIF in image viewer first

### "Animation is too bright/messy"
- **Threshold too low** → Try `--threshold 120`
- **Size too large** → Try `--width 64 --height 8`

### "Animation is pixelated/blocky"
- **Size too small** → Try `--width 124 --height 19`
- **Source GIF low quality** → Find better source

### "Colors look wrong"
- Braille only supports **brightness**, not color
- Orange DMD pixels → grayscale brightness
- Ensure high contrast in source

### "Animation too slow/fast"
- Frame timing comes from GIF metadata
- Re-export GIF with different frame delays
- Or edit with ezgif.com

## Recommended Workflow

1. **Find GIF** (Tenor, VPUniverse, create your own)
2. **Test with quality tester**:
   ```bash
   python tools/test_dmd_quality.py your.gif
   ```
3. **Pick best settings** from tests
4. **Add to library** with those settings
5. **Map to git command**
6. **Rebuild and enjoy!**

## Example: Adding Terminator DMD

```bash
# 1. Download GIF
curl -L "URL" -o ref/dmd_terminator.gif

# 2. Test quality
python tools/test_dmd_quality.py ref/dmd_terminator.gif
# → Looks best at 100x15, threshold 70

# 3. Edit src/dmd_library.rs
# Add Terminator variant, map to "git rebase"

# 4. Update load function for custom settings:
DmdAnimation::Terminator => (100, 15, 70),

# 5. Rebuild
cargo build --release --features gif

# 6. Test
crabcrust git rebase
```

---

**Remember**: Manual tuning gives you full control over quality. Take the time to find the right settings for each DMD!
