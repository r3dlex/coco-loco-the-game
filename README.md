<p align="center">
  <img src="assets/logos/png/logo_640x360.png" alt="Coco Loco: The Game" width="480"/>
</p>

<h1 align="center">Coco Loco: The Game</h1>

<p align="center">
  <em>Two Brothers. One Wild Adventure.</em>
</p>

<p align="center">
  <a href="https://github.com/r3dlex/coco-loco-the-game/actions"><img src="https://github.com/r3dlex/coco-loco-the-game/actions/workflows/ci.yml/badge.svg" alt="CI"/></a>
  <img src="https://img.shields.io/badge/engine-Bevy%200.15-232326?logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAA4AAAAOCAYAAAAfSC3RAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAj0lEQVQoz2NgGAWkAEYGBgb+////czMwMPD8//9fgBBfcnIy4////z8A0kYPGBgYGNLS0v4D2UxAGgiAmYmJiYEYjSQBJgYqAuL8LyEhofP///+1QFfWE6txJgMDgx0DA8N8BgaGGwwMDAeAGpSBpjMwMDAIEKPxPwMDwwdKnUiKxv9ANjE+p56LRwEJAACDCUEPfPiKhAAAAABJRU5ErkJggg==&style=flat" alt="Bevy"/>
  <img src="https://img.shields.io/badge/lang-Rust-dea584?logo=rust&style=flat" alt="Rust"/>
  <img src="https://img.shields.io/badge/scripting-Rhai-4A90D9?style=flat" alt="Rhai"/>
</p>

---

A 2D side-scrolling adventure game built in **Rust** with the **Bevy** engine, designed to entertain toddlers. Play as Loko (6) and Roco (3), two superhero brothers who dash, cry sonic waves, and fuse into **Double Trouble** to defeat silly household enemies.

## Features

- **Two playable characters** — Loko (orange, speed/fury) and Roco (teal, sonic cry/hammer)
- **Local co-op** — Player 2 drops in/out anytime (keyboard or gamepad)
- **Double Trouble fusion** — Fill the star meter, merge into one powerful hero
- **Difficulty modes** — Toddler (forgiving) and Kid (challenging), driven by `DifficultyConfig`
- **Rhai scripting** — Level layouts, enemy AI, and boss choreography in script
- **6 themed phases** — Home, Space, Dino, Ocean, Candy, Dream (Home Phase first)

## Tech Stack

| Layer | Tech |
|---|---|
| Language | Rust |
| Engine | Bevy 0.15 |
| Physics | avian2d 0.2 |
| Audio | bevy_kira_audio 0.21 |
| Scripting | Rhai 1.19 (sync) |
| Art pipeline | Python + SDXL Turbo (local) or DALL-E 3 |
| CI | GitHub Actions |

## Quick Start

```bash
# Build and run the game
cargo run

# Run tests (169 unit + integration tests)
cargo test

# Check specs
cd tools/pipeline_runner && poetry install && poetry run pipeline check-all
```

## Art Generation

Game art is generated locally using **SDXL Turbo** (free, no API key needed) or optionally via DALL-E 3.

```bash
cd tools/pipeline_runner
poetry install --with flux    # install ML dependencies
poetry run pip install torch  # PyTorch (separate due to solver constraints)

# Generate all 23 art assets locally (~6s each on Apple Silicon)
poetry run generate-art --backend local

# Or generate a single asset
poetry run generate-art --backend local --only loko_reference

# Or use DALL-E 3 (requires OPENAI_API_KEY in .env)
poetry run generate-art --backend dalle
```

See [`assets/art/prompt_cookbook.md`](assets/art/prompt_cookbook.md) for all prompt definitions and manual workflow.

## Project Structure

```
src/                    # Rust game source
  main.rs               # App entry, all plugins registered
  state.rs              # GameState, FusionState, GameSystemSet
  events.rs             # All game events
  components/           # ECS components
  resources/            # Global resources (DifficultyConfig, StarMeter, CoOpState)
  plugins/              # One plugin per system
  systems/              # Shared systems (input, movement, camera, companion AI)
assets/
  scripts/              # Rhai level/enemy/phase scripts
  art/                  # Generated art assets
  logos/                # SVG + PNG brand suite
  sprites/              # Sprite atlases
  audio/                # Music + SFX
spec/                   # Spec-driven development artifacts
  features/             # Feature specs
  acceptance/           # Acceptance criteria
  schemas/              # Data contracts
tools/pipeline_runner/  # Python pipeline tooling (Poetry, zero-install)
```

## Development

This project uses **spec-driven development**. See [AGENTS.md](AGENTS.md) for the full workflow.

```bash
# Run a specific spec validation
cd tools/pipeline_runner
poetry run pipeline validate --spec co-op

# Coverage report
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```

## Base Resolution

640x360 (integer-scales to 1280x720 and 1920x1080).

## License

All rights reserved.
