use bevy::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use super::AppState;
use crate::constants::{
    GROUND_COLOR, GROUND_HEIGHT, LAUNCH_FACTOR, STONE1, STONE2, TARGET_COLOR, TARGET_FORCE_THRESH,
    WOOD1, WOOD2,
};
use crate::read_levels::read_levels;

#[derive(PartialEq, Debug)]
enum LevelState {
    Prelaunch,
    Launched,
    Complete,
    LastLevelComplete,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct LevelText;

#[derive(Component)]
struct PowerIndicator;

#[derive(Component)]
struct GameNode;

#[derive(Component)]
struct LevelNode;

struct Power(f32);
struct MaxLevel(usize);

struct Game {
    state: LevelState,
    level: usize,
}

struct LaunchEvent {
    power: f32,
    target: Vec2,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaunchEvent>()
            .insert_resource(Game {
                state: LevelState::Prelaunch,
                level: 0,
            })
            .insert_resource(Power(0.))
            .add_enter_system(AppState::InGame, setup_game)
            .add_exit_system(AppState::InGame, remove_game)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(keyboard_listener)
                    .with_system(power_indicator.run_if(is_prelaunch))
                    .with_system(launch.run_if(is_prelaunch))
                    .with_system(target_collisions.run_if(is_launched))
                    .with_system(level_complete.run_if(is_launched))
                    .into(),
            );
    }
}

fn is_prelaunch(game: Res<Game>) -> bool {
    game.state == LevelState::Prelaunch
}

fn is_launched(game: Res<Game>) -> bool {
    game.state == LevelState::Launched
}

fn setup_game(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    game: ResMut<Game>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    let level_data = read_levels();
    commands.insert_resource(MaxLevel(level_data.len() - 1));

    // ground
    commands
        .spawn()
        .insert_bundle(lyon::GeometryBuilder::build_as(
            &lyon::shapes::Rectangle {
                extents: Vec2::new(win_w, GROUND_HEIGHT * 2.),
                origin: lyon::shapes::RectangleOrigin::Center,
            },
            lyon::DrawMode::Outlined {
                fill_mode: lyon::FillMode::color(GROUND_COLOR),
                outline_mode: lyon::StrokeMode::color(GROUND_COLOR),
            },
            Transform::default(),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(win_w / 2., GROUND_HEIGHT))
        .insert(Transform::from_xyz(0.0, -win_h / 2., 0.0))
        .insert(GameNode);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.),
                    right: Val::Px(15.),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "Press \"r\" to restart".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(GameNode);

    setup_level(commands, windows, game, asset_server);
}

fn setup_level(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
) {
    let level_data = read_levels();

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    game.state = LevelState::Prelaunch;

    let level_label = format!(
        "Level {} / {}",
        game.level.to_string(),
        level_data.len() - 1
    );
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.),
                    left: Val::Px(8.),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                level_label,
                TextStyle {
                    font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            ),
            ..default()
        })
        .insert(LevelNode)
        .insert(LevelText);

    // ball
    commands
        .spawn()
        .insert_bundle(lyon::GeometryBuilder::build_as(
            &lyon::shapes::Circle {
                radius: 10.,
                center: Vec2::ZERO,
            },
            lyon::DrawMode::Outlined {
                fill_mode: lyon::FillMode::color(Color::BLACK),
                outline_mode: lyon::StrokeMode::color(Color::BLACK),
            },
            Transform::default(),
        ))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(10.))
        .insert(Restitution::coefficient(0.7))
        .insert(ExternalImpulse::default())
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Transform::from_xyz(
            -win_w / 4.,
            (-win_h / 2.0) + (30. / 2.) + GROUND_HEIGHT,
            0.0,
        ))
        .insert(LevelNode)
        .insert(Ball);

    // help text for first level
    if game.level == 0 {
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(50.),
                        left: Val::Px(8.),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    "1. Aim with mouse\n2. Click, hold, and release \nRight Mouse Button to launch"
                        .to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            })
            .insert(LevelNode)
            .insert(LevelText);
    }

    // spawn the tower
    for node in level_data[game.level].tower.iter() {
        let mat = match node.kind.as_str() {
            "wood1" => WOOD1,
            "wood2" => WOOD2,
            "stone1" => STONE1,
            "stone2" => STONE2,
            _ => WOOD1,
        };

        commands
            .spawn()
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Rectangle {
                    extents: Vec2::new(node.w, node.h),
                    origin: lyon::shapes::RectangleOrigin::Center,
                },
                lyon::DrawMode::Outlined {
                    fill_mode: lyon::FillMode::color(mat.color1),
                    outline_mode: lyon::StrokeMode::new(mat.color2, 2.),
                },
                Transform::default(),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(node.w / 2., node.h / 2.))
            .insert(ColliderMassProperties::Density(mat.density))
            .insert(Transform::from_xyz(
                (win_w / 4.) + node.x + (node.w / 2.),
                (-win_h / 2.) + node.y + (node.h / 2.) + GROUND_HEIGHT,
                5.0,
            ))
            .insert(LevelNode);
    }

    // spawn the targets
    for node in level_data[game.level].targets.iter() {
        commands
            .spawn()
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Circle {
                    radius: 10.,
                    center: Vec2::ZERO,
                },
                lyon::DrawMode::Outlined {
                    fill_mode: lyon::FillMode::color(TARGET_COLOR.0),
                    outline_mode: lyon::StrokeMode::color(TARGET_COLOR.1),
                },
                Transform::default(),
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(10.))
            .insert(Restitution::coefficient(0.7))
            .insert(ActiveEvents::default())
            .insert(ContactForceEventThreshold(TARGET_FORCE_THRESH))
            .insert(Transform::from_xyz(
                (win_w / 4.) + node.x,
                (-win_h / 2.0) + node.y + 5. + GROUND_HEIGHT,
                0.0,
            ))
            .insert(LevelNode)
            .insert(Target);
    }
}

fn remove_game(
    mut commands: Commands,
    level_nodes: Query<Entity, With<LevelNode>>,
    game_nodes: Query<Entity, With<GameNode>>,
) {
    for ent in level_nodes.iter() {
        commands.entity(ent).despawn_recursive();
    }

    for ent in game_nodes.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn keyboard_listener(
    mut commands: Commands,
    windows: ResMut<Windows>,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    keyboard: Res<Input<KeyCode>>,
    level_nodes: Query<Entity, With<LevelNode>>,
) {
    if keyboard.just_pressed(KeyCode::R) {
        for ent in level_nodes.iter() {
            commands.entity(ent).despawn();
        }

        setup_level(commands, windows, game, asset_server);
    } else if keyboard.just_pressed(KeyCode::N) && game.state == LevelState::Complete {
        for ent in level_nodes.iter() {
            commands.entity(ent).despawn();
        }

        game.level += 1;
        setup_level(commands, windows, game, asset_server);
    } else if keyboard.just_pressed(KeyCode::Q) && game.state == LevelState::LastLevelComplete {
        commands.insert_resource(NextState(AppState::MainMenu));
    }
}

fn power_indicator(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut power: ResMut<Power>,
    mut launch_evt: EventWriter<LaunchEvent>,
    mut query: Query<Entity, With<PowerIndicator>>,
) {
    if mouse.pressed(MouseButton::Right) {
        let window = windows.get_primary().unwrap();
        let pos1 = window.cursor_position().unwrap();
        let pos2 = pos1 - Vec2::new(window.width() / 2., window.height() / 2.);

        if let Ok(ent) = query.get_single_mut() {
            commands.entity(ent).despawn();
        }

        commands
            .spawn()
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &lyon::shapes::Circle {
                    radius: 1. + power.0,
                    center: Vec2::ZERO,
                },
                lyon::DrawMode::Outlined {
                    fill_mode: lyon::FillMode::color(Color::rgba(0.5, 0.5, 0.5, 0.7)),
                    outline_mode: lyon::StrokeMode::color(Color::rgba(0.5, 0.5, 0.5, 0.7)),
                },
                Transform::default(),
            ))
            .insert(Transform::from_xyz(pos2.x, pos2.y, 10.0))
            .insert(PowerIndicator);

        if power.0 < 70. {
            power.0 += 0.75;
        }
    }

    if mouse.just_released(MouseButton::Right) {
        let window = windows.get_primary().unwrap();
        let pos1 = window.cursor_position().unwrap();
        let pos2 = pos1 - Vec2::new(window.width() / 2., window.height() / 2.);

        if let Ok(ent) = query.get_single_mut() {
            commands.entity(ent).despawn();
        }

        launch_evt.send(LaunchEvent {
            power: power.0,
            target: pos2,
        });

        power.0 = 0.;
    }
}

fn launch(
    mut launch_evt: ResMut<Events<LaunchEvent>>,
    mut game: ResMut<Game>,
    mut ball: Query<(&mut ExternalImpulse, &Transform), With<Ball>>,
    mut targets: Query<&mut ActiveEvents, With<Target>>,
) {
    let mut clear_force = true;
    if let Ok((mut imp, trans)) = ball.get_single_mut() {
        for ev in launch_evt.drain() {
            // activate collisions on targets
            for mut tgt in targets.iter_mut() {
                *tgt = ActiveEvents::CONTACT_FORCE_EVENTS;
            }

            clear_force = false;
            let vec = ev.target - Vec2::new(trans.translation.x, trans.translation.y);
            let bottom = (vec.x.powi(2) + vec.y.powi(2)).sqrt();
            let norm_vec = vec / bottom;
            imp.impulse = norm_vec * ev.power * LAUNCH_FACTOR;
            game.state = LevelState::Launched;
        }

        if clear_force {
            imp.impulse = Vec2::ZERO;
        }
    }
}

fn target_collisions(
    mut commands: Commands,
    mut contact_force_events: EventReader<ContactForceEvent>,
    targets: Query<Entity, With<Target>>,
) {
    for collision_event in contact_force_events.iter() {
        if targets.contains(collision_event.collider2) {
            commands.entity(collision_event.collider2).despawn();
        }
    }
}

fn level_complete(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    max_level: Res<MaxLevel>,
    query: Query<&Target>,
) {
    if query.iter().len() == 0 {
        game.state = LevelState::Complete;

        let text;
        if game.level + 1 > max_level.0 {
            text = "All levels complete! Press \"q\" to quit";
            game.state = LevelState::LastLevelComplete;
        } else {
            text = "Press \"n\" for next level";
        };

        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(35.),
                        right: Val::Px(15.),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    text.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            })
            .insert(LevelNode);
    }
}
