use crate::elf::system::{handle_input, spawn_elf};
use crate::elf::Elf;
use bevy::prelude::{App, Plugin, Update};

#[derive(Debug, Default)]
pub struct ElfPlugin;

impl Plugin for ElfPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Elf>()
            .add_systems(Update, (spawn_elf, handle_input));
    }
}
