use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub enum GameMode {
    #[default]
    Normal,
    Sandbox,
}

#[derive(Resource, Serialize, Deserialize, PartialEq, Clone)]
pub struct UserSettings {
    pub fullscreen: bool,
    pub volume: f32,
    pub game_mode: GameMode,
}

// Laden
pub fn load_settings(path: &str) -> UserSettings {
    fs::read_to_string(path)
        .ok()
        .and_then(|data| toml::from_str(&data).ok())
        .unwrap_or_default()
}

// Speichern
pub fn save_settings(path: &str, settings: &UserSettings) {
    if let Ok(data) = toml::to_string(settings) {
        let _ = fs::write(path, data);
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            fullscreen: false,
            volume: 0.8,
            game_mode: GameMode::Normal,
        }
    }
}

// Beispiel-Plugin, das die Settings lädt und als Resource einfügt
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config_dir = dirs::config_dir()
            .unwrap()
            .join("citysim");
        let settings_path = config_dir.join("settings.toml");
        let settings = load_settings(settings_path.to_str().unwrap());
        app.insert_resource(settings);
    }
}