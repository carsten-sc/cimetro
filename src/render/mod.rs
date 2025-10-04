use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        // Hier kommen später deine Systeme rein
        info!("RenderPlugin loaded");
    }
}
