use bevy::prelude::{Font, Handle, Res, Resource};
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource, Debug)]
pub struct AppAssets {
    #[asset(key = "font")]
    pub font: Handle<Font>,
}
