use bevy::prelude::*;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        // Hier kommen später deine Systeme rein
        info!("SimulationPlugin geladen");
    }
}

pub mod economy;
pub use economy::EconomyPlugin;

