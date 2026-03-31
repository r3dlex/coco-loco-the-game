use coco_loco::resources::difficulty::*;
use coco_loco::resources::star_meter::StarMeter;

/// Spec: section 3.2 — Toddler mode must produce a 5-star meter.
#[test]
fn toddler_config_produces_correct_meter_size() {
    let config = DifficultyConfig::toddler();
    let meter = StarMeter::new(config.fusion_meter_size);
    assert_eq!(meter.max, 5);
}

/// Spec: section 3.3 — Kid mode must produce a 10-star meter.
#[test]
fn kid_config_produces_correct_meter_size() {
    let config = DifficultyConfig::kid();
    let meter = StarMeter::new(config.fusion_meter_size);
    assert_eq!(meter.max, 10);
}

/// Spec: section 3.4 — all mode-dependent behaviour flows from DifficultyConfig.
#[test]
fn toddler_and_kid_modes_differ_on_all_key_fields() {
    let t = DifficultyConfig::toddler();
    let k = DifficultyConfig::kid();

    assert_ne!(t.mode, k.mode);
    assert_ne!(t.coyote_time, k.coyote_time);
    assert_ne!(t.fusion_trigger, k.fusion_trigger);
    assert_ne!(t.allow_character_switching, k.allow_character_switching);
    assert_ne!(t.show_cooldowns, k.show_cooldowns);
    assert_ne!(t.p2_invulnerable, k.p2_invulnerable);
    assert_ne!(t.fusion_meter_size, k.fusion_meter_size);
    assert_ne!(t.auto_guidance_delay, k.auto_guidance_delay);
}

/// Spec: section 2.1 — Toddler: every button press does something.
/// Reflected in simplified control scheme.
#[test]
fn toddler_p2_simplified_controls() {
    let t = DifficultyConfig::toddler();
    assert_eq!(t.p2_control_scheme, ControlScheme::Simplified);
}

/// Spec: section 3.3 — Kid: full controls for P2.
#[test]
fn kid_p2_full_controls() {
    let k = DifficultyConfig::kid();
    assert_eq!(k.p2_control_scheme, ControlScheme::Full);
}
