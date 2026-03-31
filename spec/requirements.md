# Product Requirements

## Overview

Coco Loco is an interactive game designed for toddlers (ages 1-4). It runs on desktop and uses simple, colourful visuals with responsive audio feedback.

## Core Constraints

- **Audience:** Toddlers. Every interaction must be safe, simple, and rewarding.
- **Input:** Keyboard mashing, mouse clicks, touchscreen taps. All inputs should produce positive feedback — there are no wrong moves.
- **Performance:** Must run at 60 FPS on modest hardware. Toddlers won't wait for loading screens.
- **Safety:** No text input, no network access, no in-app purchases, no ads.
- **Scripting:** Game logic and scene behaviour are defined in Rhai scripts under `assets/scripts/`. This allows rapid iteration without recompiling.

## Functional Requirements

1. The game presents interactive scenes with colourful objects.
2. Any input (key press, click, tap) triggers a visual and audio response.
3. Scenes can be swapped by parents via a simple mechanism (e.g., arrow keys, menu).
4. Each scene is self-contained and defined by a Rhai script + associated assets.

## Non-Functional Requirements

- Zero-install: clone and `cargo run`.
- Cross-platform: Linux, macOS, Windows.
- Accessible: high-contrast colours, large touch targets.
