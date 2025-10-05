use bevy::prelude::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        // Hier kommen später deine Systeme rein
        info!("ConfigPlugin loaded");
    }
}

pub mod user_settings;
pub use user_settings::{UserSettings, GameMode};
// ConfigPlugin wird hier nicht aus user_settings re-exportiert, sondern direkt aus mod.rs