use bevy::prelude::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        // Hier kommen später deine Systeme rein
        info!("AssetPlugin loaded");
    }
}
