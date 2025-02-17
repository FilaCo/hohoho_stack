use crate::app::AppState;
use crate::game::system::{
    advance_game, apply_gravity, interpolate_rendered_transform, setup_game,
};
use crate::game::{GameState, Speed, Velocity};
use bevy::prelude::{
    App, AppExtStates, FixedUpdate, IntoSystemConfigs, OnEnter, Plugin, RunFixedMainLoop,
    RunFixedMainLoopSystem,
};

#[derive(Debug, Default)]
pub struct HohohoGamePlugin;

impl Plugin for HohohoGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameState>()
            .init_state::<GameState>()
            .add_systems(OnEnter(AppState::GameRunning), setup_game)
            .add_systems(FixedUpdate, (advance_game, apply_gravity))
            .add_systems(
                RunFixedMainLoop,
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            );
    }
}
