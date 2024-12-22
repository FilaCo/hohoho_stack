use bevy::prelude::{Component, Reflect, ReflectComponent};

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Gift;
