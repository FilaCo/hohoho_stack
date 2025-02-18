use crate::game::{
    Elf, ElfAction, Entry, GameState, Grounded, Map, MapPosition, PreviousMapPosition, Speed,
    Velocity,
};
use bevy::math::U8Vec2;
use bevy::prelude::{Commands, Entity, KeyCode, NextState, Query, Res, ResMut, Time, Vec2, With};
use leafwing_input_manager::prelude::{ActionState, InputMap, VirtualAxis};
use leafwing_input_manager::InputManagerBundle;

pub fn setup_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::InGame);
}

pub fn setup_level(mut commands: Commands) {
    const MAP_WIDTH: u8 = 16;
    const MAP_HEIGHT: u8 = 9;

    let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT);

    let elf_pos = MapPosition(U8Vec2::new(map.width() / 2, 0));
    let input_map = InputMap::default()
        .with_axis(
            ElfAction::Move,
            VirtualAxis::new(KeyCode::KeyA, KeyCode::KeyD),
        )
        .with(ElfAction::Jump, KeyCode::Space);

    map[&elf_pos] = Entry::Elf;
    commands
        .spawn(Elf)
        .insert(elf_pos)
        .insert(InputManagerBundle::with_map(input_map))
        .insert(Speed(U8Vec2::new(4, 2)));

    commands.insert_resource(map);
}

pub fn check_grounded(
    mut commands: Commands,
    q: Query<(Entity, &MapPosition, Option<&Grounded>)>,
    map: Res<Map>,
) {
    q.iter().for_each(|(entity, pos, grounded)| {
        let is_grounded = map.raycast(pos, &(Vec2::NEG_Y - pos.as_vec2())).1.is_some();

        if is_grounded && grounded.is_none() {
            commands.entity(entity).insert(Grounded);
        } else if !is_grounded && grounded.is_some() {
            commands.entity(entity).remove::<Grounded>();
        }
    });
}

pub fn handle_input(
    mut q: Query<(&mut Velocity, &Speed, &ActionState<ElfAction>), With<Grounded>>,
) {
    q.iter_mut().for_each(|(mut vel, speed, action_state)| {
        vel.x = (action_state.value(&ElfAction::Move) * speed.x as f32) as u8;

        if action_state.pressed(&ElfAction::Jump) {
            vel.y = speed.y
        }
    })
}

pub fn advance_game(
    mut q: Query<(&mut MapPosition, &mut PreviousMapPosition, &Velocity)>,
    mut map: ResMut<Map>,
    time: Res<Time>,
) {
    q.iter_mut().for_each(|(mut pos, mut prev_pos, vel)| {
        let start_pos = MapPosition(pos.0);
        let target_pos = MapPosition(pos.0 + (vel.0.as_vec2() * time.delta_secs()).as_u8vec2());

        prev_pos.0 = start_pos.0;
        let (actual_target_pos, obstacle_pos) =
            map.raycast(&start_pos, &(target_pos.0 - start_pos.0).as_vec2());

        *pos = match obstacle_pos {
            Some(_) => actual_target_pos,
            None => target_pos,
        };

        map[&start_pos] = Entry::Empty;
    })
}
