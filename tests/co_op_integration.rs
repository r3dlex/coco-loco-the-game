use coco_loco::resources::co_op::*;
use bevy::prelude::*;

/// Spec: section 4.3 — P2 joins: no pause, AI disengages.
#[test]
fn p2_join_activates_co_op() {
    let mut state = CoOpState::default();
    assert!(!state.active);

    state.join(InputDevice::Keyboard { layout: KeyboardLayout::Arrows });
    assert!(state.active);
    assert_eq!(state.current_tier, CoOpTier::CompanionTakeover);
}

/// Spec: section 4.3 — P2 leaves: AI re-engages.
#[test]
fn p2_leave_deactivates_and_resets_tier() {
    let mut state = CoOpState::default();
    state.join(InputDevice::Keyboard { layout: KeyboardLayout::Arrows });
    state.set_tier(CoOpTier::BossTeamUp);
    state.leave();

    assert!(!state.active);
    assert_eq!(state.current_tier, CoOpTier::CompanionTakeover);
    assert_eq!(state.p2_input_device, None);
}

/// Spec: section 4.2 — Tier 2 escalation.
#[test]
fn tier_escalation_team_up() {
    let mut state = CoOpState::default();
    state.join(InputDevice::Keyboard { layout: KeyboardLayout::Arrows });
    state.set_tier(CoOpTier::TeamUp);
    assert_eq!(state.current_tier, CoOpTier::TeamUp);
}

/// Spec: section 4.2 — Tier 3 boss team-up.
#[test]
fn tier_escalation_boss() {
    let mut state = CoOpState::default();
    state.join(InputDevice::Keyboard { layout: KeyboardLayout::Wasd });
    state.set_tier(CoOpTier::BossTeamUp);
    assert_eq!(state.current_tier, CoOpTier::BossTeamUp);
}

/// Spec: section 4.5 — Multiple input devices supported.
#[test]
fn gamepad_input_device() {
    let mut state = CoOpState::default();
    let fake_entity = Entity::from_raw(42);
    state.join(InputDevice::Gamepad(fake_entity));
    assert_eq!(state.p2_input_device, Some(InputDevice::Gamepad(fake_entity)));
}

/// Edge case: join-leave-join with different device.
#[test]
fn rejoin_with_different_device() {
    let mut state = CoOpState::default();
    state.join(InputDevice::Keyboard { layout: KeyboardLayout::Arrows });
    state.leave();
    state.join(InputDevice::Keyboard { layout: KeyboardLayout::Wasd });
    assert!(state.active);
    assert_eq!(
        state.p2_input_device,
        Some(InputDevice::Keyboard { layout: KeyboardLayout::Wasd })
    );
}
