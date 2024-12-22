use bevy::prelude::{Reflect, ReflectResource, Resource};

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct Map {
    pub width: f32,
    pub height: f32,
}
