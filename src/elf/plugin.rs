use crate::app::AppState;
use crate::elf::system::{handle_input, spawn_elf};
use crate::elf::{Elf, ElfAction};
use bevy::prelude::{
    App, IntoSystemConfigs, OnEnter, Plugin, RunFixedMainLoop, RunFixedMainLoopSystem, Update,
};
use leafwing_input_manager::prelude::InputManagerPlugin;

#[derive(Debug, Default)]
pub struct ElfPlugin;

impl Plugin for ElfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ElfAction>::default())
            .register_type::<Elf>()
            .add_systems(OnEnter(AppState::GameRunning), spawn_elf)
            .add_systems(
                RunFixedMainLoop,
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
            );
    }
}
