use bevy::app::plugin_group;

pub mod app;
pub mod camera;
pub mod game;
pub mod menu;
pub mod util;
pub mod window;

plugin_group! {
    #[derive(Debug, Default)]
    pub struct HohohoPlugins {
        app:::HohohoAppPlugin,
        camera:::HohohoCameraPlugin,
        game:::HohohoGamePlugin,
        menu:::HohohoMenuPlugin,
        window:::HohohoWindowPlugin,
    }
}
