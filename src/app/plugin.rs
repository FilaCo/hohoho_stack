use crate::app::{AppAssets, AppState, ASSETS_FILE_PATH};
use crate::game::{GameAssets, ASSETS_FILE_PATH as GAME_ASSETS_FILE_PATH};
use bevy::prelude::{App, AppExtStates, Plugin};
use bevy_asset_loader::prelude::{
    ConfigureLoadingState, LoadingState, LoadingStateAppExt, StandardDynamicAssetCollection,
};

#[derive(Debug, Default)]
pub struct HohohoAppPlugin;

impl Plugin for HohohoAppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::AppLoading)
                    .continue_to_state(AppState::MenuRunning)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>(ASSETS_FILE_PATH)
                    .load_collection::<AppAssets>(),
            )
            .add_loading_state(
                LoadingState::new(AppState::GameLoading)
                    .continue_to_state(AppState::GameRunning)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                        GAME_ASSETS_FILE_PATH,
                    )
                    .load_collection::<GameAssets>(),
            );
    }
}
