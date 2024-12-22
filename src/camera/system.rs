use crate::camera::HohohoCamera;
use bevy::prelude::Commands;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(HohohoCamera);
}
