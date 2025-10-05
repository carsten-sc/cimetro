// core module

pub mod scheduler;
pub mod event_bus;
pub mod config;
pub mod rng;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SimulationTime::default())
            .add_system(tick_scheduler_system.label("scheduler"))
            .add_system(dispatch_events_system.after("scheduler"))
            .add_system(load_config_system.before("scheduler"))
            .add_system(seed_rng_system);
    }
}

