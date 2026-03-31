use bevy::prelude::*;

use crate::{
    components::{
        abilities::{loko_kid_abilities, loko_toddler_abilities, roco_kid_abilities, roco_toddler_abilities},
        character::{ActiveCharacter, Character, CharacterType, CompanionAI, CompanionCharacter, IdleTracker, Player1},
        movement::{CoyoteTimer, Facing, Grounded, Movement},
    },
    events::CharacterSwitchedEvent,
    resources::difficulty::{DifficultyConfig, DifficultyMode},
    state::{GameState, GameSystemSet},
    systems::{
        input::read_character_input,
        movement::{apply_character_movement, apply_coyote_time, apply_dash, tick_ability_cooldowns},
    },
};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_characters);

        app.add_systems(
            Update,
            (
                read_character_input.in_set(GameSystemSet::Input),
                tick_ability_cooldowns.in_set(GameSystemSet::GameLogic),
                apply_dash.in_set(GameSystemSet::GameLogic),
                apply_character_movement.in_set(GameSystemSet::GameLogic),
                apply_coyote_time.in_set(GameSystemSet::GameLogic),
                handle_character_switch.in_set(GameSystemSet::GameLogic),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn spawn_characters(mut commands: Commands, difficulty: Res<DifficultyConfig>) {
    let (loko_abilities, roco_abilities) = match difficulty.mode {
        DifficultyMode::Toddler => (loko_toddler_abilities(), roco_toddler_abilities()),
        DifficultyMode::Kid => (loko_kid_abilities(), roco_kid_abilities()),
    };

    // Loko — starts as active character
    commands.spawn((
        Character { name: "Loko".into(), character_type: CharacterType::Loko },
        ActiveCharacter,
        Player1,
        Movement {
            speed: 220.0,
            jump_force: 460.0,
            dash_speed: 640.0,
            coyote_time: difficulty.coyote_time,
        },
        loko_abilities,
        Facing::default(),
        Grounded::default(),
        CoyoteTimer::default(),
        IdleTracker::default(),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        Visibility::default(),
    ));

    // Roco — starts as companion
    commands.spawn((
        Character { name: "Roco".into(), character_type: CharacterType::Roco },
        CompanionCharacter,
        CompanionAI,
        Movement {
            speed: 200.0,
            jump_force: 420.0,
            dash_speed: 0.0,
            coyote_time: difficulty.coyote_time,
        },
        roco_abilities,
        Facing::default(),
        Grounded::default(),
        CoyoteTimer::default(),
        IdleTracker::default(),
        Transform::from_xyz(-200.0, 0.0, 0.0),
        Visibility::default(),
    ));

    info!("Characters spawned: Loko (P1 active) + Roco (companion AI)");
}

fn handle_character_switch(
    mut commands: Commands,
    difficulty: Res<DifficultyConfig>,
    co_op: Res<crate::resources::CoOpState>,
    keys: Res<ButtonInput<KeyCode>>,
    active: Query<Entity, With<ActiveCharacter>>,
    companion: Query<Entity, With<CompanionCharacter>>,
    mut switch_events: EventWriter<CharacterSwitchedEvent>,
) {
    if !difficulty.allow_character_switching || co_op.active {
        return;
    }

    if keys.just_pressed(KeyCode::Tab) {
        if let (Ok(from), Ok(to)) = (active.get_single(), companion.get_single()) {
            commands.entity(from).remove::<ActiveCharacter>().insert(CompanionCharacter);
            commands.entity(to).remove::<CompanionCharacter>().insert(ActiveCharacter);
            switch_events.send(CharacterSwitchedEvent { from, to });
        }
    }
}
