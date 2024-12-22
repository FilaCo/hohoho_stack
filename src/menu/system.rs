use crate::app::{AppAssets, AppState};
use crate::menu::{MenuButtonAction, MenuState, OnMainMenuScreen, BG_COLOR, FG_COLOR};
use bevy::prelude::{
    default, AlignItems, AppExit, BackgroundColor, BorderColor, BuildChildren, Button, Changed,
    ChildBuild, Commands, EventWriter, FlexDirection, Interaction, JustifyContent, NextState, Node,
    Query, Res, ResMut, Text, TextColor, TextFont, UiRect, Val, With,
};

pub fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn menu_action(
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

pub fn setup_main_menu(mut commands: Commands, assets: Res<AppAssets>) {
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
                                // TODO: change to i18n
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
                                // TODO: change to i18n
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
                                // TODO: change to i18n
                                .spawn(Text::new("Quit"))
                                .insert(text_font)
                                .insert(TextColor(FG_COLOR));
                        });
                });
        });
}
