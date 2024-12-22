use bevy::prelude::{Camera2d, Component, Reflect, ReflectComponent};

#[derive(Component, Debug, Reflect)]
#[require(Camera2d)]
#[reflect(Component)]
pub struct HohohoCamera;
