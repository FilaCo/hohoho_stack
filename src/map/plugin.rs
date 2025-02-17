use crate::app::AppState;
use crate::map::system::setup_map;
use crate::map::{Map, MapPosition, PreviousMapPosition};
use bevy::prelude::{App, OnEnter, Plugin};

#[derive(Debug, Default)]
pub struct HohohoMapPlugin;

impl Plugin for HohohoMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Map>()
            .add_systems(OnEnter(AppState::GameRunning), setup_map);
    }
}
