use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    AppLoading,
    MenuRunning,
    GameLoading,
    GameRunning,
    AppClosing,
}
