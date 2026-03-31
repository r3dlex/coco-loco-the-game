use bevy::prelude::*;

use crate::{
    components::character::{ActiveCharacter, IdleTracker},
    resources::difficulty::DifficultyConfig,
    state::{GameState, GameSystemSet},
};

/// Auto-guidance system: shows a glowing trail after idle, nudges the companion forward.
pub struct GuidancePlugin;

#[derive(Component)]
pub struct GuidanceTrail;

impl Plugin for GuidancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                track_idle_time.in_set(GameSystemSet::GameLogic),
                spawn_guidance_trail.in_set(GameSystemSet::GameLogic),
                despawn_trail_on_input.in_set(GameSystemSet::GameLogic),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn track_idle_time(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut IdleTracker, With<ActiveCharacter>>,
) {
    let has_input = keys.get_just_pressed().len() > 0 || keys.get_pressed().len() > 0;

    for mut tracker in &mut query {
        if has_input {
            tracker.elapsed = 0.0;
        } else {
            tracker.elapsed += time.delta_secs();
        }
    }
}

pub fn spawn_guidance_trail(
    mut commands: Commands,
    difficulty: Res<DifficultyConfig>,
    query: Query<(&IdleTracker, &Transform), With<ActiveCharacter>>,
    existing_trail: Query<Entity, With<GuidanceTrail>>,
) {
    let Some(threshold) = difficulty.auto_guidance_delay else { return };

    for (tracker, _transform) in &query {
        if tracker.elapsed >= threshold && existing_trail.is_empty() {
            commands.spawn((
                GuidanceTrail,
                Transform::default(),
                Visibility::default(),
            ));
            info!("Guidance trail spawned after {:.1}s idle", tracker.elapsed);
        }
    }
}

pub fn despawn_trail_on_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    trail: Query<Entity, With<GuidanceTrail>>,
) {
    if keys.get_just_pressed().len() > 0 {
        for entity in &trail {
            commands.entity(entity).despawn();
        }
    }
}
