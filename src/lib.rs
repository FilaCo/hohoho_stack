use bevy::app::plugin_group;

pub mod app;
pub mod camera;
pub mod deer;
pub mod elf;
pub mod game;
pub mod gift;
pub mod map;
pub mod menu;
pub mod util;
pub mod window;

plugin_group! {
    #[derive(Debug, Default)]
    pub struct HohohoPlugins {
        app:::HohohoAppPlugin,
        camera:::HohohoCameraPlugin,
        deer:::DeerPlugin,
        elf:::ElfPlugin,
        game:::HohohoGamePlugin,
        gift:::GiftPlugin,
        map:::HohohoMapPlugin,
        menu:::HohohoMenuPlugin,
        window:::HohohoWindowPlugin,
    }
}
