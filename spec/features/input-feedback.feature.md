# Feature: Input Feedback

## Summary

Every user input (key press, mouse click, touch tap) produces an immediate, positive visual and audio response. There are no wrong inputs.

## Behaviour

- **Any key press** triggers a visual effect at a random position on screen and plays a sound.
- **Mouse click / tap** triggers a visual effect at the click/tap position and plays a sound.
- Effects and sounds are defined per-scene in the scene's Rhai script.
- If no scene-specific mapping exists, a default global feedback is used.

## Rhai API

```rhai
// Called by the engine on any key press
fn on_key_press(key) {
    spawn_effect("sparkle", random_position());
    play_sound("pop");
}

// Called by the engine on mouse click / tap
fn on_click(x, y) {
    spawn_effect("burst", position(x, y));
    play_sound("boing");
}
```

## Constraints

- Feedback latency must be < 50ms from input to first visual frame.
- Concurrent inputs must each produce their own independent feedback.
- Sound playback must not block or delay visual feedback.
