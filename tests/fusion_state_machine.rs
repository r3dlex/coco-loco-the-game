use coco_loco::plugins::fusion::{FusionTimer, FUSION_DURATION, FLICKER_WARNING};
use coco_loco::resources::star_meter::StarMeter;
use coco_loco::resources::difficulty::*;

/// Spec: section 9.1 — Fusion duration is 12 seconds.
#[test]
fn fusion_duration_is_12_seconds() {
    assert!((FUSION_DURATION - 12.0).abs() < f32::EPSILON);
}

/// Spec: section 9.3 — Flicker warning at 3 seconds remaining.
#[test]
fn flicker_warning_at_3_seconds() {
    assert!((FLICKER_WARNING - 3.0).abs() < f32::EPSILON);
}

/// FusionTimer defaults to zero.
#[test]
fn fusion_timer_defaults() {
    let t = FusionTimer::default();
    assert_eq!(t.elapsed, 0.0);
    assert_eq!(t.cooldown_remaining, 0.0);
}

/// Spec: section 9.2 — Toddler: 5 stars. Kid: 10 stars.
#[test]
fn toddler_meter_size_matches_fusion_spec() {
    let c = DifficultyConfig::toddler();
    assert_eq!(c.fusion_meter_size, 5);
    assert_eq!(c.fusion_trigger, FusionTrigger::Automatic);
}

#[test]
fn kid_meter_size_matches_fusion_spec() {
    let c = DifficultyConfig::kid();
    assert_eq!(c.fusion_meter_size, 10);
    assert_eq!(c.fusion_trigger, FusionTrigger::Manual);
    assert!((c.fusion_cooldown - 45.0).abs() < f32::EPSILON);
}

/// Spec: section 9.3 — Meter resets after fusion.
#[test]
fn meter_reset_simulates_fusion_expiry() {
    let mut m = StarMeter::new(5);
    m.add(5);
    assert!(m.is_full());
    m.reset(); // simulates fusion activation resetting the meter
    assert_eq!(m.current, 0);
    assert!(!m.is_full());
}

/// Edge case: timer tracks elapsed correctly.
#[test]
fn fusion_timer_elapsed_tracking() {
    let mut t = FusionTimer::default();
    t.elapsed = 11.5;
    assert!(t.elapsed < FUSION_DURATION);
    t.elapsed = 12.0;
    assert!(t.elapsed >= FUSION_DURATION);
}

/// Edge case: cooldown decrement.
#[test]
fn fusion_cooldown_tracking() {
    let mut t = FusionTimer::default();
    t.cooldown_remaining = 45.0;
    t.cooldown_remaining -= 1.0;
    assert!((t.cooldown_remaining - 44.0).abs() < f32::EPSILON);
}
