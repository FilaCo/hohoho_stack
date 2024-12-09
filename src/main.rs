use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .init_state::<MenuState>()
        .init_state::<GameState>()
        .add_systems(Startup, setup_window)
        .add_systems(OnEnter(AppState::MenuRunning), setup_menu)
        .add_systems(OnEnter(MenuState::Main), setup_main_menu)
        .add_systems(OnEnter(AppState::GameRunning), setup_game)
        .add_systems(OnEnter(GameState::InGame), setup_level)
        .run();
}

const GAME_NAME: &str = "Ho Ho Ho! Stack Attack!";

const MAIN_MENU_NEW_GAME_LABEL: &str = "New Game!";
const MAIN_MENU_SETTINGS_LABEL: &str = "Settings";
const MAIN_MENU_QUIT_LABEL: &str = "Quit";

/// <div style="background-color:rgb(94.9%, 43.1%, 21.2%); width: 10px; padding: 10px; border: 1px solid;"></div>
const BG_COLOR: Srgba = Srgba::rgb(242.0, 110.0, 54.0);
/// <div style="background-color:rgb(0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const FG_COLOR: Srgba = Srgba::BLACK;

#[derive(Component, Debug)]
struct Elf;

#[derive(Component, Debug)]
struct Deer;

#[derive(Component, Debug)]
struct Gift;

#[derive(Component, Debug)]
struct HohohoCamera;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    AppLoading,
    MenuRunning,
    GameLoading,
    GameRunning,
    AppClosing,
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum MenuState {
    Main,
    #[default]
    Disabled,
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum GameState {
    InGame,
    Paused,
    GameOver,
    #[default]
    None,
}

fn setup_window(mut q: Query<&mut Window>) {
    q.iter_mut().for_each(|mut window| {
        window.title = GAME_NAME.into();
        window.visible = true
    });
}

fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(50.0),
                    height: Val::Percent(75.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(Button).insert(Node { ..default() });
                    parent.spawn(Button).insert(Node { ..default() });
                    parent.spawn(Button).insert(Node { ..default() });
                });
        });
}

fn setup_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::InGame);
}

fn setup_level(mut commands: Commands) {
    // spawn elf
    commands.spawn(Elf);
}

fn despawn_by_component<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive())
}
