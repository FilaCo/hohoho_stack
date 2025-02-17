use crate::map::{Map, MapPosition};
use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, UVec2, Vec2, With};

pub fn despawn_by_component<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive())
}

pub fn to_map(render_pos: &Vec2, scale: f32, map: &Map) -> MapPosition {
    let mut unscaled = render_pos / scale;

    unscaled.x += map.half_width() as f32;
    unscaled.y += map.half_height() as f32;

    MapPosition(UVec2::new(
        (unscaled.x + map.half_width() as f32) as u8,
        (unscaled.y + map.half_height() as f32) as u32,
    ))
}

pub fn from_map(map_pos: &UVec2, scale: f32, map: &Map) -> Vec2 {
    Vec2::new(
        (map_pos.x - map.half_width() as u32) as f32,
        (map_pos.y - map.half_height() as u32) as f32,
    ) * scale
}
