use bevy::prelude::*;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        info!("EnginePlugin loaded");
    }
}