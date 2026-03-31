# Feature: Difficulty Modes

## Summary

Two modes selectable on first launch via big icons (no text). Changeable from pause screen.

## Toddler Mode (ages 3-5)

- **Controls:** 3 inputs. All face buttons = jump. All shoulders = action.
- **Switching:** Disabled. Level pre-assigns character.
- **Fusion:** Automatic on full meter (5 stars).
- **Enemies:** Slow (0.6× speed), 2s wind-up, 1 star damage per hit.
- **Bosses:** Sit down every 15 seconds. No enrage.
- **Platforming:** Wide. Coyote time: 300ms.
- **No death:** Float from pits. Stars scatter nearby (80px radius, 1 star lost).
- **HUD:** Star meter only (5 icons).
- **Guidance:** Trail at 10s idle. Companion walks at 20s.
- **Co-op P2:** Simplified, invulnerable, 50% damage.

## Kid Mode (ages 5-8+)

- **Controls:** Full. Dedicated jump, action, special, switch.
- **Switching:** Tab (0.5s cooldown). Disabled in co-op.
- **Fusion:** Manual. P1 presses F key.
- **Fusion cooldown:** 45 seconds.
- **Enemies:** Full speed, 0.75s wind-up, 2-3 stars damage per hit.
- **Bosses:** Phase transitions at 50% health. 1.3× attack speed in phase 2.
- **Platforming:** Tighter. Coyote time: 100ms.
- **No death:** Stars scatter farther (160px radius, 3 stars lost).
- **HUD:** Stars, cooldown indicators, portraits.
- **Co-op P2:** Full controls, vulnerable, 100% damage.

## Implementation

`DifficultyConfig` resource initialized from the selected mode. All gameplay systems read from it at runtime.
