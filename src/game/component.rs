use bevy::math::U8Vec2;
use bevy::prelude::{Component, Deref, DerefMut};

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub U8Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Speed(pub U8Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default)]
#[component(storage = "SparseSet")]
pub struct Grounded;
