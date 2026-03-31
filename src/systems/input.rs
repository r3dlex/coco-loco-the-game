use bevy::prelude::*;

use crate::{
    components::{
        character::{ActiveCharacter, CharacterType, IdleTracker},
        movement::Facing,
    },
    resources::{
        co_op::CoOpState,
        difficulty::{ControlScheme, DifficultyConfig},
    },
};

/// Reads keyboard / gamepad input for the active P1 character.
///
/// Velocity / jump impulses are applied in `movement.rs` systems.
/// This system only tags intent via components; physics resolves it.
pub fn read_character_input(
    keys: Res<ButtonInput<KeyCode>>,
    _gamepads: Query<&Gamepad>,
    _difficulty: Res<DifficultyConfig>,
    _co_op: Res<CoOpState>,
    mut query: Query<
        (&CharacterType, &mut Facing, &mut IdleTracker),
        With<ActiveCharacter>,
    >,
) {
    for (_char_type, mut facing, mut idle) in &mut query {
        let mut any_input = false;

        // ── Horizontal movement ───────────────────────────────────────────────
        let left = keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA);
        let right = keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD);

        if left {
            *facing = Facing::Left;
            any_input = true;
        } else if right {
            *facing = Facing::Right;
            any_input = true;
        }

        if any_input {
            idle.elapsed = 0.0;
        }
    }
}

/// Reads P2 input (simplified or full scheme depending on difficulty).
pub fn read_p2_input(
    keys: Res<ButtonInput<KeyCode>>,
    difficulty: Res<DifficultyConfig>,
    co_op: Res<CoOpState>,
    mut query: Query<
        (&CharacterType, &mut Facing, &mut IdleTracker),
        With<crate::components::character::Player2>,
    >,
) {
    if !co_op.active {
        return;
    }

    for (_char_type, mut facing, mut idle) in &mut query {
        let (left, right, _jump, _action) = match difficulty.p2_control_scheme {
            ControlScheme::Simplified => (
                keys.pressed(KeyCode::ArrowLeft),
                keys.pressed(KeyCode::ArrowRight),
                keys.pressed(KeyCode::ArrowUp)
                    || keys.just_pressed(KeyCode::KeyZ)
                    || keys.just_pressed(KeyCode::KeyX),
                keys.just_pressed(KeyCode::ShiftRight) || keys.just_pressed(KeyCode::ControlRight),
            ),
            ControlScheme::Full => (
                keys.pressed(KeyCode::ArrowLeft),
                keys.pressed(KeyCode::ArrowRight),
                keys.just_pressed(KeyCode::ArrowUp),
                keys.just_pressed(KeyCode::ShiftRight),
            ),
        };

        if left {
            *facing = Facing::Left;
            idle.elapsed = 0.0;
        } else if right {
            *facing = Facing::Right;
            idle.elapsed = 0.0;
        }
    }
}
