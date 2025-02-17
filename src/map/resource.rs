use bevy::prelude::{Reflect, ReflectResource, Resource};

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct Map {
    pub width: u8,
    pub height: u8,
}

impl Map {
    pub fn half_width(&self) -> u8 {
        self.width / 2
    }

    pub fn half_height(&self) -> u8 {
        self.height / 2
    }
}
