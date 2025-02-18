use bevy::prelude::Component;
#[derive(Component, Debug)]
pub struct OnMainMenuScreen;

#[derive(Component, Debug)]
pub enum MenuButtonAction {
    NewGame,
    Quit,
}
