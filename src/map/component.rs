use bevy::prelude::{Component, Deref, DerefMut, Reflect, ReflectComponent, Vec2};

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect)]
pub struct MapPosition(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect)]
pub struct PreviousMapPosition(pub Vec2);
