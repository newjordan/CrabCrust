#!/usr/bin/env python3
"""
DMD GIF Quality Tester

Test different conversion parameters to find the best settings for each DMD GIF.
Usage: python tools/test_dmd_quality.py path/to/your.gif
"""

import sys
import subprocess
import os

def test_conversion(gif_path, width, height, threshold):
    """Test a conversion with specific parameters"""
    cmd = [
        "cargo", "run", "--release", "--features", "gif", "--",
        "convert", gif_path,
        "--width", str(width),
        "--height", str(height),
        "--threshold", str(threshold),
        "--play"
    ]

    print(f"\n{'='*60}")
    print(f"Testing: {width}x{height} cells, threshold {threshold}")
    print(f"{'='*60}")

    try:
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError:
        print("❌ Conversion failed")
    except KeyboardInterrupt:
        print("\n⏭️  Skipped")

def main():
    if len(sys.argv) < 2:
        print("Usage: python tools/test_dmd_quality.py path/to/your.gif")
        sys.exit(1)

    gif_path = sys.argv[1]

    if not os.path.exists(gif_path):
        print(f"❌ File not found: {gif_path}")
        sys.exit(1)

    print(f"🎮 DMD Quality Tester")
    print(f"📁 Testing: {gif_path}")
    print(f"\nPress Ctrl+C to skip to next test\n")

    # Test different combinations
    tests = [
        # (width, height, threshold, description)
        (64, 8, 128, "Standard DMD (128x32 dots), high threshold"),
        (64, 8, 80, "Standard DMD (128x32 dots), medium threshold"),
        (64, 8, 50, "Standard DMD (128x32 dots), low threshold"),
        (124, 19, 128, "Large size (248x76 dots), high threshold"),
        (124, 19, 80, "Large size (248x76 dots), medium threshold"),
        (124, 19, 50, "Large size (248x76 dots), low threshold"),
        (100, 15, 60, "Medium size (200x60 dots), balanced"),
    ]

    for width, height, threshold in tests:
        try:
            test_conversion(gif_path, width, height, threshold)
            input("\nPress Enter to continue to next test (or Ctrl+C to quit)...")
        except KeyboardInterrupt:
            print("\n\n✅ Testing complete!")
            break

    print("\n💡 Recommendation:")
    print("   - If too dark/faint: LOWER threshold (try 30-50)")
    print("   - If too bright/bloated: RAISE threshold (try 100-150)")
    print("   - If pixelated: INCREASE size (try 124x19)")
    print("   - If too detailed/noisy: DECREASE size (try 64x8)")

if __name__ == "__main__":
    main()
