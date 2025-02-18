use crate::app::AppState;
use crate::game::system::{check_grounded, handle_input, setup_game, setup_level};
use crate::game::{ElfAction, GameState};
use bevy::prelude::{
    App, AppExtStates, IntoSystemConfigs, OnEnter, Plugin, RunFixedMainLoop, RunFixedMainLoopSystem,
};
use leafwing_input_manager::prelude::InputManagerPlugin;

#[derive(Debug, Default)]
pub struct HohohoGamePlugin;

impl Plugin for HohohoGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ElfAction>::default())
            .init_state::<GameState>()
            .add_systems(OnEnter(AppState::GameRunning), setup_game)
            .add_systems(OnEnter(GameState::InGame), setup_level)
            .add_systems(
                RunFixedMainLoop,
                (
                    handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                    check_grounded.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
                ),
            );
    }
}
