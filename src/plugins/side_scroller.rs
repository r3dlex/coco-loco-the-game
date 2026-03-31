use bevy::prelude::*;

use crate::state::{GameState, GameSystemSet};

/// Side-scroller plugin: gravity, parallax background layers, camera leading.
pub struct SideScrollerPlugin;

/// Gravity scale for the current phase (1.0 = normal, 0.3 = Space).
#[derive(Resource)]
pub struct PhaseGravity(pub f32);

impl Default for PhaseGravity {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Parallax background layer entity tag.
#[derive(Component)]
pub struct ParallaxLayer {
    /// How much this layer moves relative to the camera (0.0 = fixed, 1.0 = no parallax).
    pub depth: f32,
}

impl Plugin for SideScrollerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhaseGravity>();

        app.add_systems(OnEnter(GameState::Playing), setup_level_geometry);

        app.add_systems(
            Update,
            (
                scroll_parallax_layers.in_set(GameSystemSet::GameLogic),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn setup_level_geometry(mut _commands: Commands) {
    // TODO: load level geometry from Rhai / asset file
    info!("Level geometry setup");
}

fn scroll_parallax_layers(
    camera: Query<&Transform, (With<Camera2d>, Without<ParallaxLayer>)>,
    mut layers: Query<(&ParallaxLayer, &mut Transform)>,
) {
    let Ok(cam) = camera.get_single() else { return };

    for (layer, mut transform) in &mut layers {
        transform.translation.x = cam.translation.x * (1.0 - layer.depth);
    }
}
