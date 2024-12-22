use crate::camera::system::setup_camera;
use crate::camera::HohohoCamera;
use bevy::prelude::{App, Plugin, Startup};

#[derive(Debug, Default)]
pub struct HohohoCameraPlugin;

impl Plugin for HohohoCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HohohoCamera>()
            .add_systems(Startup, setup_camera);
    }
}
