# Feature: Scene Management

## Summary

The game is composed of independent scenes. Each scene is defined by a Rhai script and its associated assets. Parents can switch scenes; toddlers interact within the active scene.

## Behaviour

- On startup, the game loads a default scene (configurable).
- Parent controls (left/right arrow keys) cycle through available scenes.
- Scene transitions include a brief crossfade animation.
- Each scene's Rhai script is loaded fresh on entry (no stale state).
- Scenes declare their required assets; the engine preloads them during transition.

## Scene Definition

A scene is a directory under `assets/scenes/<scene-name>/` containing:
- `scene.rhai` — scene logic
- `assets.toml` — asset manifest (sprites, sounds)

## Constraints

- Scene transitions must complete in < 500ms.
- If a scene's script fails to load, fall back to the default scene with a log warning.
- Maximum memory per scene: defined by available assets, but scripts must not allocate unbounded data.
