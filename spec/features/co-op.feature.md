# Feature: Local Co-Op

## Summary

A second player can join at any time to control the companion brother. Three tiers of co-op escalate from companion takeover to full team-up moments and boss fights.

## Tiers

### Tier 1: Companion Takeover (default)
P2 joins by pressing Start. No menu, no pause. CompanionAI is removed; Player2 component is added to the companion entity.
- Toddler: simplified 3-button controls, P2 invulnerable, 50% damage.
- Kid: full controls, P2 vulnerable, 100% damage.

### Tier 2: Team-Up Promotion (scripted)
Level Rhai scripts call `level.add_team_up_zone(...)`. When both players enter the zone, the tier escalates. A "TEAM UP!" banner plays. Both players at full power.

### Tier 3: Boss Team-Up
Always active when P2 is present during a boss fight. Boss health scaled up.

## Drop-In / Drop-Out

- P2 presses Start on any device to join. No pause.
- P2 presses Start again to leave. Companion AI resumes.
- Disconnect: AI takes over after 5 seconds.

## Shared Star Meter

Both players collect into one shared meter. No competition.

## Character Switching

Disabled in co-op. Each player permanently owns one brother.

## Constraints

- All co-op configurations must be tested: Toddler+simplified P2, Toddler+none, Kid+full P2, Kid+none.
- P2 join/leave must not pause or interrupt gameplay.
- Mid-fusion join: P1 moves, P2 attacks.
