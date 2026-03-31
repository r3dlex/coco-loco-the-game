use coco_loco::components::abilities::*;
use coco_loco::events::AbilityId;

/// Spec: section 8.2 — Loko Kid: Punch (no cd), Dash (3s), Fury (30s).
#[test]
fn loko_kid_ability_cooldowns_match_spec() {
    let a = loko_kid_abilities();
    let punch = &a.list[0];
    let dash = &a.list[1];
    let fury = &a.list[2];

    assert_eq!(punch.cooldown, None);
    assert_eq!(dash.cooldown, Some(3.0));
    assert_eq!(fury.cooldown, Some(30.0));
}

/// Spec: section 8.3 — Roco Kid: Cry (5s), SmartVision (none), Mesmerize (15s), Hammer (25s).
#[test]
fn roco_kid_ability_cooldowns_match_spec() {
    let a = roco_kid_abilities();
    assert_eq!(a.list[0].cooldown, Some(5.0));
    assert_eq!(a.list[1].cooldown, None);
    assert_eq!(a.list[2].cooldown, Some(15.0));
    assert_eq!(a.list[3].cooldown, Some(25.0));
}

/// Spec: section 3.2 — Toddler: no cooldowns.
#[test]
fn toddler_abilities_have_no_cooldowns() {
    for abilities in [loko_toddler_abilities(), roco_toddler_abilities()] {
        for ability in &abilities.list {
            assert_eq!(
                ability.cooldown, None,
                "Toddler ability {:?} should have no cooldown",
                ability.id
            );
        }
    }
}

/// Simulates 60 FPS tick for cooldown recovery.
#[test]
fn ability_cooldown_recovers_over_frames() {
    let mut a = Ability::new(AbilityId::Dash, Some(3.0));
    a.trigger();

    let dt = 1.0 / 60.0;
    // 181 ticks to account for FP accumulation (180 * 1/60 < 3.0 due to rounding)
    for _ in 0..181 {
        a.tick(dt);
    }
    assert!(a.is_ready(), "Dash should be ready after ~3 seconds of ticks");
}

/// Spec: contextual action means no wrong button presses.
/// Reflected: Toddler punch has no cooldown, can be spammed.
#[test]
fn toddler_punch_can_be_spammed() {
    let mut a = Ability::new(AbilityId::Punch, None);
    for _ in 0..100 {
        a.trigger();
        assert!(a.is_ready());
    }
}
