use crate::app::AppState;
use crate::menu::system::{menu_action, setup_main_menu, setup_menu};
use crate::menu::{MenuButtonAction, MenuState, OnMainMenuScreen};
use crate::util::despawn_by_component;
use bevy::prelude::{App, AppExtStates, OnEnter, OnExit, Plugin, Update};

#[derive(Debug, Default)]
pub struct HohohoMenuPlugin;

impl Plugin for HohohoMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OnMainMenuScreen>()
            .register_type::<MenuButtonAction>()
            .register_type::<MenuState>()
            .init_state::<MenuState>()
            .add_systems(OnEnter(AppState::MenuRunning), setup_menu)
            .add_systems(OnEnter(MenuState::Main), setup_main_menu)
            .add_systems(
                OnExit(MenuState::Main),
                despawn_by_component::<OnMainMenuScreen>,
            )
            .add_systems(Update, menu_action);
    }
}
