use bevy::prelude::*;
use super::events::*;

pub fn register_events(app: &mut App) {
    app.add_event::<TrafficCongestion>()
       .add_event::<VehicleSpawned>()
       .add_event::<PopulationChanged>()
       .add_event::<BudgetUpdated>();
}