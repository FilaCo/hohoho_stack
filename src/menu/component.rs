use bevy::prelude::{Component, Reflect, ReflectComponent};
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct OnMainMenuScreen;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub enum MenuButtonAction {
    NewGame,
    Quit,
}
