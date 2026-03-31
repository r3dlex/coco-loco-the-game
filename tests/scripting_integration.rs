use rhai::{Engine, Scope};

/// Verify the home phase config returns all expected keys.
#[test]
fn home_phase_config_has_all_fields() {
    let engine = Engine::new();
    let source = std::fs::read_to_string("assets/scripts/phases/home.rhai").unwrap();
    let ast = engine.compile(&source).unwrap();
    let mut scope = Scope::new();
    let config: rhai::Map = engine.call_fn(&mut scope, &ast, "phase_config", ()).unwrap();

    assert!(config.contains_key("name"));
    assert!(config.contains_key("gravity"));
    assert!(config.contains_key("parallax_layers"));
    assert!(config.contains_key("post_process"));
    assert!(config.contains_key("boss_co_op_scale_toddler"));
    assert!(config.contains_key("boss_co_op_scale_kid"));
    assert!(config.contains_key("levels"));
    assert!(config.contains_key("music_exploration"));
    assert!(config.contains_key("music_action"));
    assert!(config.contains_key("music_boss"));
}

/// Verify Home phase gravity is 1.0 (normal).
#[test]
fn home_phase_gravity_is_normal() {
    let engine = Engine::new();
    let source = std::fs::read_to_string("assets/scripts/phases/home.rhai").unwrap();
    let ast = engine.compile(&source).unwrap();
    let mut scope = Scope::new();
    let config: rhai::Map = engine.call_fn(&mut scope, &ast, "phase_config", ()).unwrap();

    let gravity = config.get("gravity").unwrap().clone_cast::<f64>();
    assert!((gravity - 1.0).abs() < f64::EPSILON);
}

/// Verify boss co-op scales match spec.
#[test]
fn home_phase_boss_coop_scales() {
    let engine = Engine::new();
    let source = std::fs::read_to_string("assets/scripts/phases/home.rhai").unwrap();
    let ast = engine.compile(&source).unwrap();
    let mut scope = Scope::new();
    let config: rhai::Map = engine.call_fn(&mut scope, &ast, "phase_config", ()).unwrap();

    let toddler_scale = config.get("boss_co_op_scale_toddler").unwrap().clone_cast::<f64>();
    let kid_scale = config.get("boss_co_op_scale_kid").unwrap().clone_cast::<f64>();
    assert!((toddler_scale - 1.25).abs() < f64::EPSILON);
    assert!((kid_scale - 1.5).abs() < f64::EPSILON);
}

/// Verify levels array has expected entries.
#[test]
fn home_phase_has_4_levels() {
    let engine = Engine::new();
    let source = std::fs::read_to_string("assets/scripts/phases/home.rhai").unwrap();
    let ast = engine.compile(&source).unwrap();
    let mut scope = Scope::new();
    let config: rhai::Map = engine.call_fn(&mut scope, &ast, "phase_config", ()).unwrap();

    let levels = config.get("levels").unwrap().clone_cast::<rhai::Array>();
    assert_eq!(levels.len(), 4);
}

/// Verify all Rhai scripts define expected functions.
#[test]
fn dust_bunny_has_required_functions() {
    let engine = Engine::new();
    let source = std::fs::read_to_string("assets/scripts/enemies/dust_bunny.rhai").unwrap();
    let ast = engine.compile(&source).unwrap();

    let fns: Vec<_> = ast.iter_functions().map(|f| f.name.to_string()).collect();
    assert!(fns.contains(&"create".to_string()));
    assert!(fns.contains(&"update".to_string()));
    assert!(fns.contains(&"on_hit".to_string()));
}

#[test]
fn boss_living_room_has_required_functions() {
    let engine = Engine::new();
    let source = std::fs::read_to_string("assets/scripts/enemies/boss_living_room.rhai").unwrap();
    let ast = engine.compile(&source).unwrap();

    let fns: Vec<_> = ast.iter_functions().map(|f| f.name.to_string()).collect();
    assert!(fns.contains(&"create".to_string()));
    assert!(fns.contains(&"update".to_string()));
    assert!(fns.contains(&"on_hit".to_string()));
    assert!(fns.contains(&"on_p2_joined".to_string()));
}

#[test]
fn level_01_has_required_functions() {
    let engine = Engine::new();
    let source = std::fs::read_to_string("assets/scripts/levels/home/level_01.rhai").unwrap();
    let ast = engine.compile(&source).unwrap();

    let fns: Vec<_> = ast.iter_functions().map(|f| f.name.to_string()).collect();
    assert!(fns.contains(&"on_enter".to_string()));
    assert!(fns.contains(&"on_trigger".to_string()));
    assert!(fns.contains(&"update".to_string()));
}
