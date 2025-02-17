use crate::map::Map;
use bevy::prelude::Commands;

pub fn setup_map(mut commands: Commands) {
    const MAP_WIDTH: u8 = 16;
    const MAP_HEIGHT: u8 = 9;

    commands.insert_resource(Map {
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
    });
}
