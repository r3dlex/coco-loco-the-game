use bevy::prelude::*;

/// Top-level game flow state.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Playing,
    Paused,
}

/// Double Trouble fusion sub-state (relevant only while Playing).
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FusionState {
    /// Normal gameplay — no fusion active.
    #[default]
    Normal,
    /// Fusion activation animation playing.
    Activating,
    /// Fused form active (12-second window).
    Fused,
    /// Post-fusion cooldown (Kid: 45s; Toddler: wait for meter refill).
    Cooldown,
}

/// System set ordering for the main Update schedule.
///
/// `InputSet → ScriptingSet → GameLogicSet → PhysicsSet → CollisionSet → CleanupSet`
///
/// Render ordering is handled by Bevy's built-in render pipeline.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSystemSet {
    Input,
    Scripting,
    GameLogic,
    Physics,
    Collision,
    Cleanup,
}
