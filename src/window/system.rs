use bevy::prelude::{MonitorSelection, Query, Window};
use bevy::window::WindowMode;

pub fn setup_window(mut q: Query<&mut Window>) {
    q.iter_mut().for_each(|mut window| {
        // TODO: Change to i18n
        window.title = "Ho Ho Ho! Stack Attack".into();
        // window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);

        window.visible = true;
    });
}

pub fn toggle_cursor(mut q: Query<&mut Window>) {
    q.iter_mut().for_each(|mut window| {
        // window.cursor_options.visible = !window.cursor_options.visible;
    });
}
