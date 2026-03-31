use bevy::prelude::*;
use std::path::PathBuf;

use crate::resources::difficulty::DifficultyConfig;

pub struct SavePlugin;

/// Persisted save data (local file, no cloud).
#[derive(Resource, Default)]
pub struct SaveData {
    pub difficulty_mode: SavedDifficulty,
    pub unlocked_phases: Vec<String>,
    pub last_level: Option<(String, u32)>,
}

#[derive(Default)]
pub enum SavedDifficulty {
    #[default]
    Toddler,
    Kid,
}

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveData>();
        app.add_systems(Startup, load_save);
    }
}

fn save_path() -> PathBuf {
    // Platform-appropriate save location — using local dir for simplicity
    PathBuf::from("save.bin")
}

fn load_save(mut commands: Commands, save: ResMut<SaveData>) {
    let path = save_path();
    if path.exists() {
        // TODO: deserialize with serde/bincode once those deps are added
        info!("Save file found at {:?} — loading…", path);
    } else {
        info!("No save file found. Starting fresh.");
    }

    // Apply saved difficulty to the DifficultyConfig resource
    let difficulty = match save.difficulty_mode {
        SavedDifficulty::Toddler => DifficultyConfig::toddler(),
        SavedDifficulty::Kid => DifficultyConfig::kid(),
    };
    commands.insert_resource(difficulty);
}
