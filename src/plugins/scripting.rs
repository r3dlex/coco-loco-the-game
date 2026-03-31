use bevy::prelude::*;
use rhai::{Engine, Scope, AST};
use std::collections::HashMap;

use crate::state::{GameState, GameSystemSet};

// ── Resources ─────────────────────────────────────────────────────────────────

/// The Rhai scripting engine. `sync` feature required for Send + Sync.
#[derive(Resource)]
pub struct RhaiEngine {
    pub engine: Engine,
}

impl Default for RhaiEngine {
    fn default() -> Self {
        let mut engine = Engine::new();
        register_game_api(&mut engine);
        Self { engine }
    }
}

/// Cache of compiled Rhai ASTs keyed by script path.
#[derive(Resource, Default)]
pub struct ScriptCache {
    pub scripts: HashMap<String, AST>,
}

/// The currently active level script.
#[derive(Resource, Default)]
pub struct ActiveLevelScript {
    pub path: Option<String>,
    pub scope: Scope<'static>,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RhaiEngine>()
            .init_resource::<ScriptCache>()
            .init_resource::<ActiveLevelScript>();

        app.add_systems(OnEnter(GameState::Playing), load_level_script);

        app.add_systems(
            Update,
            (
                run_level_script_update.in_set(GameSystemSet::Scripting),
                #[cfg(debug_assertions)]
                hot_reload_scripts.in_set(GameSystemSet::Scripting),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

// ── Script loading ────────────────────────────────────────────────────────────

fn load_level_script(
    engine: Res<RhaiEngine>,
    mut cache: ResMut<ScriptCache>,
    mut active: ResMut<ActiveLevelScript>,
) {
    let path = "assets/scripts/levels/home/level_01.rhai";

    match std::fs::read_to_string(path) {
        Ok(source) => match engine.engine.compile(&source) {
            Ok(ast) => {
                cache.scripts.insert(path.to_string(), ast.clone());
                active.path = Some(path.to_string());

                // Call on_enter
                let mut scope = Scope::new();
                if let Err(e) = engine.engine.call_fn::<()>(&mut scope, &ast, "on_enter", ()) {
                    warn!("Script on_enter error: {e}");
                }
                active.scope = scope;
                info!("Level script loaded: {path}");
            }
            Err(e) => warn!("Failed to compile script {path}: {e}"),
        },
        Err(e) => warn!("Failed to read script {path}: {e}"),
    }
}

fn run_level_script_update(
    engine: Res<RhaiEngine>,
    cache: Res<ScriptCache>,
    mut active: ResMut<ActiveLevelScript>,
    time: Res<Time>,
) {
    let Some(path) = &active.path.clone() else { return };
    let Some(ast) = cache.scripts.get(path) else { return };

    let dt = time.delta_secs();
    // update(dt) is optional — many scripts won't define it
    let _ = engine
        .engine
        .call_fn::<()>(&mut active.scope, ast, "update", (dt,));
}

/// Hot-reloads scripts when files change on disk (debug builds only).
#[cfg(debug_assertions)]
fn hot_reload_scripts(
    engine: Res<RhaiEngine>,
    mut cache: ResMut<ScriptCache>,
    active: Res<ActiveLevelScript>,
) {
    let Some(path) = &active.path else { return };

    // Simple timestamp-based reload: try reading and recompiling each frame.
    // A real implementation would use `notify` or file metadata timestamps.
    if let Ok(source) = std::fs::read_to_string(path) {
        if let Ok(ast) = engine.engine.compile(&source) {
            cache.scripts.insert(path.clone(), ast);
        }
    }
}

// ── Rhai game API registration ─────────────────────────────────────────────────

fn register_game_api(engine: &mut Engine) {
    // Log helpers
    engine.register_fn("log_info", |msg: &str| info!("[Script] {msg}"));
    engine.register_fn("log_warn", |msg: &str| warn!("[Script] {msg}"));

    // Math helpers
    engine.register_fn("random_f32", || rand_f32());

    // TODO: register spawn_enemy, spawn_pickup, play_sound, set_music,
    // show_guide_trail, add_team_up_zone, etc. as the Bevy API surfaces mature.
    // These will use a thread-local command queue pattern or a channel approach
    // to bridge into the ECS safely.
}

fn rand_f32() -> f32 {
    // Simple LCG — replace with `rand` crate when added as a dep
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (t % 10000) as f32 / 10000.0
}
