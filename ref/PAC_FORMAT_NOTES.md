# Pin2DMD .pac File Format Analysis

## File: pin2dmd.pac (TMNT Animations)

### Header (8 bytes)
```
50 41 43 20  01 00  01 00
"PAC "       v1.0   fmt:0x0001
```

### Findings

**Size**: 635,233 bytes (620 KB)
- Header: 8 bytes  
- Payload: 635,225 bytes

**Compression**: ❌ Not standard (DEFLATE/ZLIB/GZIP all failed)

**Entropy**: 8.00 bits/byte = **Very high** (compressed or encrypted)

**Structure**: Unknown/Proprietary
- No clear frame boundaries
- High byte distribution (appears random)
- Likely custom compression or encryption

### Standard DMD Format
- Resolution: 128×32 pixels = 4,096 pixels
- Color depth: Usually 4-bit (16 colors) or 8-bit (256 colors)
- Frame size: 2,048 bytes (4-bit) or 4,096 bytes (8-bit)

### Estimated Content
If 155 frames @ 4096 bytes/frame (8-bit):
- Uncompressed: ~633 KB
- Matches file size! (Likely compressed 155 frames)

## Problem

Pin2DMD .pac files appear to be **proprietary format**, possibly:
1. Custom compression algorithm
2. Encrypted for licensing/DRM
3. Obfuscated binary format

## Solution: Alternative Workflow

Instead of parsing .pac files, use this workflow:

### Option 1: Export from Pin2DMD
1. Use Pin2DMD Editor or other tools to export animations
2. Export as GIF or video
3. Convert to CrabCrust with: `crabcrust convert animation.gif`

### Option 2: Find DMD Videos/GIFs
1. Search for pinball DMD animations as GIF/MP4
2. Many available online from pinball community
3. Convert directly: `crabcrust convert dmd_jackpot.gif --save jackpot`

### Option 3: Create Custom
1. Record pinball machine DMD with screen capture
2. Or create 128×32 animations in any tool
3. Export as GIF and convert

## Recommendation

**Skip .pac parsing** - too much reverse engineering for proprietary format.

**Use GIF/video pipeline** - already working in CrabCrust!

```bash
# Find DMD GIFs online or export from Pin2DMD Editor
# Then convert:
crabcrust convert tmnt_jackpot.gif --play --save tmnt/jackpot
```

This gives you authentic DMD animations without fighting proprietary formats! 🎮
