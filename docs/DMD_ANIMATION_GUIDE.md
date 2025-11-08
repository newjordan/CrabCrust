# Pinball DMD Animation Guide for CrabCrust

This guide shows how to import and use classic pinball Dot Matrix Display (DMD) animations in CrabCrust!

## Quick Start

```bash
# Install CrabCrust with GIF support
cargo install crabcrust --features gif

# Convert a DMD GIF to Braille animation
crabcrust convert your_dmd_animation.gif --play

# Use in your git workflow
crabcrust git commit -m "Epic commit with DMD animations!"
```

## What are DMD Animations?

DMD (Dot Matrix Display) animations are the iconic 128×32 pixel displays used in pinball machines since the 1990s. They show:
- Game scores and status
- Character animations
- Special effects
- Multiball celebrations
- Jackpot announcements

CrabCrust can convert these into Braille terminal animations!

## Finding DMD Animations

### Free DMD GIF Sources

1. **Tenor GIFs** - Search for "DMD pinball" or specific games
   - Example: [Monster Bash DMD](https://tenor.com/view/dmd-monster-bash-pinball-gif-18103635)
   - Direct download: Right-click → Save image

2. **VPUniverse** (vpuniverse.com)
   - Free downloads for registered members
   - Full-screen DMD animations for virtual pinball
   - Search: "DMD animated"

3. **LottieFiles** (lottiefiles.com)
   - Search: "pinball"
   - Export as GIF format
   - Free animated pinball graphics

4. **GitHub Projects**
   - [VincentBean/pinball-dmd-gif-panel](https://github.com/VincentBean/pinball-dmd-gif-panel)
   - Community-created 128×32 DMD GIFs

### Creating Your Own DMD Animations

Use any GIF creation tool:
- **Aseprite** - Pixel art GIF editor
- **GIMP** - Free image editor with GIF support
- **Photoshop** - Professional GIF creation
- **Online tools** - ezgif.com, gifmaker.me

**Recommended specs:**
- Dimensions: 128×32 pixels (classic DMD size)
- Colors: Monochrome or orange (authentic DMD look)
- Frame rate: 10-30 FPS
- File size: Keep under 5MB for best performance

## Converting DMD GIFs

### Basic Conversion

```bash
# Convert and preview
crabcrust convert monster_bash.gif --play

# Convert and loop
crabcrust convert jackpot.gif --play --loop-play

# Custom size (in terminal cells)
crabcrust convert dmd.gif --width 64 --height 8 --play
```

### Advanced Options

```bash
# Adjust brightness threshold (0-255)
# Higher = fewer bright pixels, lower = more bright pixels
crabcrust convert dmd.gif --threshold 100 --play

# Resize for different terminal sizes
crabcrust convert dmd.gif --width 80 --height 10 --play

# Get help
crabcrust convert --help
```

### Understanding the Conversion

CrabCrust converts DMD animations to Braille Unicode characters (U+2800–U+28FF):
- Each Braille cell = 2×4 dots
- Standard DMD (128×32 pixels) = 64×8 terminal cells
- Maintains aspect ratio and animation timing

## Using DMD Animations in Code

### Rust Integration

```rust
use crabcrust::{AnimationPlayer, FrameBasedAnimation};
use crabcrust::video::converter;

// Load DMD GIF
let frames = converter::gif_to_frames("jackpot.gif", 64, 8, 128)?;
let animation = FrameBasedAnimation::from_braille_frames(frames, true);

// Play inline (1/3 terminal height)
let mut player = AnimationPlayer::inline_auto()?;
player.play(animation)?;
```

### In Your CLI Tool

```rust
// Show DMD animation on successful git push
use crabcrust::wrapper::git::GitWrapper;

let mut git = GitWrapper::new()?;
let result = git.run(&["push", "origin", "main"])?;

if result.success() {
    // Play celebration DMD!
    let frames = converter::gif_to_frames("success.gif", 64, 8, 128)?;
    let animation = FrameBasedAnimation::from_braille_frames(frames, false);
    AnimationPlayer::inline_auto()?.play(animation)?;
}
```

## Classic DMD Animation Ideas

### Celebrations
- 🎉 **JACKPOT!** - Big score celebrations
- 🎯 **MULTIBALL!** - Multiple ball modes
- 🏆 **HIGH SCORE!** - New record achievements
- ⭐ **EXTRA BALL!** - Bonus awards

### Status Messages
- 💾 **SAVING...** - File operations
- ⬇️ **DOWNLOADING...** - Network operations
- 🔄 **PROCESSING...** - Long-running tasks
- ✅ **COMPLETE!** - Success messages

### Git-Specific
- 🎮 **COMMIT COMBO!** - Multiple commits
- 🚀 **DEPLOY MODE!** - Deployment success
- 🔥 **MERGE MANIA!** - Branch merging
- 💪 **BUILD BONUS!** - Successful builds

## DMD Animation Best Practices

### Performance
- Keep GIF files under 5MB
- Limit to 100 frames for smooth playback
- Use 128×32 resolution when possible (scales perfectly)

### Visual Design
- High contrast works best in terminals
- Simple animations are more readable
- Classic DMD orange color (#FF8000) is iconic
- Keep text large and bold

### Terminal Compatibility
- Test in your target terminal
- Some terminals may vary in Braille rendering
- Inline mode (1/3 height) is less disruptive
- Fullscreen mode for dramatic effects

## Troubleshooting

### "Feature not enabled" error
```bash
# Make sure to build with gif feature
cargo build --features gif
```

### GIF not converting
- Verify file is a valid GIF: `file your.gif`
- Check dimensions: Should be reasonable size
- Try lowering threshold: `--threshold 100`

### Animation looks wrong
- Adjust threshold for better contrast
- Resize: `--width 80 --height 10`
- Check GIF in image viewer first

### Slow performance
- Reduce GIF file size
- Limit frames: Use shorter animations
- Close other terminal applications

## Community Resources

### Get Help
- [GitHub Issues](https://github.com/anthropics/claude-code/issues)
- Search for "DMD" or "animation" tags

### Share Your DMDs
- Create awesome DMD animations for CrabCrust
- Share in GitHub Discussions
- Tag with #crabcrust #dmd

### Respect Copyright
- Use only DMDs you have rights to
- Many pinball DMDs are copyrighted
- Create original content or use public domain
- Credit original artists

## Example Workflow

Here's a complete workflow for adding a DMD animation:

```bash
# 1. Find or create a DMD GIF
curl -L "https://media1.tenor.com/m/Z2fGSx32xqcAAAAC/dmd-monster-bash.gif" -o monster_bash.gif

# 2. Test the conversion
crabcrust convert monster_bash.gif --play

# 3. Adjust if needed
crabcrust convert monster_bash.gif --threshold 120 --play

# 4. Use in your git workflow
crabcrust git commit -m "Add DMD support!"

# 5. Share your creation!
# (Open GitHub Discussions and post your favorite DMD!)
```

## Technical Details

### File Format Support
- ✅ **GIF** - Animated GIFs (with `--features gif`)
- ✅ **Video** - MP4, AVI, MOV, etc. (with `--features video`, requires ffmpeg)
- ❌ **PAC** - Pin2DMD proprietary format (encrypted, not supported)

### Feature Flags
- `gif` - GIF support only (no ffmpeg required)
- `video` - Full video + GIF support (requires ffmpeg libraries)

### Resolution Scaling

Input pixels → Terminal cells:
- 128×32 DMD → 64×8 cells (2:1 pixel-to-dot ratio)
- 256×64 DMD → 128×16 cells (2:1 pixel-to-dot ratio)
- Custom sizes → Nearest-neighbor scaling

## What's Next?

- 🎨 Create a DMD animation library
- 🎯 Add more pinball-themed animations
- 🚀 Share your coolest DMDs with the community
- 💡 Suggest features in GitHub Issues

---

**Happy pinball coding! 🦀🎮✨**

*Created by the CrabCrust community*
