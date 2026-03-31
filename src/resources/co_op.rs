use bevy::prelude::*;

/// Which tier of co-op cooperation is active right now.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum CoOpTier {
    /// P2 controls companion brother. Companion AI disabled.
    #[default]
    CompanionTakeover,
    /// Scripted zone escalation. Both players at full power.
    TeamUp,
    /// Always active during boss fights when P2 is present.
    BossTeamUp,
}

/// Input device type for auto-detection on P2 join.
///
/// In Bevy 0.15, gamepads are entities. We store the entity id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputDevice {
    Gamepad(Entity),
    Keyboard { layout: KeyboardLayout },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyboardLayout {
    /// Arrow keys + right-side keys
    Arrows,
    /// WASD + left-side keys
    Wasd,
}

/// Global co-op state resource.
#[derive(Resource, Default)]
pub struct CoOpState {
    pub active: bool,
    pub p2_input_device: Option<InputDevice>,
    pub current_tier: CoOpTier,
}

impl CoOpState {
    pub fn join(&mut self, device: InputDevice) {
        self.active = true;
        self.p2_input_device = Some(device);
        self.current_tier = CoOpTier::CompanionTakeover;
    }

    pub fn leave(&mut self) {
        self.active = false;
        self.p2_input_device = None;
        self.current_tier = CoOpTier::default();
    }

    pub fn set_tier(&mut self, tier: CoOpTier) {
        self.current_tier = tier;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_inactive() {
        let s = CoOpState::default();
        assert!(!s.active);
        assert_eq!(s.p2_input_device, None);
        assert_eq!(s.current_tier, CoOpTier::CompanionTakeover);
    }

    #[test]
    fn join_activates_and_sets_device() {
        let mut s = CoOpState::default();
        let device = InputDevice::Keyboard { layout: KeyboardLayout::Arrows };
        s.join(device.clone());
        assert!(s.active);
        assert_eq!(s.p2_input_device, Some(device));
        assert_eq!(s.current_tier, CoOpTier::CompanionTakeover);
    }

    #[test]
    fn leave_deactivates_and_clears_device() {
        let mut s = CoOpState::default();
        s.join(InputDevice::Keyboard { layout: KeyboardLayout::Arrows });
        s.leave();
        assert!(!s.active);
        assert_eq!(s.p2_input_device, None);
        assert_eq!(s.current_tier, CoOpTier::CompanionTakeover);
    }

    #[test]
    fn set_tier_changes_tier() {
        let mut s = CoOpState::default();
        s.set_tier(CoOpTier::TeamUp);
        assert_eq!(s.current_tier, CoOpTier::TeamUp);
        s.set_tier(CoOpTier::BossTeamUp);
        assert_eq!(s.current_tier, CoOpTier::BossTeamUp);
    }

    #[test]
    fn join_resets_tier_to_companion_takeover() {
        let mut s = CoOpState::default();
        s.set_tier(CoOpTier::TeamUp);
        s.join(InputDevice::Keyboard { layout: KeyboardLayout::Wasd });
        assert_eq!(s.current_tier, CoOpTier::CompanionTakeover);
    }

    #[test]
    fn leave_after_tier_escalation_resets() {
        let mut s = CoOpState::default();
        s.join(InputDevice::Keyboard { layout: KeyboardLayout::Arrows });
        s.set_tier(CoOpTier::BossTeamUp);
        s.leave();
        assert_eq!(s.current_tier, CoOpTier::CompanionTakeover);
    }

    #[test]
    fn join_leave_join_cycle() {
        let mut s = CoOpState::default();
        s.join(InputDevice::Keyboard { layout: KeyboardLayout::Arrows });
        assert!(s.active);
        s.leave();
        assert!(!s.active);
        s.join(InputDevice::Keyboard { layout: KeyboardLayout::Wasd });
        assert!(s.active);
        assert_eq!(
            s.p2_input_device,
            Some(InputDevice::Keyboard { layout: KeyboardLayout::Wasd })
        );
    }
}
