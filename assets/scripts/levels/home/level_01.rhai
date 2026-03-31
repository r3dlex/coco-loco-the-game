// Home Phase — Level 01: The Living Room
// Introduces Loko's punch and Roco's cry via contextual action button.
// Duration: 2-3 min (Toddler) / 4-6 min (Kid)

fn on_enter(level) {
    level.set_music("home_cozy");
    level.set_gravity(1.0);

    // Enemies
    level.spawn_enemy("dust_bunny", 400.0, 200.0, "patrol_simple");
    level.spawn_enemy("dust_bunny", 800.0, 200.0, "patrol_simple");
    level.spawn_enemy("dust_bunny", 1300.0, 200.0, "patrol_simple");

    // Star pickups
    level.spawn_pickup("star", 300.0,  250.0);
    level.spawn_pickup("star", 600.0,  300.0);
    level.spawn_pickup("star", 950.0,  250.0);
    level.spawn_pickup("star", 1200.0, 300.0);
    level.spawn_pickup("star", 1600.0, 280.0);

    // Team-up zone: Loko holds a heavy bookshelf while Roco runs through
    level.add_team_up_zone(900.0, 0.0, 1100.0, 400.0, 45.0);

    log_info("Level 01 (Home) loaded");
}

fn on_trigger(level, trigger_id, entity) {
    if trigger_id == "first_wall" {
        // Show a directional hint on the breakable wall
        if level.get_active_character() == "loko" {
            level.show_guide_trail(entity.x, entity.y, entity.x + 50.0, entity.y);
        }
    }

    if trigger_id == "mid_checkpoint" {
        level.spawn_pickup("star", entity.x + 50.0, entity.y + 30.0);
        level.spawn_pickup("star", entity.x + 100.0, entity.y + 30.0);
        log_info("Checkpoint reached — bonus stars spawned");
    }

    if trigger_id == "level_end" {
        level.trigger_cutscene("home_level_01_complete");
    }
}

fn update(dt) {
    // Per-frame script logic (optional — most levels leave this empty)
}
