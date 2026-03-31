# Architecture

## System Overview

```
┌──────────────────────────────────────────┐
│               Bevy App                   │
│  ┌────────────┐  ┌───────────────────┐   │
│  │  Core ECS  │  │  Rhai Script VM   │   │
│  │  Systems   │◄─┤  (scene logic)    │   │
│  └─────┬──────┘  └───────────────────┘   │
│        │                                 │
│  ┌─────▼──────┐  ┌───────────────────┐   │
│  │  Rendering │  │  Audio Feedback   │   │
│  │  Pipeline  │  │  System           │   │
│  └────────────┘  └───────────────────┘   │
└──────────────────────────────────────────┘
```

## Modules

### `core` — Application bootstrap and ECS setup
- Initialises the Bevy app with default plugins.
- Registers all game systems and resources.

### `scenes` — Scene management
- Loads and unloads scenes based on Rhai scripts.
- Each scene is a Rhai script that declares entities, components, and behaviour.

### `input` — Input handling
- Captures all keyboard, mouse, and touch events.
- Forwards input events to the active scene's Rhai context.

### `feedback` — Audio and visual feedback
- Plays sounds and triggers animations in response to input.
- Feedback mappings are defined per-scene in Rhai.

### `scripting` — Rhai integration
- Embeds the Rhai engine.
- Exposes a safe API surface to scripts (spawn entities, play sounds, animate).
- Scripts live in `assets/scripts/`.

## Data Flow

1. User input captured by `input` system.
2. Input event dispatched to active scene's Rhai script.
3. Script calls engine API (spawn, animate, play_sound).
4. `feedback` and `rendering` systems process the resulting ECS changes.

## Boundaries

- Rhai scripts MUST NOT access the filesystem or network.
- All asset references in scripts use logical names resolved by the asset loader.
- Scene transitions are managed by the `scenes` module, not by scripts.
