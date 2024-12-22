use bevy::prelude::{Reflect, ReflectState, States};

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Reflect)]
#[reflect(State)]
pub enum GameState {
    InGame,
    Paused,
    GameOver,
    #[default]
    None,
}
