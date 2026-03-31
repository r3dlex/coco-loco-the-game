#!/usr/bin/env python3
"""
DALL-E art generation script for Coco Loco.

Art direction derived from the Storybook logo (#03):
- Parchment/cream backgrounds (#FFF8E7 → #FFE4B5)
- Warm brown outlines and borders (#8B4513, #5C3317)
- Gold accents (#D4A017, #FFE066)
- Loko = orange (#FF6B35), Roco = teal (#4ECDC4), skin = #FFD4A0
- Georgia/Palatino serif italic feel — old storybook page aesthetic
- Warm upper-left lighting, soft brush strokes
- Round, friendly, non-threatening shapes
"""

import os
import sys
import json
import time
import urllib.request
from pathlib import Path

# Load .env
REPO_ROOT = Path(__file__).resolve().parents[3]
env_path = REPO_ROOT / ".env"
if env_path.exists():
    for line in env_path.read_text().splitlines():
        line = line.strip()
        if line and not line.startswith("#") and "=" in line:
            key, _, val = line.partition("=")
            val = val.strip().strip("'\"")  # strip quotes
            os.environ[key.strip()] = val

from openai import OpenAI

# Support both OPENAI_KEY and OPENAI_API_KEY
api_key = os.environ.get("OPENAI_API_KEY") or os.environ.get("OPENAI_KEY")
if not api_key:
    print("ERROR: Set OPENAI_API_KEY or OPENAI_KEY in .env")
    sys.exit(1)
client = OpenAI(api_key=api_key)

OUTPUT_DIR = REPO_ROOT / "assets" / "art" / "raw"

# ── Style anchor (derived from Storybook logo) ────────────────────────────────

ANCHOR = (
    "Children's storybook illustration on a warm parchment-cream background "
    "(#FFF8E7 to #FFE4B5 gradient). Warm upper-left lighting. Soft watercolor "
    "brush strokes. Thick warm-brown outlines (#5C3317) around characters. "
    "Gold star accents (#D4A017). Round friendly shapes. "
    "Palette: orange (#FF6B35), teal (#4ECDC4), skin peach (#FFD4A0), "
    "dark brown (#5C3317), gold (#D4A017), cream (#FFF8E7). "
    "Style of a hand-illustrated children's picture book page. "
    "No text anywhere in the image."
)

# ── Prompt definitions ─────────────────────────────────────────────────────────

PROMPTS = [
    # ── Characters ─────────────────────────────────────────────────────────
    {
        "id": "loko_reference",
        "category": "characters",
        "prompt": (
            "Character reference sheet for Loko, a 6-year-old boy superhero. "
            "Orange color scheme (#FF6B35). Messy spiky brown hair. Big confident grin. "
            "Wears an orange cape, star emblem on chest, oversized red sneakers. "
            "Peach skin (#FFD4A0). Stocky and energetic build. "
            "Show: front view, side view, back view, 3/4 action pose. "
            "Clean parchment background. Thick warm-brown outlines."
        ),
        "size": "1024x1024",
    },
    {
        "id": "loko_sprites",
        "category": "characters",
        "prompt": (
            "Sprite sheet of Loko, a 6-year-old boy superhero in orange cape. "
            "8 evenly spaced frames in a horizontal row: "
            "1) standing idle, 2) running frame 1, 3) running frame 2, "
            "4) jumping up, 5) falling down, 6) punching forward, "
            "7) funny knockback (silly face, stars around head), 8) celebrating arms up. "
            "Side view facing right. Peach skin, orange cape, star on chest. "
            "Parchment background."
        ),
        "size": "1792x1024",
    },
    {
        "id": "loko_dash",
        "category": "characters",
        "prompt": (
            "Sprite sheet of Loko, 6-year-old boy superhero, performing a super-speed dash. "
            "4 frames: 1) crouch wind-up, 2) horizontal blur with orange speed lines, "
            "3) mid-dash with afterimage trail, 4) skid stop with dust cloud. "
            "Orange speed lines trailing behind. Side view facing right. "
            "Parchment background. Thick brown outlines."
        ),
        "size": "1792x1024",
    },
    {
        "id": "roco_reference",
        "category": "characters",
        "prompt": (
            "Character reference sheet for Roco, a 3-year-old toddler superhero. "
            "Teal color scheme (#4ECDC4). Round chubby cheeks. Huge curious eyes. "
            "Wears a tiny teal cape and oversized boots. Star-shaped backpack. "
            "Peach skin (#FFD4A0). Smaller and rounder than his older brother. "
            "Show: front view, side view, back view, 3/4 view. "
            "Clean parchment background. Thick warm-brown outlines."
        ),
        "size": "1024x1024",
    },
    {
        "id": "roco_sprites",
        "category": "characters",
        "prompt": (
            "Sprite sheet of Roco, a 3-year-old toddler superhero in teal cape. "
            "8 evenly spaced frames in a horizontal row: "
            "1) standing idle (wobbles), 2) toddling run frame 1, 3) toddling run frame 2, "
            "4) jumping (flailing arms), 5) floating down gently, "
            "6) crying sonic wave attack (teal rings), 7) bonked (silly face), 8) happy dance. "
            "Side view facing right. Peach skin, teal cape, star backpack. "
            "Parchment background."
        ),
        "size": "1792x1024",
    },
    {
        "id": "roco_cry",
        "category": "characters",
        "prompt": (
            "Sprite sheet of Roco, 3-year-old toddler superhero, performing a sonic cry. "
            "4 frames: 1) inhale with puffed cheeks, 2) mouth wide open with teal sonic rings, "
            "3) sustained cry pushing enemies back, 4) tired exhale. "
            "Teal (#4ECDC4) sonic wave rings visible emanating forward. "
            "Side view facing right. Parchment background."
        ),
        "size": "1792x1024",
    },
    {
        "id": "roco_hammer",
        "category": "characters",
        "prompt": (
            "Sprite sheet of Roco, 3-year-old toddler, doing a ground pound with a tiny hammer. "
            "4 frames: 1) jumping up high, 2) raising tiny hammer overhead, "
            "3) slamming down with gold impact stars, 4) shockwave ring expanding. "
            "Side view. Parchment background. Thick brown outlines."
        ),
        "size": "1792x1024",
    },

    # ── Double Trouble Fusion ──────────────────────────────────────────────
    {
        "id": "fusion_reference",
        "category": "fusion",
        "prompt": (
            "Character reference for Double Trouble, a glowing fused superhero form. "
            "Combination of a 6-year-old (orange) and 3-year-old (teal) merged into one. "
            "Gold and white radiant glow. Swirling orange and teal energy. "
            "Enlarged star emblem on chest, shining brightly. Sparkle particle trail. "
            "Taller and more powerful looking. Show front view and action pose. "
            "Parchment background with golden light radiating outward."
        ),
        "size": "1024x1024",
    },
    {
        "id": "fusion_activate",
        "category": "fusion",
        "prompt": (
            "Sprite sequence of two kid superheroes merging into one glowing form. "
            "6 frames: 1) orange boy and teal toddler running toward each other, "
            "2) collision with golden star burst, 3) swirling orange+teal energy vortex, "
            "4) silhouette forming inside light, 5) bright flash, "
            "6) fused golden hero standing triumphantly. "
            "Lots of sparkles, stars, and golden light. Parchment background."
        ),
        "size": "1792x1024",
    },
    {
        "id": "fusion_deactivate",
        "category": "fusion",
        "prompt": (
            "Sprite sequence of a glowing fused superhero popping apart into two kids. "
            "4 frames: 1) golden form flickering and wobbling, 2) comic 'POP' effect with stars, "
            "3) two kids flying apart with hilarious surprised expressions, "
            "4) both landing with dizzy cartoon stars circling their heads. "
            "Funny and playful. Parchment background."
        ),
        "size": "1792x1024",
    },

    # ── Enemies ────────────────────────────────────────────────────────────
    {
        "id": "dust_bunny",
        "category": "enemies",
        "prompt": (
            "Sprite sheet of a cute dust bunny enemy for a children's game. "
            "Round fluffy gray ball with big googly eyes and tiny stubby feet. "
            "6 frames: 1) idle bounce, 2) patrol walk left, 3) patrol walk right, "
            "4) wind-up puff (getting bigger), 5) charge attack, "
            "6) defeated poof cloud dissipating with sparkles. "
            "Cute and silly, not scary at all. Parchment background. Brown outlines."
        ),
        "size": "1792x1024",
    },
    {
        "id": "boss_vacuum",
        "category": "enemies",
        "prompt": (
            "Character sheet of a giant sentient vacuum cleaner boss for a children's game. "
            "Big googly eyes on the front. Flexible hose acts like an elephant trunk arm. "
            "Wheels for feet. Has a goofy expression. "
            "4 views: 1) angry sucking pose, 2) dizzy resting (spiral eyes, sitting down), "
            "3) charging across room (wheels spinning fast), 4) phase 2 (red glow, angrier). "
            "Imposing but silly and not scary. Parchment background. Thick brown outlines."
        ),
        "size": "1024x1024",
    },
    {
        "id": "boss_vacuum_attacks",
        "category": "enemies",
        "prompt": (
            "Sprite sheet of a giant vacuum cleaner boss attacking in a side-scroller. "
            "6 frames: 1) idle hover, 2) suck attack with visible wind lines pulling inward, "
            "3) charge across screen with spinning wheels, 4) slam down creating shockwave, "
            "5) stunned/resting with dizzy spirals and sitting down, "
            "6) phase 2 transition glowing red with steam. "
            "Children's storybook style. Parchment background."
        ),
        "size": "1792x1024",
    },

    # ── Backgrounds (Home Phase) ───────────────────────────────────────────
    {
        "id": "bg_home_layer1_far",
        "category": "backgrounds",
        "prompt": (
            "Seamless horizontal parallax background layer for a children's game. "
            "Far distance: cozy house exterior seen through a window. "
            "Warm afternoon sky with soft puffy clouds. Garden with oversized whimsical flowers. "
            "Very soft and slightly blurry. Warm cream and amber tones. "
            "Upper-left warm lighting. Dreamy watercolor feel. "
            "Must tile seamlessly left-to-right. Landscape 16:9."
        ),
        "size": "1792x1024",
    },
    {
        "id": "bg_home_layer2_mid",
        "category": "backgrounds",
        "prompt": (
            "Seamless horizontal parallax background for a children's game. "
            "Mid distance: living room wall with framed family drawings (children's art), "
            "a wooden bookshelf with colorful book spines, a friendly cuckoo clock. "
            "Warm parchment tones. Upper-left lighting. Soft watercolor style. "
            "Must tile seamlessly left-to-right. Landscape 16:9."
        ),
        "size": "1792x1024",
    },
    {
        "id": "bg_home_layer3_near",
        "category": "backgrounds",
        "prompt": (
            "Seamless horizontal parallax background for a children's game. "
            "Near distance: giant furniture from a child's tiny perspective. "
            "Enormous sofa legs, massive wooden table leg, huge floor lamp. "
            "Everything towers overhead like a forest of furniture. "
            "Warm brown wood tones on parchment cream. Sharper details than far layers. "
            "Upper-left lighting. Must tile seamlessly. Landscape 16:9."
        ),
        "size": "1792x1024",
    },
    {
        "id": "bg_home_layer4_foreground",
        "category": "backgrounds",
        "prompt": (
            "Foreground elements for a children's side-scroller game set in a living room. "
            "Scattered on the floor: colorful building blocks, a toy fire truck, "
            "crayons, a teddy bear, a ball. These serve as platforms and obstacles. "
            "Each item clearly separated with space between them. "
            "Sharp thick brown outlines. Warm parchment background. "
            "Items should be usable as game platforms. Landscape 16:9."
        ),
        "size": "1792x1024",
    },

    # ── Pickups & Effects ──────────────────────────────────────────────────
    {
        "id": "star_pickup",
        "category": "pickups",
        "prompt": (
            "Sprite sheet of a golden star collectible for a children's game. "
            "A cute 5-pointed star with a tiny happy face (dot eyes, small smile). "
            "Glowing golden (#D4A017 and #FFE066). "
            "4 frames: 1) idle gentle glow, 2) rotating 30 degrees, "
            "3) rotating 60 degrees, 4) rotating 90 degrees with sparkle. "
            "Each frame 64x64. Parchment background. Thick brown outlines on star."
        ),
        "size": "1024x1024",
    },
    {
        "id": "effect_bonk",
        "category": "effects",
        "prompt": (
            "Sprite sheet of a comic 'bonk' hit effect for a children's game. "
            "4 frames: 1) impact flash with cartoon stars, 2) expanding ring of tiny stars "
            "and birds circling, 3) stars scattering outward, 4) fading away. "
            "Bright warm colors: gold stars, orange flash, brown outlines. "
            "Funny and silly, not violent. Parchment background."
        ),
        "size": "1024x1024",
    },
    {
        "id": "effect_poof",
        "category": "effects",
        "prompt": (
            "Sprite sheet of a cute 'poof' cloud for enemy defeat in a children's game. "
            "4 frames: 1) small initial puff, 2) expanding soft cloud with golden sparkles, "
            "3) cloud starting to dissipate with lingering sparkles, 4) almost gone, just sparkles. "
            "Soft warm gray cloud with gold (#D4A017) sparkle accents. "
            "Parchment background. Gentle and satisfying."
        ),
        "size": "1024x1024",
    },

    # ── Co-op UI Banners ───────────────────────────────────────────────────
    {
        "id": "banner_player2",
        "category": "ui",
        "prompt": (
            "A festive game banner for 'Player 2 joined' moment. "
            "Shows two cute kid superhero silhouettes (one orange, one teal) high-fiving. "
            "Confetti and tiny stars bursting around them. Warm golden light. "
            "Celebratory and exciting. No text in the image. "
            "Landscape banner shape (4:1 ratio). Parchment background with golden border."
        ),
        "size": "1792x1024",
    },
    {
        "id": "banner_team_up",
        "category": "ui",
        "prompt": (
            "An exciting game banner for a 'Team Up' co-op moment. "
            "Two kid superhero silhouettes (orange and teal) standing together powerfully. "
            "Lightning bolt and star effects between them. Musical notes suggesting a fanfare. "
            "Orange and teal energy combining. Warm parchment background. "
            "No text. Landscape banner shape. Golden storybook border."
        ),
        "size": "1792x1024",
    },
    {
        "id": "banner_double_trouble",
        "category": "ui",
        "prompt": (
            "An explosive celebratory banner for a 'Double Trouble' fusion activation. "
            "A radiant golden silhouette of a fused superhero in the center. "
            "Radiating star burst with fireworks and sparkles all around. "
            "Orange and teal energy swirling into gold. Triumphant and exciting. "
            "Parchment background with ornate golden storybook border. "
            "No text. Landscape shape."
        ),
        "size": "1792x1024",
    },
]


def generate_one(entry: dict) -> str:
    """Generate a single image and save it. Returns the output path."""
    category = entry["category"]
    art_id = entry["id"]
    prompt = f"{ANCHOR} {entry['prompt']}"
    size = entry.get("size", "1024x1024")

    out_dir = OUTPUT_DIR / category
    out_dir.mkdir(parents=True, exist_ok=True)
    out_path = out_dir / f"{art_id}.png"
    prompt_path = out_dir / f"{art_id}.prompt.txt"

    if out_path.exists():
        print(f"  SKIP {art_id} (already exists)")
        return str(out_path)

    print(f"  Generating {art_id} ({size})...", end=" ", flush=True)

    response = client.images.generate(
        model="dall-e-3",
        prompt=prompt,
        size=size,
        quality="hd",
        n=1,
    )

    url = response.data[0].url
    revised = response.data[0].revised_prompt or ""
    urllib.request.urlretrieve(url, str(out_path))

    prompt_path.write_text(
        f"ID: {art_id}\nSize: {size}\n\n"
        f"Original prompt:\n{prompt}\n\n"
        f"Revised prompt (DALL-E):\n{revised}\n"
    )

    print(f"OK → {out_path.relative_to(OUTPUT_DIR.parent.parent)}")
    return str(out_path)


def main():
    print(f"Coco Loco Art Generator — {len(PROMPTS)} assets")
    print(f"Output: {OUTPUT_DIR}\n")

    results = {"generated": [], "skipped": [], "failed": []}

    for i, entry in enumerate(PROMPTS, 1):
        print(f"[{i}/{len(PROMPTS)}] {entry['id']}")
        try:
            path = generate_one(entry)
            if "SKIP" not in path:
                results["generated"].append(entry["id"])
            else:
                results["skipped"].append(entry["id"])
        except Exception as e:
            print(f"  FAILED: {e}")
            results["failed"].append({"id": entry["id"], "error": str(e)})

        # Rate limiting: DALL-E 3 allows ~7 images/min on Tier 1
        if i < len(PROMPTS):
            time.sleep(10)

    # Summary
    print(f"\n{'='*50}")
    print(f"Generated: {len(results['generated'])}")
    print(f"Skipped:   {len(results['skipped'])}")
    print(f"Failed:    {len(results['failed'])}")

    if results["failed"]:
        print("\nFailed assets:")
        for f in results["failed"]:
            print(f"  - {f['id']}: {f['error']}")

    # Save manifest
    manifest_path = OUTPUT_DIR / "manifest.json"
    manifest_path.write_text(json.dumps(results, indent=2))
    print(f"\nManifest: {manifest_path}")


if __name__ == "__main__":
    main()
