use bevy::prelude::*;
use std::time::Duration;

/// Resource that holds the simulation time state.
#[derive(Resource)]
pub struct SimulationTime {
    pub tick: u64,          // Number of ticks since simulation start
    pub time_of_day: f32,   // Time of day in hours (e.g. 13.5 = 13:30)
    pub tick_interval: Duration, // Duration between ticks
}

impl Default for SimulationTime {
    fn default() -> Self {
        SimulationTime {
            tick: 0,
            time_of_day: 6.0, // Start at 6:00 AM
            tick_interval: Duration::from_millis(100),
        }
    }
}

/// System that advances the simulation time.
pub fn tick_scheduler_system(
    time: Res<Time>,
    mut sim_time: ResMut<SimulationTime>,
) {
    // Advance tick based on Bevy's delta time
    let delta = time.delta();

    // Only advance if enough time has passed
    if delta >= sim_time.tick_interval {
        sim_time.tick += 1;
        sim_time.time_of_day += 0.01; // Advance time of day (tune as needed)

        if sim_time.time_of_day >= 24.0 {
            sim_time.time_of_day = 0.0; // Wrap around after midnight
        }

        // Optional: print debug info
        info!(
            "Tick: {}, Time of Day: {:.2}h",
            sim_time.tick, sim_time.time_of_day
        );
    }
}
