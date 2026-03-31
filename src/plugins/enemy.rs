use bevy::prelude::*;

use crate::{
    events::{DamageEvent, EnemyDefeatedEvent},
    resources::difficulty::DifficultyConfig,
    state::{GameState, GameSystemSet},
};

/// Enemy identity and state.
#[derive(Component)]
pub struct Enemy {
    pub script_name: String,
    pub health: i32,
    pub attack_timer: f32,
}

/// AI state for patrolling enemies.
#[derive(Component, Default)]
pub enum EnemyState {
    #[default]
    Patrol,
    Attacking,
    Stunned { remaining: f32 },
    Resting { remaining: f32 },
}

/// Patrol corridor for simple enemies.
#[derive(Component)]
pub struct PatrolBounds {
    pub x_min: f32,
    pub x_max: f32,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_ai_update.in_set(GameSystemSet::GameLogic),
                handle_enemy_damage.in_set(GameSystemSet::Collision),
                check_enemy_defeated.in_set(GameSystemSet::Cleanup),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn enemy_ai_update(
    time: Res<Time>,
    difficulty: Res<DifficultyConfig>,
    mut enemies: Query<(&mut Transform, &mut Enemy, &mut EnemyState, &PatrolBounds)>,
) {
    let dt = time.delta_secs();
    let speed = 60.0 * difficulty.enemy_speed_multiplier;

    for (mut transform, mut enemy, mut state, bounds) in &mut enemies {
        match *state {
            EnemyState::Patrol => {
                transform.translation.x += speed * dt;
                if transform.translation.x >= bounds.x_max {
                    transform.translation.x = bounds.x_max;
                    transform.scale.x = -transform.scale.x.abs(); // flip left
                } else if transform.translation.x <= bounds.x_min {
                    transform.translation.x = bounds.x_min;
                    transform.scale.x = transform.scale.x.abs(); // flip right
                }
            }
            EnemyState::Stunned { ref mut remaining } => {
                *remaining -= dt;
                if *remaining <= 0.0 {
                    *state = EnemyState::Patrol;
                }
            }
            EnemyState::Resting { ref mut remaining } => {
                *remaining -= dt;
                if *remaining <= 0.0 {
                    *state = EnemyState::Patrol;
                }
            }
            EnemyState::Attacking => {
                enemy.attack_timer -= dt;
                // TODO: trigger attack when timer expires
            }
        }
    }
}

fn handle_enemy_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut enemies: Query<&mut Enemy>,
) {
    for event in damage_events.read() {
        if let Ok(mut enemy) = enemies.get_mut(event.target) {
            enemy.health -= 1;
        }
    }
}

fn check_enemy_defeated(
    mut commands: Commands,
    enemies: Query<(Entity, &Enemy, &Transform)>,
    mut defeated_events: EventWriter<EnemyDefeatedEvent>,
) {
    for (entity, enemy, transform) in &enemies {
        if enemy.health <= 0 {
            defeated_events.send(EnemyDefeatedEvent {
                enemy: entity,
                position: transform.translation.truncate(),
            });
            commands.entity(entity).despawn();
        }
    }
}
