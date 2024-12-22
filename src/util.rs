use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, With};

pub fn despawn_by_component<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive())
}
