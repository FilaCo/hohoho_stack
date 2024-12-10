use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_asset_loader::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
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
    .add_loading_state(
        LoadingState::new(AppState::AppLoading)
            .continue_to_state(AppState::MenuRunning)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(APP_ASSETS_FILE_PATH)
            .load_collection::<AppAssets>(),
    )
    .add_loading_state(
        LoadingState::new(AppState::GameLoading)
            .continue_to_state(AppState::GameRunning)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(GAME_ASSETS_FILE_PATH)
            .load_collection::<GameAssets>(),
    )
    .add_systems(OnEnter(AppState::MenuRunning), (setup_menu, setup_camera))
    .add_systems(OnEnter(MenuState::Main), setup_main_menu)
    .add_systems(Update, menu_action)
    .add_systems(
        OnExit(MenuState::Main),
        despawn_by_component::<OnMainMenuScreen>,
    )
    .add_systems(OnEnter(AppState::GameRunning), setup_game)
    .add_systems(OnEnter(GameState::InGame), setup_level);

    if cfg!(feature = "dev") {
        app.add_plugins(bevy::dev_tools::ui_debug_overlay::DebugUiPlugin)
            // .add_plugins(bevy_editor_pls::EditorPlugin::default())
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
    }

    app.run();
}

const GAME_NAME: &str = "Ho Ho Ho! Stack Attack";

const MAIN_MENU_NEW_GAME_LABEL: &str = "New Game!";
const MAIN_MENU_SETTINGS_LABEL: &str = "Settings";
const MAIN_MENU_QUIT_LABEL: &str = "Quit";

/// <div style="background-color:rgb(94.9%, 43.1%, 21.2%); width: 10px; padding: 10px; border: 1px solid;"></div>
const BG_COLOR: Color = Color::srgb(0.949, 0.431, 0.212);
/// <div style="background-color:rgb(0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const FG_COLOR: Color = Color::BLACK;

const APP_ASSETS_FILE_PATH: &str = "app.assets.ron";
const GAME_ASSETS_FILE_PATH: &str = "game.assets.ron";

#[derive(Component, Debug)]
struct Elf;

#[derive(Component, Debug)]
struct Deer;

#[derive(Component, Debug)]
struct Gift;

#[derive(Component, Debug)]
#[require(Camera2d)]
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

#[derive(AssetCollection, Resource, Debug)]
struct AppAssets {
    #[asset(key = "font")]
    font: Handle<Font>,
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

#[derive(AssetCollection, Resource, Debug)]
struct GameAssets {}

#[derive(Component, Debug)]
enum MenuButtonAction {
    NewGame,
    Quit,
}

#[derive(Component, Debug)]
struct OnMainMenuScreen;

fn setup_window(mut q: Query<&mut Window>) {
    q.iter_mut().for_each(|mut window| {
        window.title = GAME_NAME.into();
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);

        window.visible = true;
    });
}

fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn setup_main_menu(mut commands: Commands, assets: Res<AppAssets>) {
    let button_node = Node {
        width: Val::Percent(75.0),
        height: Val::Percent(100.0 / 3.0),
        margin: UiRect::all(Val::Percent(2.5)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Percent(1.0)),
        ..default()
    };

    let text_font = TextFont {
        font: assets.font.clone(),
        font_size: 32.0,
        ..default()
    };

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .insert(BackgroundColor(BG_COLOR))
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            parent
                .spawn(Text::new("Ho Ho Ho! Stack Attack!"))
                .insert(text_font.clone().with_font_size(64.0))
                .insert(TextColor(FG_COLOR))
                .insert(Node {
                    margin: UiRect::all(Val::Percent(5.0)),
                    ..default()
                });

            parent
                .spawn(Node {
                    width: Val::Vw(50.0),
                    height: Val::Vh(50.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Button)
                        .insert(MenuButtonAction::NewGame)
                        .insert(button_node.clone())
                        .insert(BorderColor(FG_COLOR))
                        .with_children(|parent| {
                            parent
                                .spawn(Text::new("New Game!"))
                                .insert(text_font.clone())
                                .insert(TextColor(FG_COLOR));
                        });
                    parent
                        .spawn(Button)
                        .insert(button_node.clone())
                        .insert(BorderColor(FG_COLOR))
                        .with_children(|parent| {
                            parent
                                .spawn(Text::new("Settings"))
                                .insert(text_font.clone())
                                .insert(TextColor(FG_COLOR));
                        });
                    parent
                        .spawn(Button)
                        .insert(MenuButtonAction::Quit)
                        .insert(button_node)
                        .insert(BorderColor(FG_COLOR))
                        .with_children(|parent| {
                            parent
                                .spawn(Text::new("Quit"))
                                .insert(text_font)
                                .insert(TextColor(FG_COLOR));
                        });
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

fn setup_camera(mut commands: Commands) {
    commands.spawn(HohohoCamera);
}

fn despawn_by_component<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter()
        .for_each(|entity| commands.entity(entity).despawn_recursive())
}

fn menu_action(
    interactions: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    interactions.iter().for_each(|(interaction, action)| {
        if *interaction != Interaction::Pressed {
            return;
        }

        match action {
            MenuButtonAction::NewGame => {
                app_state.set(AppState::GameLoading);
                menu_state.set(MenuState::Disabled);
            }
            MenuButtonAction::Quit => {
                app_exit_events.send(AppExit::Success);
            }
        }
    });
}
