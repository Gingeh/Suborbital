#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy::window::WindowResolution;

use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod game;
mod menu;
mod splash;
mod utils;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum AppState {
    #[default]
    Splash,
    Menu,
    Playing,
    Paused,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "ship.png")]
    spaceship: Handle<Image>,

    #[asset(path = "rock.png")]
    rock_astroid: Handle<Image>,

    #[asset(path = "rock-broken.png")]
    broken_rock_astroid: Handle<Image>,

    #[asset(path = "ice.png")]
    ice_astroid: Handle<Image>,

    #[asset(path = "background.png")]
    background: Handle<Image>,

    #[asset(path = "Overpass-SemiBold.ttf")]
    font: Handle<Font>,

    #[asset(path = "bevy.png")]
    bevy_logo: Handle<Image>,

    #[asset(path = "logo.png")]
    game_logo: Handle<Image>,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(800.0, 800.0),
            ..default()
        }),
        ..default()
    }));

    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_state::<AppState>()
        .init_collection::<GameAssets>()
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: assets.background.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });
}
