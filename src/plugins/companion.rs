use bevy::prelude::*;

use crate::{
    state::{GameState, GameSystemSet},
    systems::companion_ai::{companion_follow, companion_idle_reactions, companion_teleport_if_offscreen},
};

/// Drives the companion AI when P2 is absent.
///
/// When P2 joins (`CoOpPlugin` removes `CompanionAI` and adds `Player2`),
/// these systems have no entities to act on and are effectively inert.
pub struct CompanionPlugin;

impl Plugin for CompanionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                companion_follow.in_set(GameSystemSet::GameLogic),
                companion_idle_reactions.in_set(GameSystemSet::GameLogic),
                companion_teleport_if_offscreen.in_set(GameSystemSet::GameLogic),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
