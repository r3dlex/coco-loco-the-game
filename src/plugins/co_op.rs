use bevy::prelude::*;

use crate::{
    components::character::{CompanionAI, CompanionCharacter, Player2},
    events::{CoOpTierChangedEvent, Player2JoinedEvent, Player2LeftEvent, TeamUpEndedEvent, TeamUpStartedEvent},
    resources::co_op::{CoOpState, CoOpTier, InputDevice, KeyboardLayout},
    state::{GameState, GameSystemSet},
    systems::camera::update_co_op_camera,
};

pub struct CoOpPlugin;

impl Plugin for CoOpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                detect_p2_join.in_set(GameSystemSet::Input),
                detect_p2_leave.in_set(GameSystemSet::Input),
                apply_p2_join.in_set(GameSystemSet::GameLogic),
                apply_p2_leave.in_set(GameSystemSet::GameLogic),
                handle_tier_change.in_set(GameSystemSet::GameLogic),
                update_co_op_camera.in_set(GameSystemSet::GameLogic),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Detects P2 pressing Start (Enter or gamepad Start).
fn detect_p2_join(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
    co_op: Res<CoOpState>,
    mut join_events: EventWriter<Player2JoinedEvent>,
) {
    if co_op.active {
        return;
    }

    // Keyboard P2 join: Enter key (arrows layout assumed for now)
    if keys.just_pressed(KeyCode::Enter) {
        join_events.send(Player2JoinedEvent {
            device: InputDevice::Keyboard { layout: KeyboardLayout::Arrows },
        });
        return;
    }

    // Gamepad P2 join: any gamepad Start button (Bevy 0.15: gamepads are entities)
    for (entity, gamepad) in &gamepads {
        if gamepad.just_pressed(GamepadButton::Start) {
            join_events.send(Player2JoinedEvent {
                device: InputDevice::Gamepad(entity),
            });
            return;
        }
    }
}

/// Detects P2 pressing Start again to leave.
fn detect_p2_leave(
    keys: Res<ButtonInput<KeyCode>>,
    co_op: Res<CoOpState>,
    mut leave_events: EventWriter<Player2LeftEvent>,
) {
    if !co_op.active {
        return;
    }
    if keys.just_pressed(KeyCode::Enter) {
        leave_events.send(Player2LeftEvent);
    }
}

/// Removes CompanionAI, adds Player2 to the companion entity.
fn apply_p2_join(
    mut commands: Commands,
    mut join_events: EventReader<Player2JoinedEvent>,
    mut co_op: ResMut<CoOpState>,
    companion: Query<Entity, With<CompanionCharacter>>,
) {
    for event in join_events.read() {
        if let Ok(entity) = companion.get_single() {
            commands.entity(entity).remove::<CompanionAI>().insert(Player2);
            co_op.join(event.device.clone());
            info!("P2 joined! Device: {:?}", event.device);
        }
    }
}

/// Removes Player2, re-adds CompanionAI to the companion entity.
fn apply_p2_leave(
    mut commands: Commands,
    mut leave_events: EventReader<Player2LeftEvent>,
    mut co_op: ResMut<CoOpState>,
    companion: Query<Entity, (With<CompanionCharacter>, With<Player2>)>,
) {
    for _ in leave_events.read() {
        if let Ok(entity) = companion.get_single() {
            commands.entity(entity).remove::<Player2>().insert(CompanionAI);
            co_op.leave();
            info!("P2 left. Companion AI resumed.");
        }
    }
}

fn handle_tier_change(
    mut tier_events: EventReader<CoOpTierChangedEvent>,
    mut team_up_start: EventWriter<TeamUpStartedEvent>,
    mut team_up_end: EventWriter<TeamUpEndedEvent>,
    mut co_op: ResMut<CoOpState>,
) {
    for event in tier_events.read() {
        info!("Co-op tier: {:?} → {:?}", event.from, event.to);
        co_op.set_tier(event.to.clone());

        match (&event.from, &event.to) {
            (_, CoOpTier::TeamUp) => { team_up_start.send(TeamUpStartedEvent); }
            (CoOpTier::TeamUp, _) => { team_up_end.send(TeamUpEndedEvent); }
            _ => {}
        }
    }
}
