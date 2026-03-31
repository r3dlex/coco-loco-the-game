use bevy::prelude::*;

use crate::{
    components::character::{ActiveCharacter, CompanionCharacter, Player2},
    resources::co_op::CoOpState,
};

/// Smooth camera follow for single-player. Camera leads slightly in the movement direction.
pub fn follow_active_character(
    time: Res<Time>,
    co_op: Res<CoOpState>,
    player: Query<&Transform, (With<ActiveCharacter>, Without<Camera2d>)>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    if co_op.active {
        return; // handled by update_co_op_camera
    }

    let Ok(player_tf) = player.get_single() else { return };
    let Ok(mut cam_tf) = camera.get_single_mut() else { return };

    let target = Vec3::new(
        player_tf.translation.x + 80.0, // lead forward
        player_tf.translation.y + 20.0, // slight upward bias
        cam_tf.translation.z,
    );

    cam_tf.translation = cam_tf.translation.lerp(target, 5.0 * time.delta_secs());
}

/// Co-op camera: keeps both players visible with dynamic zoom.
pub fn update_co_op_camera(
    co_op: Res<CoOpState>,
    p1: Query<&Transform, (With<ActiveCharacter>, Without<Camera2d>)>,
    p2: Query<&Transform, (With<CompanionCharacter>, With<Player2>, Without<Camera2d>)>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    time: Res<Time>,
) {
    if !co_op.active {
        return;
    }

    let (Ok(p1_tf), Ok(p2_tf)) = (p1.get_single(), p2.get_single()) else { return };
    let Ok((mut cam_tf, mut projection)) = camera.get_single_mut() else { return };

    let midpoint = (p1_tf.translation + p2_tf.translation) / 2.0;
    let target_pos = Vec3::new(midpoint.x, midpoint.y + 20.0, cam_tf.translation.z);
    cam_tf.translation = cam_tf.translation.lerp(target_pos, 5.0 * time.delta_secs());

    // Dynamic zoom: widen when players are far apart
    let distance = (p1_tf.translation.x - p2_tf.translation.x).abs();
    let player_widths = distance / 64.0; // assume 64px character width
    let target_scale = if player_widths > 16.0 {
        1.5_f32
    } else if player_widths > 8.0 {
        1.0 + (player_widths - 8.0) / 16.0
    } else {
        1.0
    };

    projection.scale = projection.scale.lerp(target_scale, 3.0 * time.delta_secs());
}
