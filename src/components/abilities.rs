use bevy::prelude::*;
use crate::events::AbilityId;

/// All abilities available to a character.
#[derive(Component, Default)]
pub struct Abilities {
    pub list: Vec<Ability>,
}

#[derive(Debug, Clone)]
pub struct Ability {
    pub id: AbilityId,
    /// None = no cooldown.
    pub cooldown: Option<f32>,
    /// Remaining cooldown in seconds (0.0 = ready).
    pub remaining: f32,
}

impl Ability {
    pub fn new(id: AbilityId, cooldown_secs: Option<f32>) -> Self {
        Self { id, cooldown: cooldown_secs, remaining: 0.0 }
    }

    pub fn is_ready(&self) -> bool {
        self.remaining <= 0.0
    }

    pub fn trigger(&mut self) {
        if let Some(cd) = self.cooldown {
            self.remaining = cd;
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.remaining = (self.remaining - dt).max(0.0);
    }
}

// ── Loko abilities ─────────────────────────────────────────────────────────────

pub fn loko_toddler_abilities() -> Abilities {
    Abilities {
        list: vec![
            Ability::new(AbilityId::Punch, None),
        ],
    }
}

pub fn loko_kid_abilities() -> Abilities {
    Abilities {
        list: vec![
            Ability::new(AbilityId::Punch, None),
            Ability::new(AbilityId::Dash, Some(3.0)),
            Ability::new(AbilityId::DevastatungFury, Some(30.0)),
        ],
    }
}

// ── Roco abilities ─────────────────────────────────────────────────────────────

pub fn roco_toddler_abilities() -> Abilities {
    Abilities {
        list: vec![
            Ability::new(AbilityId::Cry, None),
            Ability::new(AbilityId::SmartVision, None),
        ],
    }
}

pub fn roco_kid_abilities() -> Abilities {
    Abilities {
        list: vec![
            Ability::new(AbilityId::Cry, Some(5.0)),
            Ability::new(AbilityId::SmartVision, None),
            Ability::new(AbilityId::Mesmerize, Some(15.0)),
            Ability::new(AbilityId::MightyHammer, Some(25.0)),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Ability core ──────────────────────────────────────────────────────────

    #[test]
    fn new_ability_starts_ready() {
        let a = Ability::new(AbilityId::Punch, Some(5.0));
        assert!(a.is_ready());
        assert_eq!(a.remaining, 0.0);
    }

    #[test]
    fn trigger_sets_cooldown() {
        let mut a = Ability::new(AbilityId::Dash, Some(3.0));
        a.trigger();
        assert!(!a.is_ready());
        assert!((a.remaining - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn trigger_no_cooldown_ability_stays_ready() {
        let mut a = Ability::new(AbilityId::Punch, None);
        a.trigger();
        assert!(a.is_ready());
        assert_eq!(a.remaining, 0.0);
    }

    #[test]
    fn tick_reduces_remaining() {
        let mut a = Ability::new(AbilityId::Dash, Some(3.0));
        a.trigger();
        a.tick(1.0);
        assert!((a.remaining - 2.0).abs() < f32::EPSILON);
        assert!(!a.is_ready());
    }

    #[test]
    fn tick_does_not_go_below_zero() {
        let mut a = Ability::new(AbilityId::Dash, Some(1.0));
        a.trigger();
        a.tick(5.0);
        assert_eq!(a.remaining, 0.0);
        assert!(a.is_ready());
    }

    #[test]
    fn tick_with_zero_dt() {
        let mut a = Ability::new(AbilityId::Dash, Some(3.0));
        a.trigger();
        a.tick(0.0);
        assert!((a.remaining - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn trigger_tick_trigger_cycle() {
        let mut a = Ability::new(AbilityId::DevastatungFury, Some(30.0));
        a.trigger();
        assert!(!a.is_ready());
        a.tick(30.0);
        assert!(a.is_ready());
        a.trigger();
        assert!(!a.is_ready());
        assert!((a.remaining - 30.0).abs() < f32::EPSILON);
    }

    // ── Factory functions ─────────────────────────────────────────────────────

    #[test]
    fn loko_toddler_has_one_ability() {
        let a = loko_toddler_abilities();
        assert_eq!(a.list.len(), 1);
        assert_eq!(a.list[0].id, AbilityId::Punch);
        assert_eq!(a.list[0].cooldown, None);
    }

    #[test]
    fn loko_kid_has_three_abilities() {
        let a = loko_kid_abilities();
        assert_eq!(a.list.len(), 3);
        assert_eq!(a.list[0].id, AbilityId::Punch);
        assert_eq!(a.list[1].id, AbilityId::Dash);
        assert_eq!(a.list[1].cooldown, Some(3.0));
        assert_eq!(a.list[2].id, AbilityId::DevastatungFury);
        assert_eq!(a.list[2].cooldown, Some(30.0));
    }

    #[test]
    fn roco_toddler_has_two_abilities() {
        let a = roco_toddler_abilities();
        assert_eq!(a.list.len(), 2);
        assert_eq!(a.list[0].id, AbilityId::Cry);
        assert_eq!(a.list[0].cooldown, None);
        assert_eq!(a.list[1].id, AbilityId::SmartVision);
    }

    #[test]
    fn roco_kid_has_four_abilities() {
        let a = roco_kid_abilities();
        assert_eq!(a.list.len(), 4);
        assert_eq!(a.list[0].id, AbilityId::Cry);
        assert_eq!(a.list[0].cooldown, Some(5.0));
        assert_eq!(a.list[1].id, AbilityId::SmartVision);
        assert_eq!(a.list[1].cooldown, None);
        assert_eq!(a.list[2].id, AbilityId::Mesmerize);
        assert_eq!(a.list[2].cooldown, Some(15.0));
        assert_eq!(a.list[3].id, AbilityId::MightyHammer);
        assert_eq!(a.list[3].cooldown, Some(25.0));
    }

    #[test]
    fn all_factory_abilities_start_ready() {
        for abilities in [
            loko_toddler_abilities(),
            loko_kid_abilities(),
            roco_toddler_abilities(),
            roco_kid_abilities(),
        ] {
            for ability in &abilities.list {
                assert!(ability.is_ready(), "Ability {:?} should start ready", ability.id);
            }
        }
    }
}
