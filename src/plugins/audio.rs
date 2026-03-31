use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::state::GameState;

/// Wraps bevy_kira_audio with game-specific sound logic:
/// - Music stem layering per phase
/// - SFX priority queuing
/// - Co-op join / fusion fanfares
pub struct GameAudioPlugin;

/// Logical SFX names resolved to asset paths at runtime.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SfxId {
    Jump,
    Land,
    StarCollect,
    HitEnemy,
    PlayerHit,
    ActionIdle,     // whoopee cushion / kazoo
    FusionSwell,
    BossStunned,
    P2Join,
    P2Leave,
}

impl SfxId {
    pub fn path(&self) -> &'static str {
        match self {
            SfxId::Jump => "audio/sfx/jump.ogg",
            SfxId::Land => "audio/sfx/land.ogg",
            SfxId::StarCollect => "audio/sfx/star_collect.ogg",
            SfxId::HitEnemy => "audio/sfx/hit_enemy.ogg",
            SfxId::PlayerHit => "audio/sfx/player_hit.ogg",
            SfxId::ActionIdle => "audio/sfx/action_idle.ogg",
            SfxId::FusionSwell => "audio/sfx/fusion_swell.ogg",
            SfxId::BossStunned => "audio/sfx/boss_stunned.ogg",
            SfxId::P2Join => "audio/sfx/p2_join.ogg",
            SfxId::P2Leave => "audio/sfx/p2_leave.ogg",
        }
    }
}

/// Queue SFX to be played next frame.
#[derive(Resource, Default)]
pub struct SfxQueue(pub Vec<SfxId>);

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SfxQueue>();

        app.add_systems(OnEnter(GameState::Playing), start_level_music);
        app.add_systems(Update, flush_sfx_queue.run_if(in_state(GameState::Playing)));
    }
}

fn start_level_music(audio: Res<Audio>, asset_server: Res<AssetServer>) {
    audio
        .play(asset_server.load("audio/music/home_cozy.ogg"))
        .looped();
}

fn flush_sfx_queue(
    mut queue: ResMut<SfxQueue>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for sfx in queue.0.drain(..) {
        audio.play(asset_server.load(sfx.path()));
    }
}
