use crate::game::{GameState, Velocity};
use crate::map::{Map, MapPosition, PreviousMapPosition};
use bevy::prelude::{Fixed, NextState, Query, Res, ResMut, Time, Transform, Vec3, Window};

pub fn setup_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::InGame);
}

pub fn advance_game(
    mut q: Query<(&mut MapPosition, &mut PreviousMapPosition, &Velocity)>,
    map: Res<Map>,
    time: Res<Time>,
) {
    q.iter_mut().for_each(|(mut pos, mut prev, vel)| {
        prev.0 = pos.0;
        pos.0 += vel.0 * time.delta_secs();

        pos.x = pos.x.clamp(0.0, map.width - 1.0);
        pos.y = pos.y.clamp(0.0, map.height - 1.0);
    });
}

pub fn interpolate_rendered_transform(
    mut q: Query<(&mut Transform, &MapPosition, &PreviousMapPosition)>,
    map: Res<Map>,
    time: Res<Time<Fixed>>,
    window: Query<&Window>,
) {
    let window = window.get_single().expect("window is absent");
    let scale = window.width() / map.width;

    q.iter_mut().for_each(|(mut transform, pos, prev)| {
        let prev_rendered = prev.0 * scale;
        let next_rendered = pos.0 * scale;
        let rendered_translation = prev_rendered.lerp(next_rendered, time.overstep_fraction());

        transform.translation = Vec3::new(rendered_translation.x, rendered_translation.y, 0.0);
    });
}
