use bevy::prelude::{Reflect, ReflectState, States};

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Reflect)]
#[reflect(State)]
pub enum AppState {
    #[default]
    AppLoading,
    MenuRunning,
    GameLoading,
    GameRunning,
    AppClosing,
}
