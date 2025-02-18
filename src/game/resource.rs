use crate::game::MapPosition;
use bevy::math::U8Vec2;
use bevy::prelude::{Reflect, ReflectResource, Resource, Vec2};
use bevy_asset_loader::prelude::AssetCollection;
use std::ops::{Index, IndexMut};

#[derive(AssetCollection, Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct GameAssets {}

#[derive(Resource, Debug, Clone)]
pub struct Map {
    width: u8,
    height: u8,
    entries: Vec<Entry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Entry {
    #[default]
    Empty,
    Elf,
    Gift,
    Deer,
}

impl Map {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            entries: vec![Entry::Empty; (width * height) as usize],
        }
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn raycast(&self, start: &MapPosition, dir: &Vec2) -> (MapPosition, Option<MapPosition>) {
        let start_f32 = start.as_vec2();

        let angle = dir.dot(Vec2::X) / dir.length();
        let mut actual_target = *start;
        let mut cur = start_f32;
        let mut cur_pos = MapPosition(cur.as_u8vec2());
        let mut idx = 0.0;
        loop {
            cur.x = start_f32.x + angle.cos() * idx as f32;
            cur.y = start_f32.y + angle.sin() * idx as f32;

            if cur.x < 0.0 || cur.y < 0.0 {
                return (actual_target, None);
            }

            cur_pos = MapPosition(cur.as_u8vec2());

            if self.idx_by_pos(&cur_pos) >= self.entries.len() {
                return (actual_target, None);
            }

            if self[&cur_pos] != Entry::Empty {
                return (actual_target, Some(cur_pos));
            }

            idx += 0.05;
            actual_target = cur_pos;
        }
    }

    fn idx_by_pos(&self, position: &MapPosition) -> usize {
        position.x as usize + position.y as usize * self.width as usize
    }
}

impl Index<&MapPosition> for Map {
    type Output = Entry;

    fn index(&self, index: &MapPosition) -> &Self::Output {
        &self.entries[self.idx_by_pos(index)]
    }
}

impl IndexMut<&MapPosition> for Map {
    fn index_mut(&mut self, index: &MapPosition) -> &mut Self::Output {
        let idx = self.idx_by_pos(index);

        &mut self.entries[idx]
    }
}
