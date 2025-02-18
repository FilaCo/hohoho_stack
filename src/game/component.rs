use bevy::math::U8Vec2;
use bevy::prelude::{Component, Deref, DerefMut, Reflect};
use leafwing_input_manager::Actionlike;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub U8Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Speed(pub U8Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Debug, Component)]
#[require(MapPosition)]
pub struct Elf;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ElfAction {
    #[actionlike(Axis)]
    Move,
    Jump,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
#[require(MapPosition)]
pub struct MapPosition(pub U8Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousMapPosition(pub U8Vec2);
