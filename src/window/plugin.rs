use crate::app::AppState;
use crate::window::system::{setup_window, toggle_cursor};
use bevy::prelude::{App, OnEnter, OnExit, Plugin, Startup};

#[derive(Debug, Default)]
pub struct HohohoWindowPlugin;

impl Plugin for HohohoWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_window)
            .add_systems(OnEnter(AppState::GameRunning), toggle_cursor)
            .add_systems(OnExit(AppState::GameRunning), toggle_cursor);
    }
}
