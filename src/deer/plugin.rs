use crate::deer::Deer;
use bevy::prelude::{App, Plugin};

#[derive(Debug, Default)]
pub struct DeerPlugin;

impl Plugin for DeerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Deer>();
    }
}
