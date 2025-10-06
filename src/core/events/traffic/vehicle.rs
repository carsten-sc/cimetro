use bevy::prelude::*;

#[derive(Event)]
pub struct VehicleSpawned {
    pub vehicle_type: String,
    pub position: (i32, i32),
}
