use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_kira_audio::AudioPlugin;

mod components;
mod events;
mod plugins;
mod resources;
mod state;
mod systems;

use plugins::{
    art::ArtPlugin,
    audio::GameAudioPlugin,
    character::CharacterPlugin,
    companion::CompanionPlugin,
    co_op::CoOpPlugin,
    core::CorePlugin,
    enemy::EnemyPlugin,
    fusion::FusionPlugin,
    guidance::GuidancePlugin,
    save::SavePlugin,
    scripting::ScriptingPlugin,
    side_scroller::SideScrollerPlugin,
    ui::UIPlugin,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Coco Loco".into(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        // Third-party plugins
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(AudioPlugin)
        // Game plugins — ordered: core first, then systems, then presentation
        .add_plugins((
            CorePlugin,
            ScriptingPlugin,
            CharacterPlugin,
            CompanionPlugin,
            CoOpPlugin,
            FusionPlugin,
            SideScrollerPlugin,
            EnemyPlugin,
            ArtPlugin,
            GameAudioPlugin,
            UIPlugin,
            GuidancePlugin,
            SavePlugin,
        ))
        .run();
}
