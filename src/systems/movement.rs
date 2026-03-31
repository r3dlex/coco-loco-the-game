use bevy::prelude::*;

use crate::components::{
    abilities::Abilities,
    movement::{CoyoteTimer, Dashing, Facing, Grounded, Movement},
};

/// Translates facing + movement speed into linear velocity via avian2d.
///
/// In a full implementation this would write to avian2d's `LinearVelocity`
/// component. Currently updates Transform directly for Phase-0 scaffolding.
pub fn apply_character_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Movement, &Facing, &mut Transform), Without<Dashing>>,
) {
    for (movement, facing, mut transform) in &mut query {
        let dir: f32 = match facing {
            Facing::Right if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) => 1.0,
            Facing::Left if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) => -1.0,
            _ => 0.0,
        };
        transform.translation.x += dir * movement.speed * time.delta_secs();
    }
}

/// Applies Loko dash movement and removes the Dashing component when done.
pub fn apply_dash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Dashing, &mut Transform)>,
) {
    for (entity, mut dash, mut transform) in &mut query {
        let dt = time.delta_secs();
        let dash_speed = 640.0_f32; // TODO: read from Movement component

        transform.translation.x += dash.direction * dash_speed * dt;
        dash.elapsed += dt;

        if dash.elapsed >= dash.duration {
            commands.entity(entity).remove::<Dashing>();
        }
    }
}

/// Tracks and expires coyote time.
pub fn apply_coyote_time(time: Res<Time>, mut query: Query<(&Grounded, &Movement, &mut CoyoteTimer)>) {
    for (grounded, _movement, mut coyote) in &mut query {
        if grounded.0 {
            coyote.elapsed = 0.0;
        } else {
            coyote.elapsed += time.delta_secs();
        }
    }
}

/// Ticks cooldowns on all abilities each frame.
pub fn tick_ability_cooldowns(time: Res<Time>, mut query: Query<&mut Abilities>) {
    let dt = time.delta_secs();
    for mut abilities in &mut query {
        for ability in &mut abilities.list {
            ability.tick(dt);
        }
    }
}
