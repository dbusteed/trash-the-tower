use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::AppState;
use crate::constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

#[derive(Component)]
struct MenuNode;

#[derive(Component)]
struct GameStartBtn;

#[derive(Component)]
struct QuitBtn;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::MainMenu, setup_menu)
            .add_exit_system(AppState::MainMenu, remove_menu)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::MainMenu)
                    .with_system(btn_start_game.run_if(on_btn_interact::<GameStartBtn>))
                    .with_system(btn_quit_game.run_if(on_btn_interact::<QuitBtn>))
                    .with_system(button_system)
                    .into(),
            );
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|container| {
            container
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(50.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|title_container| {
                    title_container
                        .spawn_bundle(TextBundle {
                            text: Text::from_section(
                                "Trash the Tower!",
                                TextStyle {
                                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                                    font_size: 80.0,
                                    color: Color::rgb(0., 0., 0.),
                                },
                            ),
                            ..default()
                        })
                        .insert(MenuNode);
                });

            container
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(225.0), Val::Px(65.0)),
                        margin: UiRect::all(Val::Px(10.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "Start Game",
                            TextStyle {
                                font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..default()
                    });
                })
                .insert(MenuNode)
                .insert(GameStartBtn);

            container
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(225.0), Val::Px(65.0)),
                        margin: UiRect::all(Val::Px(10.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "Quit",
                            TextStyle {
                                font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..default()
                    });
                })
                .insert(MenuNode)
                .insert(QuitBtn);
        });
}

fn remove_menu(mut commands: Commands, query: Query<Entity, With<MenuNode>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn btn_start_game(mut commands: Commands) {
    commands.insert_resource(NextState(AppState::InGame));
}

fn btn_quit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

pub fn on_btn_interact<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => *color = PRESSED_BUTTON.into(),
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        }
    }
}
