use bevy::prelude::*;

use crate::{
    components::character::{ActiveCharacter, CompanionCharacter, DoubleTrouble, Player1},
    events::{FusionActivatedEvent, FusionExpiredEvent, StarCollectedEvent, StarMeterFullEvent},
    resources::{
        difficulty::{DifficultyConfig, DifficultyMode, FusionTrigger},
        star_meter::StarMeter,
    },
    state::{FusionState, GameState, GameSystemSet},
};

/// Duration of the fused form in seconds.
pub const FUSION_DURATION: f32 = 12.0;
/// How many seconds before expiry the form starts flickering.
pub const FLICKER_WARNING: f32 = 3.0;

#[derive(Resource, Default)]
pub struct FusionTimer {
    pub elapsed: f32,
    pub cooldown_remaining: f32,
}

pub struct FusionPlugin;

impl Plugin for FusionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FusionTimer>();

        app.add_systems(
            Update,
            (
                collect_stars.in_set(GameSystemSet::GameLogic),
                check_fusion_trigger.in_set(GameSystemSet::GameLogic),
                handle_fusion_activated.in_set(GameSystemSet::GameLogic),
                tick_fusion_timer.in_set(GameSystemSet::GameLogic),
                tick_fusion_cooldown.in_set(GameSystemSet::GameLogic),
                handle_fusion_expired.in_set(GameSystemSet::GameLogic),
            )
                .run_if(in_state(GameState::Playing)),
        );

        app.add_systems(OnEnter(FusionState::Fused), on_fusion_enter);
        app.add_systems(OnExit(FusionState::Fused), on_fusion_exit);
    }
}

pub fn collect_stars(
    mut star_events: EventReader<StarCollectedEvent>,
    mut meter: ResMut<StarMeter>,
    mut full_events: EventWriter<StarMeterFullEvent>,
) {
    for _ in star_events.read() {
        if meter.add(1) {
            full_events.send(StarMeterFullEvent);
        }
    }
}

pub fn check_fusion_trigger(
    difficulty: Res<DifficultyConfig>,
    fusion_timer: Res<FusionTimer>,
    fusion_state: Res<State<FusionState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut full_events: EventReader<StarMeterFullEvent>,
    mut activate_events: EventWriter<FusionActivatedEvent>,
) {
    if *fusion_state != FusionState::Normal || fusion_timer.cooldown_remaining > 0.0 {
        full_events.clear();
        return;
    }

    match difficulty.fusion_trigger {
        FusionTrigger::Automatic => {
            for _ in full_events.read() {
                activate_events.send(FusionActivatedEvent);
            }
        }
        FusionTrigger::Manual => {
            full_events.clear();
            if keys.just_pressed(KeyCode::KeyF) {
                activate_events.send(FusionActivatedEvent);
            }
        }
    }
}

pub fn handle_fusion_activated(
    mut commands: Commands,
    mut activate_events: EventReader<FusionActivatedEvent>,
    mut next_state: ResMut<NextState<FusionState>>,
    mut meter: ResMut<StarMeter>,
    p1_char: Query<Entity, (With<ActiveCharacter>, With<Player1>)>,
    companion_char: Query<Entity, With<CompanionCharacter>>,
) {
    for _ in activate_events.read() {
        for entity in p1_char.iter().chain(companion_char.iter()) {
            commands.entity(entity).insert(Visibility::Hidden);
        }

        commands.spawn((
            DoubleTrouble,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
        ));

        meter.reset();
        next_state.set(FusionState::Fused);
        info!("Double Trouble activated!");
    }
}

pub fn tick_fusion_timer(
    time: Res<Time>,
    fusion_state: Res<State<FusionState>>,
    mut timer: ResMut<FusionTimer>,
    mut expired_events: EventWriter<FusionExpiredEvent>,
) {
    if *fusion_state != FusionState::Fused {
        return;
    }
    timer.elapsed += time.delta_secs();
    if timer.elapsed >= FUSION_DURATION {
        expired_events.send(FusionExpiredEvent);
        timer.elapsed = 0.0;
    }
}

pub fn tick_fusion_cooldown(
    time: Res<Time>,
    mut timer: ResMut<FusionTimer>,
    fusion_state: Res<State<FusionState>>,
    mut next_state: ResMut<NextState<FusionState>>,
) {
    if *fusion_state != FusionState::Cooldown {
        return;
    }
    if timer.cooldown_remaining > 0.0 {
        timer.cooldown_remaining -= time.delta_secs();
        if timer.cooldown_remaining <= 0.0 {
            timer.cooldown_remaining = 0.0;
            next_state.set(FusionState::Normal);
        }
    }
}

pub fn handle_fusion_expired(
    mut commands: Commands,
    mut expired_events: EventReader<FusionExpiredEvent>,
    mut next_state: ResMut<NextState<FusionState>>,
    difficulty: Res<DifficultyConfig>,
    mut fusion_timer: ResMut<FusionTimer>,
    fused: Query<Entity, With<DoubleTrouble>>,
    p1_chars: Query<Entity, (With<ActiveCharacter>, With<Player1>)>,
    companions: Query<Entity, With<CompanionCharacter>>,
) {
    for _ in expired_events.read() {
        for entity in fused.iter() {
            commands.entity(entity).despawn();
        }

        for entity in p1_chars.iter().chain(companions.iter()) {
            commands.entity(entity).insert(Visibility::Visible);
        }

        match difficulty.mode {
            DifficultyMode::Toddler => {
                next_state.set(FusionState::Normal);
            }
            DifficultyMode::Kid => {
                fusion_timer.cooldown_remaining = difficulty.fusion_cooldown;
                next_state.set(FusionState::Cooldown);
            }
        }

        info!("Double Trouble expired. Brothers pop apart!");
    }
}

fn on_fusion_enter() {
    info!("[Fusion] Fused state entered — screen shake, particles, orchestral swell");
}

fn on_fusion_exit() {
    info!("[Fusion] Fused state exited");
}
