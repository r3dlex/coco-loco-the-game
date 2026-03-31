use bevy::prelude::*;

use crate::state::GameState;

/// Handles sprite atlas loading, character animations, and LUT post-processing.
pub struct ArtPlugin;

/// Animation state for sprite-sheet animation.
#[derive(Component)]
pub struct SpriteAnimation {
    pub frames: Vec<usize>,
    pub current: usize,
    pub fps: f32,
    pub elapsed: f32,
    pub looping: bool,
}

impl SpriteAnimation {
    pub fn new(frames: Vec<usize>, fps: f32, looping: bool) -> Self {
        Self { frames, current: 0, fps, elapsed: 0.0, looping }
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        self.elapsed += dt;
        if self.elapsed >= 1.0 / self.fps {
            self.elapsed = 0.0;
            if self.current + 1 < self.frames.len() {
                self.current += 1;
                return true;
            } else if self.looping {
                self.current = 0;
                return true;
            }
        }
        false
    }

    pub fn current_frame(&self) -> usize {
        self.frames[self.current]
    }
}

impl Plugin for ArtPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), preload_assets);
        app.add_systems(Update, tick_sprite_animations.run_if(in_state(GameState::Playing)));
    }
}

fn preload_assets(mut _commands: Commands, _asset_server: Res<AssetServer>) {
    // TODO: load sprite atlases and LUT textures via AssetServer
    info!("Preloading art assets…");
}

fn tick_sprite_animations(
    time: Res<Time>,
    mut query: Query<(&mut SpriteAnimation, &mut Sprite)>,
) {
    for (mut anim, sprite) in &mut query {
        if anim.tick(time.delta_secs()) {
            // TODO: update TextureAtlas index when atlas sprites are wired up
            let _ = sprite; // suppress unused warning
        }
    }
}
