use crate::map::Map;
use bevy::prelude::Commands;

pub fn setup_map(mut commands: Commands) {
    const MAP_WIDTH: f32 = 16.0;
    const MAP_HEIGHT: f32 = 9.0;

    commands.insert_resource(Map {
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
    });
}
