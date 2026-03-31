use rhai::Engine;

/// Verify all Rhai scripts compile without syntax errors.
/// This catches typos and script regressions at test time.

fn compile_script(path: &str) {
    let engine = Engine::new();
    let source = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));
    engine
        .compile(&source)
        .unwrap_or_else(|e| panic!("Failed to compile {path}: {e}"));
}

#[test]
fn level_01_compiles() {
    compile_script("assets/scripts/levels/home/level_01.rhai");
}

#[test]
fn dust_bunny_compiles() {
    compile_script("assets/scripts/enemies/dust_bunny.rhai");
}

#[test]
fn boss_living_room_compiles() {
    compile_script("assets/scripts/enemies/boss_living_room.rhai");
}

#[test]
fn home_phase_compiles() {
    compile_script("assets/scripts/phases/home.rhai");
}

/// Verify phase_config returns a map.
#[test]
fn home_phase_config_returns_map() {
    let engine = Engine::new();
    let source = std::fs::read_to_string("assets/scripts/phases/home.rhai").unwrap();
    let ast = engine.compile(&source).unwrap();
    let mut scope = rhai::Scope::new();
    let result: rhai::Map = engine.call_fn(&mut scope, &ast, "phase_config", ()).unwrap();
    assert!(result.contains_key("name"));
    assert!(result.contains_key("gravity"));
    assert!(result.contains_key("levels"));
}
