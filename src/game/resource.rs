use bevy::prelude::{Reflect, ReflectResource, Resource};
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct GameAssets {}
