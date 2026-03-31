#!/usr/bin/env python3
"""
Art generation script for Coco Loco.

Supports two backends:
  --backend dalle   (default) Uses OpenAI DALL-E 3 API (requires OPENAI_API_KEY)
  --backend flux    Uses Flux.1-schnell locally via Apple MPS / CUDA

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
            val = val.strip().strip("'\"")
            os.environ[key.strip()] = val

OUTPUT_DIR = REPO_ROOT / "assets" / "art" / "raw"

# ── Style anchors (derived from Storybook logo) ───────────────────────────────

# Full anchor for DALL-E (handles long prompts well)
ANCHOR_DALLE = (
    "Children's storybook illustration on a warm parchment-cream background "
    "(#FFF8E7 to #FFE4B5 gradient). Warm upper-left lighting. Soft watercolor "
    "brush strokes. Thick warm-brown outlines (#5C3317) around characters. "
    "Gold star accents (#D4A017). Round friendly shapes. "
    "Palette: orange (#FF6B35), teal (#4ECDC4), skin peach (#FFD4A0), "
    "dark brown (#5C3317), gold (#D4A017), cream (#FFF8E7). "
    "Style of a hand-illustrated children's picture book page. "
    "No text anywhere in the image."
)

# Short anchor for local models (CLIP has 77-token limit)
ANCHOR_LOCAL = (
    "Children's storybook watercolor illustration, warm parchment background, "
    "thick brown outlines, gold star accents, no text."
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
        "id": "loko_fury",
        "category": "characters",
        "prompt": (
            "Sprite sheet of Loko, 6-year-old boy superhero, in berserker rage mode. "
            "4 frames: 1) powering up with red-orange aura glowing around him, "
            "2) fury punch 1 with fist forward and energy trail, "
            "3) fury punch 2 with other fist and screen-shake energy lines, "
            "4) exhausted cooldown slumped over catching breath. "
            "Red and orange glow around character. Side view facing right. "
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

    # ── Star Meter UI ─────────────────────────────────────────────────────
    {
        "id": "star_meter_empty",
        "category": "ui",
        "prompt": (
            "A row of 5 star outlines for a children's game HUD. "
            "Stars are faint, empty, with thin brown outlines on parchment. "
            "Simple and clean. Each star is the same size in a horizontal row. "
            "Warm cream background."
        ),
        "size": "1024x1024",
    },
    {
        "id": "star_meter_filled",
        "category": "ui",
        "prompt": (
            "A row of 5 bright golden filled stars for a children's game HUD. "
            "Stars are glowing gold with sparkle highlights. Thick brown outlines. "
            "Each star is the same size in a horizontal row. "
            "Warm cream background."
        ),
        "size": "1024x1024",
    },

    # ── Guidance & Boss Defeat ────────────────────────────────────────────
    {
        "id": "guide_trail",
        "category": "effects",
        "prompt": (
            "A gentle glowing trail of small golden arrows pointing right, "
            "like breadcrumbs guiding a child forward in a game. "
            "Soft sparkle particles around the arrows. Warm golden light. "
            "Horizontal strip on transparent parchment background."
        ),
        "size": "1792x1024",
    },
    {
        "id": "explosion_big",
        "category": "effects",
        "prompt": (
            "A big celebratory cartoon explosion for defeating a boss in a children's game. "
            "4 frames: 1) bright golden flash, 2) expanding ring of stars and confetti, "
            "3) fireworks bursting with sparkles, 4) fading golden dust settling. "
            "Joyful and triumphant, not violent. Warm golden and orange colors. "
            "Parchment background."
        ),
        "size": "1024x1024",
    },

    # ── Platforms (Home Phase) ────────────────────────────────────────────
    {
        "id": "platform_block",
        "category": "platforms",
        "prompt": (
            "A colorful wooden building block toy, viewed from the side, "
            "for use as a platform in a children's side-scroller game. "
            "Rectangular, warm brown wood with a painted letter 'A' on it. "
            "Thick brown outlines. Parchment background. Simple and clear."
        ),
        "size": "1024x1024",
    },
    {
        "id": "platform_book",
        "category": "platforms",
        "prompt": (
            "A thick colorful children's picture book lying flat on its side, "
            "for use as a platform in a side-scroller game. "
            "Red and blue cover with gold spine. Viewed from the side. "
            "Thick brown outlines. Parchment background."
        ),
        "size": "1024x1024",
    },
    {
        "id": "platform_cushion",
        "category": "platforms",
        "prompt": (
            "A soft cozy sofa cushion lying on a floor, viewed from the side, "
            "for use as a bouncy platform in a children's side-scroller game. "
            "Warm orange fabric with a simple pattern. Slightly puffy and round. "
            "Thick brown outlines. Parchment background."
        ),
        "size": "1024x1024",
    },
    {
        "id": "ground_floor",
        "category": "platforms",
        "prompt": (
            "A seamless horizontal strip of warm wooden floor planks, "
            "for use as the ground in a children's side-scroller game. "
            "Warm honey-brown wood with subtle grain. Thick brown outlines. "
            "Viewed from the side. Must tile seamlessly left-to-right. "
            "Parchment background."
        ),
        "size": "1792x1024",
    },
]


# ── Backend: DALL-E 3 ──────────────────────────────────────────────────────────

def _init_dalle():
    from openai import OpenAI

    api_key = os.environ.get("OPENAI_API_KEY") or os.environ.get("OPENAI_KEY")
    if not api_key:
        print("ERROR: Set OPENAI_API_KEY or OPENAI_KEY in .env")
        sys.exit(1)
    return OpenAI(api_key=api_key)


def generate_dalle(entry: dict, client) -> str:
    category = entry["category"]
    art_id = entry["id"]
    prompt = f"{ANCHOR_DALLE} {entry['prompt']}"
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
        f"ID: {art_id}\nSize: {size}\nBackend: dalle\n\n"
        f"Original prompt:\n{prompt}\n\n"
        f"Revised prompt (DALL-E):\n{revised}\n"
    )

    print(f"OK → {out_path.relative_to(REPO_ROOT)}")
    return str(out_path)


# ── Backend: Local (SDXL Turbo — ungated, ~6.9 GB, fast) ──────────────────────

_local_pipe = None
_local_device = None


def _init_local():
    """Load SDXL Turbo pipeline once, reuse across generations."""
    global _local_pipe, _local_device
    if _local_pipe is not None:
        return _local_pipe

    print("Loading SDXL Turbo model (first run downloads ~6.9 GB)...")

    import torch

    # Determine device
    if torch.backends.mps.is_available():
        _local_device = "mps"
        dtype = torch.float16
        print(f"  Device: Apple MPS (Metal)")
    elif torch.cuda.is_available():
        _local_device = "cuda"
        dtype = torch.float16
        print(f"  Device: CUDA ({torch.cuda.get_device_name()})")
    else:
        _local_device = "cpu"
        dtype = torch.float32
        print(f"  Device: CPU (slow!)")

    from diffusers import AutoPipelineForText2Image

    _local_pipe = AutoPipelineForText2Image.from_pretrained(
        "stabilityai/sdxl-turbo",
        torch_dtype=dtype,
        variant="fp16" if dtype == torch.float16 else None,
    )
    _local_pipe.to(_local_device)

    print(f"  Pipeline loaded on {_local_device}")
    return _local_pipe


def _parse_size(size_str: str) -> tuple[int, int]:
    """Convert '1024x1024' to (width, height). SDXL Turbo works best at 512x512."""
    w, h = size_str.split("x")
    w, h = int(w), int(h)
    # SDXL Turbo produces best quality at 512x512.
    # For wide formats, use 768x512. Always multiples of 8.
    if w > h:
        return 768, 512
    return 512, 512


def generate_local(entry: dict, pipe) -> str:
    category = entry["category"]
    art_id = entry["id"]
    prompt = f"{ANCHOR_LOCAL} {entry['prompt']}"
    size_str = entry.get("size", "1024x1024")
    width, height = _parse_size(size_str)

    out_dir = OUTPUT_DIR / category
    out_dir.mkdir(parents=True, exist_ok=True)
    out_path = out_dir / f"{art_id}.png"
    prompt_path = out_dir / f"{art_id}.prompt.txt"

    if out_path.exists():
        print(f"  SKIP {art_id} (already exists)")
        return str(out_path)

    print(f"  Generating {art_id} ({width}x{height})...", end=" ", flush=True)

    image = pipe(
        prompt=prompt,
        guidance_scale=0.0,  # Turbo doesn't use guidance
        height=height,
        width=width,
        num_inference_steps=4,
    ).images[0]

    # Upscale to requested size for game use
    orig_w, orig_h = size_str.split("x")
    target_w, target_h = int(orig_w), int(orig_h)
    if (target_w, target_h) != (width, height):
        from PIL import Image
        image = image.resize((target_w, target_h), Image.LANCZOS)

    image.save(str(out_path))

    prompt_path.write_text(
        f"ID: {art_id}\nSize: {width}x{height} → upscaled to {target_w}x{target_h}\n"
        f"Backend: sdxl-turbo\nSteps: 4\n\n"
        f"Prompt:\n{prompt}\n"
    )

    print(f"OK → {out_path.relative_to(REPO_ROOT)}")
    return str(out_path)


# ── Main ───────────────────────────────────────────────────────────────────────

def main():
    import argparse

    parser = argparse.ArgumentParser(description="Coco Loco art generator")
    parser.add_argument(
        "--backend", choices=["dalle", "local"], default="local",
        help="Generation backend: 'dalle' (OpenAI API, paid) or 'local' (SDXL Turbo, free). Default: local"
    )
    parser.add_argument(
        "--only", type=str, default=None,
        help="Generate only a specific asset by ID (e.g., --only loko_reference)"
    )
    parser.add_argument(
        "--category", type=str, default=None,
        help="Generate only assets in a category (characters, enemies, backgrounds, etc.)"
    )
    args = parser.parse_args()

    prompts = PROMPTS
    if args.only:
        prompts = [p for p in prompts if p["id"] == args.only]
        if not prompts:
            print(f"ERROR: No asset with id '{args.only}'")
            sys.exit(1)
    elif args.category:
        prompts = [p for p in prompts if p["category"] == args.category]
        if not prompts:
            print(f"ERROR: No assets in category '{args.category}'")
            sys.exit(1)

    print(f"Coco Loco Art Generator — {len(prompts)} assets ({args.backend} backend)")
    print(f"Output: {OUTPUT_DIR}\n")

    # Init backend
    if args.backend == "dalle":
        backend_ctx = _init_dalle()
        gen_fn = generate_dalle
    else:
        backend_ctx = _init_local()
        gen_fn = generate_local

    results = {"generated": [], "skipped": [], "failed": []}

    for i, entry in enumerate(prompts, 1):
        print(f"[{i}/{len(prompts)}] {entry['id']}")
        try:
            path = gen_fn(entry, backend_ctx)
            if "SKIP" not in path:
                results["generated"].append(entry["id"])
            else:
                results["skipped"].append(entry["id"])
        except Exception as e:
            print(f"  FAILED: {e}")
            results["failed"].append({"id": entry["id"], "error": str(e)})

        # Rate limiting for DALL-E only
        if args.backend == "dalle" and i < len(prompts):
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
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    manifest_path.write_text(json.dumps(results, indent=2))
    print(f"\nManifest: {manifest_path}")


if __name__ == "__main__":
    main()
