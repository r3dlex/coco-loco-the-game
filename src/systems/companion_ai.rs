use bevy::prelude::*;

use crate::components::character::{ActiveCharacter, CompanionAI, CompanionCharacter};

/// Distance the companion maintains behind the active character.
const FOLLOW_DISTANCE: f32 = 180.0;
/// If the companion is farther than this, teleport them.
const TELEPORT_THRESHOLD: f32 = 1024.0;
/// After this many seconds of the companion being off-screen, teleport.
const OFF_SCREEN_TIMEOUT: f32 = 3.0;

#[derive(Component, Default)]
pub struct OffScreenTimer(pub f32);

/// Companion walks toward P1, staying roughly FOLLOW_DISTANCE behind.
pub fn companion_follow(
    time: Res<Time>,
    p1: Query<&Transform, (With<ActiveCharacter>, Without<CompanionAI>)>,
    mut companion: Query<&mut Transform, (With<CompanionAI>, With<CompanionCharacter>)>,
) {
    let Ok(p1_tf) = p1.get_single() else { return };
    let Ok(mut comp_tf) = companion.get_single_mut() else { return };

    let target_x = p1_tf.translation.x - FOLLOW_DISTANCE;
    let diff = target_x - comp_tf.translation.x;

    // Only move if the gap is significant
    if diff.abs() > 8.0 {
        let speed = 180.0_f32;
        comp_tf.translation.x += diff.signum() * speed * time.delta_secs();
    }

    // Mirror Y movement loosely
    let y_diff = p1_tf.translation.y - comp_tf.translation.y;
    if y_diff.abs() > 4.0 {
        comp_tf.translation.y += y_diff.signum() * 120.0 * time.delta_secs();
    }
}

/// Plays idle animations and reacts to game events.
pub fn companion_idle_reactions(
    _time: Res<Time>,
    _p1: Query<&Transform, (With<ActiveCharacter>, Without<CompanionAI>)>,
    _companion: Query<&Transform, (With<CompanionAI>, With<CompanionCharacter>)>,
) {
    // TODO: trigger wave/dance animations and reaction sounds
}

/// If the companion is too far from P1, snap them close.
pub fn companion_teleport_if_offscreen(
    _commands: Commands,
    _time: Res<Time>,
    p1: Query<&Transform, (With<ActiveCharacter>, Without<CompanionAI>)>,
    mut companion: Query<(Entity, &mut Transform, Option<&mut OffScreenTimer>), (With<CompanionAI>, With<CompanionCharacter>)>,
) {
    let Ok(p1_tf) = p1.get_single() else { return };

    for (_entity, mut comp_tf, _timer) in &mut companion {
        let dist = (p1_tf.translation - comp_tf.translation).length();

        if dist > TELEPORT_THRESHOLD {
            // Instant teleport — snap behind P1
            comp_tf.translation.x = p1_tf.translation.x - FOLLOW_DISTANCE;
            comp_tf.translation.y = p1_tf.translation.y;
            info!("Companion teleported to P1 (distance was {dist:.0}px)");
        }
    }
}
