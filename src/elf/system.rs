use crate::elf::{Elf, ElfAction};
use crate::game::{Grounded, Speed, Velocity};
use crate::map::{Map, MapPosition};
use bevy::math::U8Vec2;
use bevy::prelude::{Commands, KeyCode, Query, Res, With};
use leafwing_input_manager::prelude::{ActionState, InputManagerBundle, InputMap, VirtualAxis};

pub fn spawn_elf(mut commands: Commands, map: Res<Map>) {
    // TODO: acquire input map from settings
    let input_map = InputMap::default()
        .with_axis(
            ElfAction::Move,
            VirtualAxis::new(KeyCode::KeyA, KeyCode::KeyD),
        )
        .with(ElfAction::Jump, KeyCode::Space);

    let elf_pos = MapPosition(U8Vec2::new(map.width / 2, 0));
    let elf_speed = Speed(U8Vec2::new(4, 4));

    commands
        .spawn(Elf)
        .insert(elf_pos)
        .insert(elf_speed)
        .insert(InputManagerBundle::with_map(input_map));
}

pub fn handle_input(
    mut q: Query<(&mut Velocity, &Speed, &ActionState<ElfAction>), (With<Elf>, With<Grounded>)>,
) {
    q.iter_mut().for_each(|(mut vel, speed, action_state)| {
        vel.x = (action_state.value(&ElfAction::Move) * speed.x as f32) as u8;

        if action_state.pressed(&ElfAction::Jump) {
            vel.y = speed.y
        }
    })
}
