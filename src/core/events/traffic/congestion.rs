use bevy::prelude::*;

#[derive(Event)]
pub struct TrafficCongestion {
    pub location: (i32, i32),
    pub severity: u8,
}