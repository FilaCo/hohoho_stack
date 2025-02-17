use bevy::math::U8Vec2;
use bevy::prelude::{Component, Deref, DerefMut};

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct MapPosition(pub U8Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousMapPosition(pub U8Vec2);
