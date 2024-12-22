use bevy::prelude::{Component, Deref, DerefMut, Reflect, ReflectComponent, Vec2};

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct Speed(pub Vec2);
