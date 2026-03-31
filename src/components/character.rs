use bevy::prelude::*;

// ── Character identity ────────────────────────────────────────────────────────

#[derive(Component, Debug, Clone)]
pub struct Character {
    pub name: String,
    pub character_type: CharacterType,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum CharacterType {
    Loko,
    Roco,
}

/// Marker: this entity is the one P1 is currently controlling.
#[derive(Component)]
pub struct ActiveCharacter;

/// Marker: this entity is the companion (not P1's controlled character).
#[derive(Component)]
pub struct CompanionCharacter;

/// Marker: controlled by Player 1.
#[derive(Component)]
pub struct Player1;

/// Marker: controlled by Player 2 (only present when co-op is active).
#[derive(Component)]
pub struct Player2;

/// Marker: driven by companion AI (removed when P2 takes over).
#[derive(Component)]
pub struct CompanionAI;

// ── Double Trouble fused form ─────────────────────────────────────────────────

/// Marker: this entity IS the Double Trouble fused form.
#[derive(Component)]
pub struct DoubleTrouble;

// ── Hurt state ────────────────────────────────────────────────────────────────

/// Transient invincibility window after taking a hit.
#[derive(Component)]
pub struct Invincible {
    /// Remaining invincibility duration in seconds.
    pub timer: f32,
}

// ── Idle tracking ─────────────────────────────────────────────────────────────

/// Tracks how long the player has been idle (no input).
#[derive(Component, Default)]
pub struct IdleTracker {
    pub elapsed: f32,
}
