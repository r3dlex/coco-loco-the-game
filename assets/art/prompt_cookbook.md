# Coco Loco — DALL-E Prompt Cookbook

All prompts use a **consistency anchor** to maintain visual unity across assets:

> **Anchor:** "Children's storybook illustration, warm upper-left lighting, soft brush strokes, 32-color palette, 1-2px dark outlines on characters, pastel backgrounds"

## Workflow

1. Generate with DALL-E 3 (via API or ChatGPT)
2. Clean up in Krita (remove artifacts, fix outlines, ensure palette)
3. Apply phase LUT for color consistency
4. Export to sprite sheet via TexturePacker
5. Import into Bevy

---

## 1. Character Reference Sheets

### 1.1 Loko (6-year-old, orange theme)

```
Children's storybook character reference sheet for a 6-year-old boy superhero named Loko.
Orange and warm yellow color scheme. Messy spiky hair. Confident wide grin. Oversized sneakers.
Wears an orange cape and a star emblem on his chest.
Show: front view, side view, back view, 3/4 view.
Clean white background. Thick dark outlines. Soft brush stroke style.
No text. No labels. Simple and expressive like a Pixar junior character.
```

### 1.2 Loko Sprite Poses

```
Sprite sheet of a 6-year-old boy superhero in orange cape, children's storybook style.
8 frames in a row: standing idle, running frame 1, running frame 2, jumping up,
falling down, punching forward, taking a hit (funny knockback), celebrating (arms up).
Side view, facing right. Thick dark outlines. Clean white background.
Warm upper-left lighting. Soft brush strokes. No text.
```

### 1.3 Loko — Dash Ability

```
Sprite sheet of a 6-year-old boy superhero dashing forward at super speed.
4 frames: wind-up crouch, blur dash frame 1, blur dash frame 2, skid stop.
Orange speed lines trailing behind. Children's storybook style.
Side view facing right. White background. Thick outlines. No text.
```

### 1.4 Loko — Devastating Fury

```
Sprite sheet of a 6-year-old boy superhero in berserker rage mode.
4 frames: powering up (red aura), fury punch 1, fury punch 2, exhausted cooldown.
Red and orange glow around character. Screen-shake energy lines.
Children's storybook style. Side view. White background. No text.
```

### 1.5 Roco (3-year-old, teal theme)

```
Children's storybook character reference sheet for a 3-year-old toddler superhero named Roco.
Teal and mint green color scheme. Round cheeks. Big curious eyes. Wears a tiny teal cape
and oversized boots. Carries a small star-shaped backpack.
Show: front view, side view, back view, 3/4 view.
Clean white background. Thick dark outlines. Soft brush stroke style.
Cuter and rounder than his older brother. No text. No labels.
```

### 1.6 Roco Sprite Poses

```
Sprite sheet of a 3-year-old toddler superhero in teal cape, children's storybook style.
8 frames in a row: standing idle (wobbles slightly), toddling run frame 1, toddling run frame 2,
jumping (flailing arms), floating down, crying sonic wave, getting bonked (silly face), happy dance.
Side view, facing right. Thick dark outlines. Clean white background.
Warm upper-left lighting. Soft brush strokes. No text.
```

### 1.7 Roco — Super Cry

```
Sprite sheet of a 3-year-old toddler superhero doing a sonic cry attack.
4 frames: inhale (puffed cheeks), crying (mouth wide open, sonic rings emanating),
sustained cry (enemies pushed back), tired exhale.
Teal sonic wave rings visible. Children's storybook style.
Side view facing right. White background. No text.
```

### 1.8 Roco — Mighty Hammer

```
Sprite sheet of a 3-year-old toddler superhero doing a ground pound with a tiny hammer.
4 frames: jumping up, raising hammer overhead, slamming down (impact stars),
shockwave expanding outward.
Children's storybook style. Side view. White background. No text.
```

---

## 2. Double Trouble (Fusion Form)

### 2.1 Fused Form Reference

```
Children's storybook character reference sheet for a glowing fused superhero.
Combination of a 6-year-old and 3-year-old merged into one radiant figure.
Gold and white glow. Both orange and teal colors swirling together.
Star emblem on chest, now enlarged and shining. Sparkle particle trail.
Show: front view, side view, action pose.
White background. Thick outlines with glow effect. No text.
```

### 2.2 Fusion Activation Sequence

```
Sprite sequence: two kid superheroes (one orange, one teal) merging into one glowing form.
6 frames: both running toward each other, collision with star burst,
swirling energy merge, silhouette forming, flash of light, fused hero standing.
Children's storybook style. Lots of sparkles and stars. White background. No text.
```

### 2.3 Fusion De-activation (Pop Apart)

```
Sprite sequence: a glowing fused superhero splitting back into two kids.
4 frames: form flickering and wobbling, comic pop effect, two kids flying apart
with silly expressions, both landing with dizzy stars.
Children's storybook style. Funny and playful. White background. No text.
```

---

## 3. Enemies

### 3.1 Dust Bunny (Home Phase)

```
Sprite sheet of a cute dust bunny enemy for a children's game.
A round fluffy gray ball with googly eyes and tiny feet.
6 frames: idle bounce, patrol walk left, patrol walk right,
wind-up attack (puffs up), charge attack, defeated (poof cloud dissipating).
Children's storybook style. Soft brush strokes. White background. No text.
Not scary — cute and silly.
```

### 3.2 Boss: Giant Vacuum Cleaner (Home Phase)

```
Character sheet of a giant sentient vacuum cleaner boss for a children's game.
Googly eyes on the front. Hose acts like an arm/trunk. Wheels for feet.
Expressions: angry (sucking), dizzy (resting/stunned with spiral eyes),
charging across room, phase 2 (red glow, faster, mouth open wider).
Children's storybook style. Large and imposing but silly, not scary.
White background. Thick outlines. No text.
```

### 3.3 Boss — Vacuum Attack Sprites

```
Sprite sheet of a giant vacuum cleaner boss attacking in a side-scroller game.
6 frames: idle hover, suck attack (pulls player toward it with wind lines),
charge across screen (wheels spinning), slam down, stunned/resting (dizzy spirals),
phase 2 transition (glowing red, steam coming out).
Children's storybook style. White background. No text.
```

---

## 4. Home Phase Backgrounds (Parallax Layers)

### 4.1 Layer 1 — Far Background (slowest scroll)

```
Children's storybook illustration of a cozy house exterior seen through a window.
Warm afternoon sky with soft clouds. A garden with oversized flowers.
Distant trees. Warm upper-left lighting. Very soft and blurry.
Pastel colors. No characters. No text. Seamless horizontal tile.
640x360 pixels, landscape orientation.
```

### 4.2 Layer 2 — Mid Background

```
Children's storybook illustration of a living room wall with family photos,
a bookshelf, and a clock. Warm lighting from upper left.
Slightly soft focus. Cozy and inviting. Pastel warm tones.
No characters. No text. Seamless horizontal tile.
640x360 pixels.
```

### 4.3 Layer 3 — Near Background

```
Children's storybook illustration of giant furniture in a living room
from a child's perspective. Huge sofa legs, enormous table,
oversized lamp. Everything looks massive.
Warm upper-left lighting. Slightly sharper than the far background.
No characters. No text. Seamless horizontal tile. 640x360 pixels.
```

### 4.4 Layer 4 — Foreground Elements

```
Children's storybook illustration of scattered toys and household items
on a living room floor. Building blocks, a toy truck, scattered crayons,
a teddy bear. These items serve as obstacles and platforms.
Sharp outlines. Warm lighting. Transparent/white background.
640x360 pixels. No text.
```

---

## 5. Pickups and UI Elements

### 5.1 Star Pickup

```
A single golden star collectible for a children's game.
Glowing, sparkly, with a cute face (two dots for eyes, small smile).
4 frames: idle glow, spinning frame 1, spinning frame 2, spinning frame 3.
Children's storybook style. Transparent/white background. No text.
64x64 pixels each frame.
```

### 5.2 Star Meter UI

```
UI element for a children's game: a row of 5 star outlines.
Each star can be empty (faint outline) or filled (bright gold with sparkle).
Show both states side by side.
Children's storybook style. Warm gold color. Simple and clear.
Transparent background. No text.
```

### 5.3 Hit Effect — Bonk

```
Comic hit effect for a children's game. Silly "bonk" impact.
4 frames: impact flash, cartoon stars circling, birds circling, fade out.
Bright colors. Playful and funny, not violent.
Children's storybook style. Transparent background. No text.
```

### 5.4 Poof Cloud (Enemy Defeat)

```
A cute poof cloud explosion for when an enemy is defeated in a children's game.
4 frames: small puff, expanding cloud with sparkles, cloud dissipating, gone.
Soft gray and white with golden sparkle accents.
Children's storybook style. Transparent background. No text.
```

---

## 6. Phase-Specific Assets (Future Phases)

### 6.1 Space Phase — Background Layer 1

```
Children's storybook illustration of a colorful outer space scene.
Purple and blue nebula. Twinkling cartoon stars. A distant friendly planet
with rings. Warm soft lighting despite being space.
Pastel space colors. No characters. No text. Seamless horizontal tile.
640x360 pixels.
```

### 6.2 Dino Phase — Background Layer 1

```
Children's storybook illustration of a prehistoric jungle.
Giant ferns, tall trees with vines, a distant smoking volcano.
Warm green and amber tones. Upper-left lighting.
Friendly and adventurous, not dark or scary.
No characters. No text. Seamless horizontal tile. 640x360 pixels.
```

### 6.3 Ocean Phase — Background Layer 1

```
Children's storybook illustration of a deep ocean scene.
Light rays filtering from above. Colorful coral reef in the distance.
Gentle blue and teal gradient. Floating bubbles.
Peaceful and magical. No characters. No text. Seamless horizontal tile.
640x360 pixels.
```

---

## 7. Co-Op UI Elements

### 7.1 "PLAYER 2" Join Banner

```
A playful banner that says "PLAYER 2" for a children's game.
Bright colors, confetti, star burst effect behind the text.
Cartoon style with thick outlines. Like a party announcement.
Transparent background. 320x80 pixels.
```

### 7.2 "TEAM UP!" Banner

```
An exciting banner that says "TEAM UP!" for a children's game co-op mode.
Both orange and teal colors. Lightning bolt and star effects.
Musical notes around it suggesting a fanfare.
Cartoon style. Transparent background. 320x80 pixels.
```

### 7.3 "DOUBLE TROUBLE" Activation Banner

```
An explosive banner for "DOUBLE TROUBLE" fusion mode in a children's game.
Gold and white glowing text. Radiating star burst.
Screen-filling excitement. Fireworks and sparkles.
Children's storybook style. Transparent background. 480x120 pixels.
```

---

## 8. API Script (Python)

Use this script to batch-generate assets via the OpenAI API:

```python
from openai import OpenAI
from pathlib import Path

client = OpenAI()  # uses OPENAI_API_KEY env var

ANCHOR = (
    "Children's storybook illustration, warm upper-left lighting, "
    "soft brush strokes, 32-color palette, 1-2px dark outlines on characters"
)

def generate(prompt: str, output_path: str, size: str = "1024x1024"):
    """Generate an image with DALL-E 3 and save it."""
    full_prompt = f"{ANCHOR}. {prompt}"
    response = client.images.generate(
        model="dall-e-3",
        prompt=full_prompt,
        size=size,
        quality="hd",
        n=1,
    )
    import urllib.request
    url = response.data[0].url
    Path(output_path).parent.mkdir(parents=True, exist_ok=True)
    urllib.request.urlretrieve(url, output_path)
    print(f"Saved: {output_path}")
    # Save the revised prompt for reference
    revised = response.data[0].revised_prompt
    with open(output_path + ".prompt.txt", "w") as f:
        f.write(f"Original: {full_prompt}\n\nRevised: {revised}\n")

# Example usage:
# generate("Character reference sheet for Loko...", "raw/loko_reference.png")
# generate("Dust bunny sprite sheet...", "raw/dust_bunny_sprites.png", "1792x1024")
```

### Recommended sizes

| Asset type | DALL-E size | Notes |
|---|---|---|
| Character reference sheets | 1024x1024 | Square, all 4 views fit |
| Sprite sheets (8 frames) | 1792x1024 | Wide, more room for frames |
| Backgrounds (parallax) | 1792x1024 | Landscape, crop to 640x360 |
| UI elements | 1024x1024 | Square, trim transparent |
| Boss characters | 1024x1024 | Square reference |

### Cost estimate (DALL-E 3 HD)

| Quality | Size | Price |
|---|---|---|
| HD | 1024x1024 | $0.080 |
| HD | 1792x1024 | $0.120 |

**Home Phase full asset set (~30 generations):** ~$3.00
**All 6 phases:** ~$18.00

---

## 9. Post-Processing Checklist

After each DALL-E generation:

- [ ] Check consistency against master palette (32 colors)
- [ ] Redraw outlines if DALL-E made them inconsistent (1-2px dark)
- [ ] Ensure warm upper-left lighting direction
- [ ] Remove any text DALL-E may have added (it sometimes does)
- [ ] Verify sprite frames are evenly spaced and aligned
- [ ] Apply phase LUT in Krita for final color grading
- [ ] Export individual frames from sprite sheets
- [ ] Run through TexturePacker for atlas generation
- [ ] Test in-engine at 640x360 base resolution
