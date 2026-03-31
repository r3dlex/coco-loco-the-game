# Schema: Scene Asset Manifest (`assets.toml`)

## Format

```toml
[scene]
name = "Ocean Fun"
description = "Colourful sea creatures react to input"

[[sprites]]
name = "fish"
path = "sprites/fish.png"

[[sprites]]
name = "octopus"
path = "sprites/octopus.png"

[[sounds]]
name = "pop"
path = "sounds/pop.ogg"

[[sounds]]
name = "splash"
path = "sounds/splash.ogg"
```

## Fields

| Field                | Type   | Required | Description                         |
|----------------------|--------|----------|-------------------------------------|
| `scene.name`         | string | yes      | Human-readable scene name           |
| `scene.description`  | string | no       | Short description for parent UI     |
| `sprites[].name`     | string | yes      | Logical name referenced in Rhai     |
| `sprites[].path`     | string | yes      | Path relative to scene directory    |
| `sounds[].name`      | string | yes      | Logical name referenced in Rhai     |
| `sounds[].path`      | string | yes      | Path relative to scene directory    |

## Constraints

- All paths are relative to the scene directory.
- Sprite formats: PNG, WebP.
- Sound formats: OGG, WAV.
- Names must be unique within their category (sprites, sounds).
