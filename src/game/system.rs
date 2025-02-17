use crate::game::{GameState, Grounded, Velocity};
use crate::map::{Map, MapPosition, PreviousMapPosition};
use crate::util::from_map;
use bevy::prelude::{
    Commands, Entity, Fixed, NextState, Query, Res, ResMut, Time, Transform, Vec2, Vec3, Window,
    Without,
};

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
        pos.0 = (pos.0.as_vec2() + vel.0.as_vec2() * time.delta_secs()).as_u8vec2();

        pos.x = pos.x.clamp(0, map.width - 1);
        pos.y = pos.y.clamp(0, map.height - 1);
    });
}

pub fn apply_gravity(mut q: Query<&mut Velocity, Without<Grounded>>, time: Res<Time>) {
    const G: u8 = 10;

    q.iter_mut().for_each(|mut vel| {
        vel.0.y -= (G as f32 * time.delta_secs()) as u8;
    });
}

pub fn check_grounded(mut commands: Commands, q: Query<(Entity, Option<&Grounded>)>) {
    q.iter().for_each(|(e, grounded)| {});
}

pub fn interpolate_rendered_transform(
    mut q: Query<(&mut Transform, &MapPosition, &PreviousMapPosition)>,
    map: Res<Map>,
    time: Res<Time<Fixed>>,
    window: Query<&Window>,
) {
    let window = window.get_single().expect("window is absent");
    let scale = window.width() / map.width as f32;

    q.iter_mut().for_each(|(mut transform, pos, prev)| {
        let prev_rendered = from_map(prev, scale, &map);
        let next_rendered = from_map(pos, scale, &map);
        let rendered_translation = prev_rendered.lerp(next_rendered, time.overstep_fraction());

        transform.translation = Vec3::new(rendered_translation.x, rendered_translation.y, 0.0);
    });
}
