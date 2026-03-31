// Dust Bunny — simplest enemy archetype.
// Patrols a corridor. Charges at the nearest player when close.
// Defeated in 2 hits. Poof cloud on death.

fn create(entity, difficulty) {
    entity.health        = 2;
    entity.speed         = difficulty.enemy_speed_multiplier * 60.0;
    entity.windup        = difficulty.enemy_windup_duration;
    entity.attack_timer  = entity.windup;
    entity.state         = "patrol";
}

fn update(entity, dt, player_pos) {
    let dist = (entity.x - player_pos.x).abs();

    if entity.state == "stunned" {
        entity.stun_timer -= dt;
        if entity.stun_timer <= 0.0 {
            entity.state = "patrol";
        }
        return;
    }

    if dist < 150.0 {
        entity.state = "attacking";
        entity.attack_timer -= dt;

        if entity.attack_timer <= 0.0 {
            let dir = if player_pos.x > entity.x { "right" } else { "left" };
            entity.attack(dir);
            entity.attack_timer = entity.windup;
        }
    } else {
        entity.state = "patrol";
        entity.patrol(
            entity.patrol_x1, entity.patrol_y1,
            entity.patrol_x2, entity.patrol_y2,
            entity.speed
        );
    }
}

fn on_hit(entity, damage) {
    entity.health -= 1;
    entity.state       = "stunned";
    entity.stun_timer  = 0.4;

    if entity.health <= 0 {
        entity.die_with_effect("poof_cloud");
    }
}
