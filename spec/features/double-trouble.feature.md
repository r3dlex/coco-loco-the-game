# Feature: Double Trouble (Fusion Mode)

## Summary

When the shared star meter fills, both brothers merge into an invincible, overpowered fused form for 12 seconds.

## State Machine

```
Normal → [full meter + trigger] → Activating → [animation done] → Fused → [timer] → Cooldown → [done] → Normal
```

## Trigger

- **Toddler:** Automatic on `StarMeterFullEvent`.
- **Kid:** Manual. P1 presses F key after meter is full.

## Fused Form

- Both brothers hidden; DoubleTrouble entity spawned.
- 1.5× abilities, invincible, 2× speed.
- Duration: 12 seconds.
- At 3 seconds remaining: form flickers.
- At 0 seconds: brothers pop apart with a silly animation, meter resets.

## Co-Op Fusion

- P1 controls movement. P2 controls attacks.
- P2 joining mid-fusion: immediately gets attack control.
- P2 leaving mid-fusion: P1 gets full control.

## Cooldown

- **Toddler:** No timer. Refill the meter to fuse again.
- **Kid:** 45-second cooldown after expiry.

## Star Meter

- Toddler: 5 stars. Kid: 10 stars.
- Both players collect into the same shared meter.
- Stars scattered by hits are re-collectable by either player.
