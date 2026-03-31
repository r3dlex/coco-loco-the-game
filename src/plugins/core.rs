use bevy::prelude::*;

use crate::{
    events::*,
    resources::{CoOpState, DifficultyConfig, StarMeter},
    state::{FusionState, GameState, GameSystemSet},
};

/// Registers states, events, system-set ordering, and global resources.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // States
        app.init_state::<GameState>();
        app.init_state::<FusionState>();

        // Global resources
        let difficulty = DifficultyConfig::default();
        let meter_size = difficulty.fusion_meter_size;
        app.insert_resource(difficulty)
            .insert_resource(CoOpState::default())
            .insert_resource(StarMeter::new(meter_size));

        // Events
        app.add_event::<DamageEvent>()
            .add_event::<CharacterSwitchedEvent>()
            .add_event::<AbilityUsedEvent>()
            .add_event::<FusionActivatedEvent>()
            .add_event::<FusionExpiredEvent>()
            .add_event::<StarCollectedEvent>()
            .add_event::<StarMeterFullEvent>()
            .add_event::<EnemyDefeatedEvent>()
            .add_event::<LevelCompleteEvent>()
            .add_event::<Player2JoinedEvent>()
            .add_event::<Player2LeftEvent>()
            .add_event::<CoOpTierChangedEvent>()
            .add_event::<TeamUpStartedEvent>()
            .add_event::<TeamUpEndedEvent>();

        // System-set ordering for Update schedule
        app.configure_sets(
            Update,
            (
                GameSystemSet::Input,
                GameSystemSet::Scripting,
                GameSystemSet::GameLogic,
                GameSystemSet::Physics,
                GameSystemSet::Collision,
                GameSystemSet::Cleanup,
            )
                .chain(),
        );

        // Startup: spawn the 2D camera
        app.add_systems(Startup, spawn_camera);

        // Select difficulty on first launch
        app.add_systems(OnEnter(GameState::MainMenu), show_main_menu);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn show_main_menu(mut _commands: Commands) {
    // TODO: spawn main-menu UI entities
    info!("Coco Loco — Main Menu");
}
