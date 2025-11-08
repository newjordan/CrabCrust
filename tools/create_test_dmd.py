#!/usr/bin/env python3
"""
Create a test 128x32 DMD animation for validating CrabCrust conversion pipeline
Classic pinball DMD style with "JACKPOT" animation
"""

from PIL import Image, ImageDraw, ImageFont
import os

# DMD dimensions
WIDTH = 128
HEIGHT = 32
FRAMES = 12

def create_frame(frame_num):
    """Create a single frame of the animation"""
    img = Image.new('RGB', (WIDTH, HEIGHT), color=(0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Classic DMD orange color
    color = (255, 128, 0)

    # Animation phases
    if frame_num < 3:
        # Flash full screen
        if frame_num % 2 == 0:
            draw.rectangle([(0, 0), (WIDTH, HEIGHT)], fill=color)
    elif frame_num < 6:
        # Draw border
        thickness = 2
        draw.rectangle([(0, 0), (WIDTH-1, HEIGHT-1)], outline=color, width=thickness)
        # Add "JACKPOT" text
        text = "JACKPOT"
        # Calculate text position (centered)
        bbox = draw.textbbox((0, 0), text)
        text_width = bbox[2] - bbox[0]
        text_height = bbox[3] - bbox[1]
        x = (WIDTH - text_width) // 2
        y = (HEIGHT - text_height) // 2 - 2
        draw.text((x, y), text, fill=color)
    else:
        # Animate stars/sparkles around text
        import random
        random.seed(frame_num)

        # Draw border
        draw.rectangle([(0, 0), (WIDTH-1, HEIGHT-1)], outline=color, width=2)

        # Draw "JACKPOT" text
        text = "JACKPOT"
        bbox = draw.textbbox((0, 0), text)
        text_width = bbox[2] - bbox[0]
        text_height = bbox[3] - bbox[1]
        x = (WIDTH - text_width) // 2
        y = (HEIGHT - text_height) // 2 - 2
        draw.text((x, y), text, fill=color)

        # Add random sparkles
        for _ in range(10):
            sx = random.randint(2, WIDTH-3)
            sy = random.randint(2, HEIGHT-3)
            draw.point((sx, sy), fill=color)
            # Make some sparkles bigger
            if random.random() > 0.5:
                draw.point((sx+1, sy), fill=color)
                draw.point((sx, sy+1), fill=color)

    return img

def main():
    print("🎰 Creating test DMD animation (128x32)...")

    # Create frames
    frames = []
    for i in range(FRAMES):
        print(f"   Frame {i+1}/{FRAMES}")
        frame = create_frame(i)
        frames.append(frame)

    # Save as animated GIF
    output_path = "/home/user/CrabCrust/ref/test_jackpot_dmd.gif"
    frames[0].save(
        output_path,
        save_all=True,
        append_images=frames[1:],
        duration=100,  # 100ms per frame
        loop=0  # Loop forever
    )

    print(f"\n✨ Created: {output_path}")
    print(f"   Dimensions: {WIDTH}x{HEIGHT}")
    print(f"   Frames: {FRAMES}")
    print(f"   Duration: {FRAMES * 100}ms")
    print(f"\n💡 Test conversion:")
    print(f"   crabcrust convert {output_path} --play --save jackpot")

if __name__ == "__main__":
    main()
