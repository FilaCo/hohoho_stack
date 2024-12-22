use crate::game::Velocity;
use bevy::prelude::{Component, Reflect, ReflectComponent};
use leafwing_input_manager::prelude::Actionlike;

#[derive(Debug, Component, Reflect)]
#[require(Velocity)]
#[reflect(Component)]
pub struct Elf;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ElfAction {
    #[actionlike(Axis)]
    Move,
    Jump,
}
