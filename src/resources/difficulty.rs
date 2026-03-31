use bevy::prelude::*;

/// Selects difficulty mode — set once on first launch, changeable from pause.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DifficultyMode {
    /// Ages 3-5. No death, simplified controls, auto-fusion.
    #[default]
    Toddler,
    /// Ages 5-8+. Full controls, manual fusion, tighter platforming.
    Kid,
}

/// How fusion is triggered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FusionTrigger {
    /// Activates automatically when the meter is full (Toddler).
    Automatic,
    /// P1 must press the dedicated button (Kid).
    Manual,
}

/// Input simplification for P2 in co-op.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlScheme {
    /// 3-button: move, jump, action. All face = jump, all shoulder = action.
    Simplified,
    /// Full: jump, action, special, switch. With cooldown indicators.
    Full,
}

/// All runtime difficulty parameters derived from the selected mode.
#[derive(Resource)]
pub struct DifficultyConfig {
    pub mode: DifficultyMode,
    /// How long after leaving a ledge a jump is still allowed (seconds).
    pub coyote_time: f32,
    pub enemy_speed_multiplier: f32,
    pub enemy_damage_stars: u32,
    /// Enemy attack wind-up duration before the hit lands (seconds).
    pub enemy_windup_duration: f32,
    /// How often the boss sits down for free hits (seconds; 0.0 = never rests).
    pub boss_rest_interval: f32,
    pub fusion_trigger: FusionTrigger,
    pub show_cooldowns: bool,
    pub allow_character_switching: bool,
    /// If Some, shows auto-guidance trail after this many seconds of idle.
    pub auto_guidance_delay: Option<f32>,
    /// Radius in which scattered stars land after a hit.
    pub star_scatter_radius: f32,
    pub stars_lost_per_hit: u32,
    pub p2_invulnerable: bool,
    pub p2_damage_multiplier: f32,
    pub p2_control_scheme: ControlScheme,
    /// Number of stars required to fill the fusion meter.
    pub fusion_meter_size: u32,
    /// Fusion cooldown after expiry (seconds). Ignored when trigger is Automatic.
    pub fusion_cooldown: f32,
}

impl DifficultyConfig {
    pub fn toddler() -> Self {
        Self {
            mode: DifficultyMode::Toddler,
            coyote_time: 0.3,
            enemy_speed_multiplier: 0.6,
            enemy_damage_stars: 1,
            enemy_windup_duration: 2.0,
            boss_rest_interval: 15.0,
            fusion_trigger: FusionTrigger::Automatic,
            show_cooldowns: false,
            allow_character_switching: false,
            auto_guidance_delay: Some(10.0),
            star_scatter_radius: 80.0,
            stars_lost_per_hit: 1,
            p2_invulnerable: true,
            p2_damage_multiplier: 0.5,
            p2_control_scheme: ControlScheme::Simplified,
            fusion_meter_size: 5,
            fusion_cooldown: 0.0,
        }
    }

    pub fn kid() -> Self {
        Self {
            mode: DifficultyMode::Kid,
            coyote_time: 0.1,
            enemy_speed_multiplier: 1.0,
            enemy_damage_stars: 2,
            enemy_windup_duration: 0.75,
            boss_rest_interval: 0.0,
            fusion_trigger: FusionTrigger::Manual,
            show_cooldowns: true,
            allow_character_switching: true,
            auto_guidance_delay: None,
            star_scatter_radius: 160.0,
            stars_lost_per_hit: 3,
            p2_invulnerable: false,
            p2_damage_multiplier: 1.0,
            p2_control_scheme: ControlScheme::Full,
            fusion_meter_size: 10,
            fusion_cooldown: 45.0,
        }
    }
}

impl Default for DifficultyConfig {
    fn default() -> Self {
        Self::toddler()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Toddler Mode (spec: section 3.2) ─────────────────────────────────────

    #[test]
    fn toddler_mode_defaults() {
        let c = DifficultyConfig::toddler();
        assert_eq!(c.mode, DifficultyMode::Toddler);
    }

    #[test]
    fn toddler_coyote_time_300ms() {
        let c = DifficultyConfig::toddler();
        assert!((c.coyote_time - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn toddler_auto_fusion() {
        let c = DifficultyConfig::toddler();
        assert_eq!(c.fusion_trigger, FusionTrigger::Automatic);
        assert_eq!(c.fusion_meter_size, 5);
    }

    #[test]
    fn toddler_no_character_switching() {
        let c = DifficultyConfig::toddler();
        assert!(!c.allow_character_switching);
    }

    #[test]
    fn toddler_no_cooldowns_shown() {
        let c = DifficultyConfig::toddler();
        assert!(!c.show_cooldowns);
    }

    #[test]
    fn toddler_enemies_slow() {
        let c = DifficultyConfig::toddler();
        assert!((c.enemy_speed_multiplier - 0.6).abs() < f32::EPSILON);
        assert!((c.enemy_windup_duration - 2.0).abs() < f32::EPSILON);
        assert_eq!(c.enemy_damage_stars, 1);
    }

    #[test]
    fn toddler_boss_rests_every_15s() {
        let c = DifficultyConfig::toddler();
        assert!((c.boss_rest_interval - 15.0).abs() < f32::EPSILON);
    }

    #[test]
    fn toddler_guidance_at_10s() {
        let c = DifficultyConfig::toddler();
        assert_eq!(c.auto_guidance_delay, Some(10.0));
    }

    #[test]
    fn toddler_p2_invulnerable_50pct_damage() {
        let c = DifficultyConfig::toddler();
        assert!(c.p2_invulnerable);
        assert!((c.p2_damage_multiplier - 0.5).abs() < f32::EPSILON);
        assert_eq!(c.p2_control_scheme, ControlScheme::Simplified);
    }

    #[test]
    fn toddler_star_scatter_nearby() {
        let c = DifficultyConfig::toddler();
        assert!((c.star_scatter_radius - 80.0).abs() < f32::EPSILON);
        assert_eq!(c.stars_lost_per_hit, 1);
    }

    // ── Kid Mode (spec: section 3.3) ──────────────────────────────────────────

    #[test]
    fn kid_mode_defaults() {
        let c = DifficultyConfig::kid();
        assert_eq!(c.mode, DifficultyMode::Kid);
    }

    #[test]
    fn kid_coyote_time_100ms() {
        let c = DifficultyConfig::kid();
        assert!((c.coyote_time - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn kid_manual_fusion_10_stars() {
        let c = DifficultyConfig::kid();
        assert_eq!(c.fusion_trigger, FusionTrigger::Manual);
        assert_eq!(c.fusion_meter_size, 10);
        assert!((c.fusion_cooldown - 45.0).abs() < f32::EPSILON);
    }

    #[test]
    fn kid_character_switching_allowed() {
        let c = DifficultyConfig::kid();
        assert!(c.allow_character_switching);
    }

    #[test]
    fn kid_cooldowns_shown() {
        let c = DifficultyConfig::kid();
        assert!(c.show_cooldowns);
    }

    #[test]
    fn kid_enemies_faster() {
        let c = DifficultyConfig::kid();
        assert!((c.enemy_speed_multiplier - 1.0).abs() < f32::EPSILON);
        assert!((c.enemy_windup_duration - 0.75).abs() < f32::EPSILON);
        assert_eq!(c.enemy_damage_stars, 2);
    }

    #[test]
    fn kid_boss_no_free_rests() {
        let c = DifficultyConfig::kid();
        assert!((c.boss_rest_interval - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn kid_no_auto_guidance() {
        let c = DifficultyConfig::kid();
        assert_eq!(c.auto_guidance_delay, None);
    }

    #[test]
    fn kid_p2_vulnerable_full_damage() {
        let c = DifficultyConfig::kid();
        assert!(!c.p2_invulnerable);
        assert!((c.p2_damage_multiplier - 1.0).abs() < f32::EPSILON);
        assert_eq!(c.p2_control_scheme, ControlScheme::Full);
    }

    #[test]
    fn kid_star_scatter_far() {
        let c = DifficultyConfig::kid();
        assert!((c.star_scatter_radius - 160.0).abs() < f32::EPSILON);
        assert_eq!(c.stars_lost_per_hit, 3);
    }

    // ── Default ───────────────────────────────────────────────────────────────

    #[test]
    fn default_is_toddler() {
        let c = DifficultyConfig::default();
        assert_eq!(c.mode, DifficultyMode::Toddler);
    }
}
