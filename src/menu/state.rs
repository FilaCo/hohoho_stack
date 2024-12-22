use bevy::prelude::{Reflect, ReflectState, States};

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Reflect)]
#[reflect(State)]
pub enum MenuState {
    Main,
    #[default]
    Disabled,
}
