use crate::elf::{Elf, ElfAction};
use crate::game::{Speed, Velocity};
use crate::map::{Map, MapPosition, PreviousMapPosition};
use bevy::prelude::{
    info, Assets, Capsule2d, Circle, Color, ColorMaterial, Commands, Entity, KeyCode, Mesh, Mesh2d,
    MeshMaterial2d, Query, Rect, Rectangle, Res, ResMut, Transform, Vec2, Window, With,
};
use leafwing_input_manager::prelude::{
    ActionState, InputManagerBundle, InputMap, VirtualAxis, VirtualDPad,
};

pub fn spawn_elf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q: Query<Entity, With<Elf>>,
    map: Res<Map>,
    window: Query<&Window>,
) {
    if !q.is_empty() {
        return;
    }

    const ELF_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);

    let window = window.get_single().expect("window is absent");
    let scale = window.width() / map.width;

    // TODO: acquire input map from settings
    let input_map = InputMap::default()
        .with_axis(
            ElfAction::Move,
            VirtualAxis::new(KeyCode::KeyA, KeyCode::KeyD),
        )
        .with(ElfAction::Jump, KeyCode::ArrowUp);

    let elf_pos = Vec2::new(map.width / 2.0, map.height - 1.0);
    let elf_speed = Vec2::new(1.0, 2.0);

    commands
        .spawn(Elf)
        .insert(MapPosition(elf_pos))
        .insert(PreviousMapPosition(elf_pos))
        .insert(Speed(elf_speed))
        .insert(InputManagerBundle::with_map(input_map))
        .insert(Transform::default())
        .insert(Mesh2d(meshes.add(Rectangle::new(scale, scale))))
        .insert(MeshMaterial2d(materials.add(ELF_COLOR)));
}

pub fn handle_input(mut q: Query<(&mut Velocity, &Speed, &ActionState<ElfAction>), With<Elf>>) {
    q.iter_mut().for_each(|(mut vel, speed, action_state)| {
        info!("{}", action_state.value(&ElfAction::Move));
        vel.0 += action_state.value(&ElfAction::Move) * speed.x
    })
}
