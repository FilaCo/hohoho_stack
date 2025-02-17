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

    #[cfg(feature = "dev")]
    {
        use bevy::dev_tools::ui_debug_overlay::DebugUiPlugin;
        use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
        use bevy_editor_pls::EditorPlugin;

        app.add_plugins(DebugUiPlugin)
            .add_plugins(EditorPlugin::default())
            .add_plugins(FrameTimeDiagnosticsPlugin);
    }

    app.run();
}
