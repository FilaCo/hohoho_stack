use bevy::prelude::*;
use hohoho_stack::HohohoPlugins;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            visible: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(HohohoPlugins);

    if cfg!(feature = "dev") {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            // .add_plugins(bevy_editor_pls::EditorPlugin::default())
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
    }

    app.run();
}
