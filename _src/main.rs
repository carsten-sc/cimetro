use bevy::prelude::*;

// Module-Deklarationen
mod engine;
mod simulation;
mod ai;
mod assets;
mod render;
mod ui;
mod config;

// Plugin-Typen importieren
use engine::EnginePlugin;
use simulation::SimulationPlugin;
use ai::AiPlugin;
use assets::AssetPlugin;
use render::RenderPlugin;
use ui::UiPlugin;
use config::ConfigPlugin;

fn main() {
    App::new()
        // Bevy-Basisplugins (Fenster, Input, Renderer etc.)
        .add_plugins(DefaultPlugins)

        // Eigene Plugins als Tupel hinzufügen
        .add_plugins((
            EnginePlugin,
            SimulationPlugin,
            AiPlugin,
            AssetPlugin,
            RenderPlugin,
            UiPlugin,
            ConfigPlugin,
        ))

        // Anwendung starten
        .run();
}
