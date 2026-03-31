use bevy::prelude::*;

use crate::resources::co_op::{CoOpTier, InputDevice};

// ── Combat ────────────────────────────────────────────────────────────────────

/// A character took a hit. `stars_lost` scatter around their position.
#[derive(Event)]
pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
    pub stars_lost: u32,
}

// ── Character ─────────────────────────────────────────────────────────────────

/// P1 switched from one character to another (Kid solo only).
#[derive(Event)]
pub struct CharacterSwitchedEvent {
    pub from: Entity,
    pub to: Entity,
}

/// An ability was used by a character.
#[derive(Event)]
pub struct AbilityUsedEvent {
    pub character: Entity,
    pub ability: AbilityId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbilityId {
    // Loko
    Punch,
    Dash,
    DevastatungFury,
    // Roco
    Cry,
    SmartVision,
    Mesmerize,
    MightyHammer,
}

// ── Fusion ────────────────────────────────────────────────────────────────────

/// Fusion activation has been triggered (meter full + trigger condition met).
#[derive(Event)]
pub struct FusionActivatedEvent;

/// Fusion timer expired; brothers pop apart.
#[derive(Event)]
pub struct FusionExpiredEvent;

// ── Stars ─────────────────────────────────────────────────────────────────────

/// A star was collected by a player or companion.
#[derive(Event)]
pub struct StarCollectedEvent {
    pub collector: Entity,
    pub position: Vec2,
}

/// The shared star meter reached its capacity — fusion is ready.
#[derive(Event)]
pub struct StarMeterFullEvent;

// ── Enemies ───────────────────────────────────────────────────────────────────

/// An enemy was defeated.
#[derive(Event)]
pub struct EnemyDefeatedEvent {
    pub enemy: Entity,
    pub position: Vec2,
}

// ── Level ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorldPhase {
    Home,
    Space,
    Dino,
    ThemePark,
    Ocean,
    Candy,
}

/// Player reached the level-end trigger.
#[derive(Event)]
pub struct LevelCompleteEvent {
    pub phase: WorldPhase,
    pub level: u32,
}

// ── Co-Op ─────────────────────────────────────────────────────────────────────

/// A second player has joined.
#[derive(Event)]
pub struct Player2JoinedEvent {
    pub device: InputDevice,
}

/// P2 has left (voluntary or disconnect).
#[derive(Event)]
pub struct Player2LeftEvent;

/// The co-op tier escalated or de-escalated.
#[derive(Event)]
pub struct CoOpTierChangedEvent {
    pub from: CoOpTier,
    pub to: CoOpTier,
}

/// A scripted team-up zone was entered.
#[derive(Event)]
pub struct TeamUpStartedEvent;

/// The team-up zone duration ended or was exited.
#[derive(Event)]
pub struct TeamUpEndedEvent;
