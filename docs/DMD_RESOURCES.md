# DMD Animation Resources

A curated list of resources for finding and creating pinball DMD animations for CrabCrust.

## 🎮 Ready-to-Use DMD GIF Sources

### Tenor (Free, No Account)
Search for DMD pinball animations on Tenor:
- Search: ["DMD pinball"](https://tenor.com/search/dmd-pinball-gifs)
- Search: ["pinball display"](https://tenor.com/search/pinball-display-gifs)
- **How to use:**
  1. Find a GIF you like
  2. Right-click → "Copy image address"
  3. `curl -L "URL" -o my_dmd.gif`
  4. `crabcrust convert my_dmd.gif --play`

### Example DMD GIFs
Here are some direct download examples:

```bash
# Monster Bash (498×144, scales well)
curl -L "https://media1.tenor.com/m/Z2fGSx32xqcAAAAC/dmd-monster-bash.gif" -o monster_bash.gif

# Then convert:
crabcrust convert monster_bash.gif --play
```

## 🏛️ Pinball Community Sites

### VPUniverse (Free with Registration)
- **URL:** https://vpuniverse.com
- **Search:** "DMD animated" or "FullDMD"
- **Content:** Full-screen DMD videos and GIFs for virtual pinball
- **Format:** Various (convert to GIF if needed)
- **Note:** Register for free downloads

### VPForums (Free with Registration)
- **URL:** https://www.vpforums.org
- **Content:** Community discussions and DMD resources
- **Topics:** DMD animation techniques and files

### Pinside (Hardware/Modding)
- **URL:** https://pinside.com
- **Content:** Real pinball DMD discussions
- **Use for:** Understanding authentic DMD styles

## 🛠️ Creation Tools

### Aseprite (Paid, $19.99)
- **URL:** https://www.aseprite.org/
- **Best for:** Pixel art GIF creation
- **DMD specs:**
  - Canvas: 128×32 pixels
  - Frame rate: 10-30 FPS
  - Export: Animated GIF

### GIMP (Free)
- **URL:** https://www.gimp.org/
- **Best for:** Free alternative to Photoshop
- **DMD workflow:**
  1. Create 128×32 canvas
  2. Make each frame a layer
  3. Export as GIF
  4. Filters → Animation → Optimize (for GIF)

### ezgif.com (Free, Online)
- **URL:** https://ezgif.com/maker
- **Best for:** Quick GIF creation from images
- **Features:**
  - Resize to 128×32
  - Adjust speed/timing
  - Optimize file size

### LottieFiles (Free Animations)
- **URL:** https://lottiefiles.com/free-animations/pinball
- **Content:** Free pinball-themed animations
- **Export:** Can export as GIF
- **Note:** May need resizing to 128×32

## 🎨 Design Inspiration

### Classic DMD Color Palettes

**Authentic Orange DMD:**
- Background: `#000000` (black)
- Dots: `#FF8000` (orange)
- Style: Monochrome, high contrast

**Color DMD (Modern):**
- Multiple colors (red, orange, yellow, white)
- Keep high contrast for terminal readability

**Minimal (Best for Terminals):**
- Black and white only
- Maximum readability in Braille

### Typography

Best fonts for 128×32 DMD text:
- **Bold, blocky fonts** (5-7px height)
- **Arcade classics:** Press Start 2P, Joystix
- **Bitmap fonts:** Terminal, Courier
- **Keep it LARGE** - Small text won't work well

### Animation Timing

Typical DMD animation speeds:
- **Fast flash:** 50-100ms per frame
- **Normal:** 100-150ms per frame
- **Slow reveal:** 200-300ms per frame
- **Total length:** 1-3 seconds (looped)

## 📚 GitHub Projects

### pinball-dmd-gif-panel
- **URL:** https://github.com/VincentBean/pinball-dmd-gif-panel
- **Description:** Raspberry Pi DMD panel that plays GIFs
- **Useful for:** Understanding 128×32 GIF format
- **License:** Check repo

### dmd-extensions
- **URL:** https://github.com/freezy/dmd-extensions
- **Description:** Virtual pinball DMD toolkit (C#)
- **Useful for:** Understanding DMD file formats
- **Note:** .pac files are encrypted, not usable

### artwork4DMD
- **Search GitHub:** "artwork4DMD"
- **Description:** DMD artwork conversion tools
- **Useful for:** Converting GIFs to DMD hardware formats

## 🎬 Video to GIF Conversion

If you find DMD videos instead of GIFs:

### Using ffmpeg (Command Line)
```bash
# Convert video to 128×32 GIF
ffmpeg -i dmd_video.mp4 -vf "scale=128:32:flags=neighbor" -r 20 output.gif

# Optimize for size
ffmpeg -i dmd_video.mp4 -vf "scale=128:32:flags=neighbor,fps=20" \
  -c:v gif -f gif output.gif
```

### Using ezgif.com (Online)
1. Go to https://ezgif.com/video-to-gif
2. Upload your video
3. Set size to 128×32
4. Set frame rate (10-30 FPS)
5. Download GIF

### Using CrabCrust (with video feature)
```bash
# Requires --features video (and ffmpeg libraries)
cargo install crabcrust --features video

# Convert video directly
crabcrust convert dmd_video.mp4 --play
```

## 🎯 Classic Pinball Game DMDs

Popular pinball machines with iconic DMDs (search for these):
- **Medieval Madness** - Castle attacks, trolls
- **Monster Bash** - Universal monsters
- **Attack from Mars** - Alien invasions
- **The Addams Family** - Thing, mansion
- **Twilight Zone** - Clock, spiral animations
- **Star Trek: The Next Generation** - Enterprise
- **Indiana Jones** - Adventure scenes
- **Terminator 2** - Skull, T-800
- **Lord of the Rings** - Ring, characters
- **The Simpsons** - Homer, couch gags

Search format: `"[Game Name] DMD gif"`

## 📏 DMD Specifications

### Standard DMD Sizes

**Classic:**
- 128×32 pixels
- Monochrome (orange)
- 1990s-2000s pinball

**HD DMD:**
- 256×64 pixels (2× classic)
- Full color
- Modern pinball

**Full Screen:**
- 1920×1080 (scales to DMD region)
- Used in virtual pinball
- Crop to 128×32 for CrabCrust

### CrabCrust Recommendations

For best results:
- **Resolution:** 128×32 pixels (scales perfectly to 64×8 cells)
- **Colors:** High contrast (black + bright color)
- **Format:** Animated GIF
- **Size:** Under 5MB
- **Frames:** Under 100 frames
- **Duration:** 1-3 seconds
- **Frame rate:** 10-30 FPS

## ⚖️ Copyright & Legal

### Important Notes
- Many pinball DMDs are copyrighted by manufacturers
- Game character/logo DMDs belong to license holders
- **Use responsibly:**
  - Create original content
  - Use public domain resources
  - Don't share copyrighted materials publicly
  - Personal use vs. distribution

### Safe Options
- ✅ Create your own DMD animations
- ✅ Use generic text/effects (JACKPOT, MULTIBALL, etc.)
- ✅ Free resources from LottieFiles, Tenor
- ✅ Public domain pinball imagery
- ✅ Parody/homage (with caution)

### Not Safe
- ❌ Ripping DMDs from commercial pinball ROMs
- ❌ Distributing copyrighted game assets
- ❌ Selling others' DMD artwork
- ❌ Claiming others' work as your own

## 🤝 Contributing

Found a great DMD resource? Share it!

1. Fork CrabCrust repo
2. Add to this file
3. Submit PR
4. Help the community!

### Submission Format
```markdown
### Resource Name
- **URL:** https://example.com
- **Description:** What it offers
- **Best for:** Primary use case
- **License/Cost:** Free/Paid/Registration required
```

## 📱 Mobile Apps for DMD Creation

### Pixaki (iOS, Paid)
- Pixel art editor for iPad
- Export animated GIFs
- Great for 128×32 creation

### Dotpict (iOS/Android, Free)
- Pixel art app
- Simple GIF export
- Good for beginners

### Pixel Studio (Android, Free/Paid)
- Mobile pixel art editor
- Animation support
- GIF export

## 🎓 Tutorials

### Creating Your First DMD GIF

1. **Choose your tool** (GIMP, Aseprite, online)
2. **Create 128×32 canvas**
3. **Design your animation:**
   - Frame 1: Empty or dim
   - Frame 2: Flash bright
   - Frame 3-N: Main content
4. **Export as GIF** (100ms/frame)
5. **Test with CrabCrust:**
   ```bash
   crabcrust convert my_first_dmd.gif --play
   ```
6. **Iterate:** Adjust brightness, timing, content

### Optimizing GIF Size

Large GIFs slow down conversion. Optimize:

1. **Reduce frames:** Remove redundant frames
2. **Limit colors:** Use 2-4 colors max
3. **Lower resolution:** 128×32 is perfect
4. **Use ezgif.com optimizer:**
   - https://ezgif.com/optimize
   - Upload your GIF
   - Choose optimization level

## 🔗 Quick Links

- [CrabCrust DMD Guide](./DMD_ANIMATION_GUIDE.md)
- [CrabCrust GitHub](https://github.com/newjordan/CrabCrust)
- [Pin2DMD.com](https://pin2dmd.com/) - Real DMD hardware
- [VPUniverse Forums](https://vpuniverse.com/forums/)
- [/r/pinball subreddit](https://reddit.com/r/pinball)

---

**Have a resource to add? Open a PR or issue!**

*Last updated: 2025-11-08*
