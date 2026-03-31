use bevy::prelude::*;

use crate::{
    resources::{
        difficulty::{DifficultyConfig, DifficultyMode},
        star_meter::StarMeter,
    },
    state::{FusionState, GameState},
};

pub struct UIPlugin;

// ── Component markers ─────────────────────────────────────────────────────────

#[derive(Component)]
pub struct StarMeterUi;

#[derive(Component)]
pub struct StarIcon {
    pub index: usize,
}

#[derive(Component)]
pub struct Player2JoinedBanner;

#[derive(Component)]
pub struct FusionFlicker;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_hud);
        app.add_systems(OnExit(GameState::Playing), despawn_hud);

        app.add_systems(
            Update,
            (
                update_star_meter_ui,
                update_fusion_flicker,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_hud(mut commands: Commands, difficulty: Res<DifficultyConfig>) {
    let meter_size = difficulty.fusion_meter_size;

    // Root HUD node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(16.0)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..default()
            },
            StarMeterUi,
        ))
        .with_children(|parent| {
            for i in 0..meter_size as usize {
                parent.spawn((
                    Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        margin: UiRect::horizontal(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 0.0, 0.3)),
                    StarIcon { index: i },
                ));
            }
        });

    // In Kid Mode, also show cooldown HUD etc.
    if difficulty.mode == DifficultyMode::Kid {
        // TODO: spawn cooldown radial indicators
    }

    info!("HUD spawned ({meter_size}-star meter)");
}

fn despawn_hud(mut commands: Commands, hud: Query<Entity, With<StarMeterUi>>) {
    for entity in &hud {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_star_meter_ui(
    meter: Res<StarMeter>,
    mut icons: Query<(&StarIcon, &mut BackgroundColor)>,
) {
    if !meter.is_changed() {
        return;
    }
    for (icon, mut color) in &mut icons {
        if icon.index < meter.current as usize {
            *color = BackgroundColor(Color::srgb(1.0, 0.9, 0.0)); // filled — gold
        } else {
            *color = BackgroundColor(Color::srgba(1.0, 1.0, 0.0, 0.3)); // empty — dim
        }
    }
}

fn update_fusion_flicker(
    fusion_state: Res<State<FusionState>>,
    timer: Res<crate::plugins::fusion::FusionTimer>,
    mut flicker: Query<&mut Visibility, With<FusionFlicker>>,
) {
    if *fusion_state != FusionState::Fused {
        return;
    }
    let remaining = 12.0 - timer.elapsed;
    let should_flicker = remaining <= 3.0;
    for mut vis in &mut flicker {
        *vis = if should_flicker && (timer.elapsed * 8.0) as u32 % 2 == 0 {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}
