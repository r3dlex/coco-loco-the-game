// Boss: The Giant Vacuum Cleaner
// Home Phase boss. Sucks players toward it, charges across the room.
// Sits down for free hits every 15s (Toddler) or timed rest windows (Kid).

fn create(boss, difficulty, co_op_active) {
    boss.max_health = 500.0;

    if co_op_active {
        let scale = if difficulty.mode == "toddler" { 1.25 } else { 1.5 };
        boss.max_health *= scale;
    }

    boss.health          = boss.max_health;
    boss.phase           = 1;
    boss.rest_interval   = difficulty.boss_rest_interval;
    boss.rest_timer      = boss.rest_interval;
    boss.attack_speed    = 1.0;
    boss.state           = "idle";
}

fn update(boss, dt, players) {
    // Rest phase check
    if boss.rest_interval > 0.0 {
        boss.rest_timer -= dt;
        if boss.rest_timer <= 0.0 {
            boss.state      = "resting";
            boss.rest_timer = 3.0;   // sit for 3 seconds
            boss.play_animation("dizzy_sit");
            return;
        }
    }

    if boss.state == "resting" {
        boss.rest_timer -= dt;
        if boss.rest_timer <= 0.0 {
            boss.state      = "idle";
            boss.rest_timer = boss.rest_interval;
        }
        return;
    }

    // Phase 2 transition at 50% health
    let health_pct = boss.health / boss.max_health;
    if health_pct < 0.5 && boss.phase == 1 {
        boss.phase        = 2;
        boss.attack_speed *= 1.3;
        boss.play_animation("phase_transition");
        log_info("Boss entered Phase 2!");
    }

    // Attack: target nearest player
    let target = boss.nearest_player(players);
    boss.attack_toward(target.x, target.y);
}

fn on_hit(boss, damage) {
    boss.health -= damage;
    if boss.health <= 0.0 {
        boss.die_with_effect("explosion_big");
        log_info("Boss defeated!");
    }
}

fn on_p2_joined(boss, difficulty) {
    let scale = if difficulty.mode == "toddler" { 1.25 } else { 1.5 };
    let extra = boss.max_health * (scale - 1.0);
    boss.max_health += extra;
    boss.health     += extra;
    log_info("Co-op scaling applied to boss health");
}
