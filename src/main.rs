use bevy::{prelude::*, window::WindowResizeConstraints};
use bevy_prototype_lyon::prelude as lyon;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

mod main_menu;
use main_menu::MainMenuPlugin;

mod game;
use game::GamePlugin;

mod constants;
use constants::{WIDTH, HEIGHT};

mod read_levels;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame
}
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.16, 0.62, 0.76)))
        .insert_resource(WindowDescriptor {
            title: "Trash the Tower".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: false,
            resize_constraints: WindowResizeConstraints {
                min_width: WIDTH,
                max_width: WIDTH,
                min_height: HEIGHT,
                max_height: HEIGHT,
            },
            ..default()
        })
        .add_loopless_state(AppState::MainMenu)
        .add_startup_system(setup_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(lyon::ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(30.0))
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
