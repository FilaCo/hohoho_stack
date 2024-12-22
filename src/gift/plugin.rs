use crate::gift::Gift;
use bevy::prelude::{App, Plugin};

#[derive(Debug, Default)]
pub struct GiftPlugin;

impl Plugin for GiftPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Gift>();
    }
}
