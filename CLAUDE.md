# Coco Loco - The Game

A Rust game built with the Bevy engine and Rhai scripting, designed to entertain toddlers.

## Tech Stack

- **Language:** Rust
- **Game Engine:** Bevy
- **Scripting:** Rhai
- **Pipelines:** Python (via `tools/pipeline_runner`, managed with Poetry)
- **Zero-install:** All tooling is self-contained. Use `poetry install` inside `tools/pipeline_runner` to bootstrap.

## Project Layout

```
spec/                  # Spec-driven development artifacts
tools/pipeline_runner/ # Python pipeline tooling (Poetry)
src/                   # Rust game source
assets/                # Game assets (sprites, sounds, scripts)
```

## Key Commands

```bash
# Build the game
cargo build

# Run the game
cargo run

# Run tests
cargo test

# Check all specs
cd tools/pipeline_runner && poetry install && poetry run pipeline check-all

# Validate a single spec
poetry run pipeline validate --spec co-op
poetry run pipeline validate --spec double-trouble
```

## Agent Workflow

See [AGENTS.md](./AGENTS.md) for the spec-driven agent workflow, including how specs drive implementation and validation pipelines.

## Project Layout (full)

```
src/
  main.rs                  # App entry, all plugins registered
  state.rs                 # GameState, FusionState, GameSystemSet
  events.rs                # All game events
  components/              # ECS components (character, movement, abilities, star_meter)
  resources/               # Global resources (difficulty, co_op, star_meter)
  plugins/                 # One plugin per system (core, character, companion, co_op,
  |                        #   fusion, side_scroller, enemy, scripting, art, audio,
  |                        #   ui, guidance, save)
  systems/                 # Shared systems (input, movement, camera, companion_ai)
assets/
  scripts/
    levels/home/           # Rhai level scripts
    enemies/               # Rhai enemy scripts (dust_bunny, boss_living_room)
    phases/                # Rhai phase configs (home)
  audio/                   # Music stems + SFX (ogg)
  sprites/                 # Sprite atlases
  luts/                    # Color LUT textures for phase post-processing
spec/
  requirements.md          # Product requirements
  architecture.md          # System architecture
  features/                # Feature specs (input-feedback, scene-management,
  |                        #   co-op, difficulty-modes, double-trouble)
  schemas/                 # Data contracts (scene-manifest)
  acceptance/              # Acceptance criteria (one per feature)
tools/pipeline_runner/     # Python/Poetry pipeline tooling
```

## Conventions

- Specs are the source of truth. Code follows specs, not the other way around.
- Every user-facing feature starts as a spec in `spec/features/`, acceptance in `spec/acceptance/`.
- All game logic configurable via `DifficultyConfig` — no hardcoded mode checks in systems.
- Rhai scripts control level layout, enemy behaviour, boss choreography. Rust controls physics and rendering.
- Pipeline scripts live in `tools/pipeline_runner` and are invoked via `poetry run`.
- Base resolution: 640×360 (integer-scales to 1280×720 and 1920×1080).
