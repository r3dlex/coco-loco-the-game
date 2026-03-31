// Home Phase configuration
// Bedroom, living room, kitchen. Everyday objects are giant obstacles.

fn phase_config() {
    #{
        name:        "Home Phase",
        description: "The adventure starts at home. Giant furniture, dusty enemies, and a warm glow.",
        gravity:     1.0,

        parallax_layers: 3,

        post_process: #{
            bloom:    0.6,
            lut:      "luts/home_warm.png",
            vignette: 0.2,
        },

        // Co-op boss scaling
        boss_co_op_scale_toddler: 1.25,
        boss_co_op_scale_kid:     1.5,

        // Levels in order
        levels: [
            "levels/home/level_01",
            "levels/home/level_02",
            "levels/home/level_03",
            "levels/home/boss",
        ],

        music_exploration: "home_cozy",
        music_action:      "home_action",
        music_boss:        "home_boss",
    }
}
