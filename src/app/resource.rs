use bevy::prelude::{Font, Handle, Reflect, ReflectResource, Res, Resource};
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct AppAssets {
    #[asset(key = "font")]
    pub font: Handle<Font>,
}
