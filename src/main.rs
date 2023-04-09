#![allow(clippy::type_complexity)]

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::WindowResolution;

use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod game;
mod menu;
mod score;
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

    #[asset(path = "clubbo.png")]
    clubbo: Handle<Image>,
}

#[derive(Component)]
struct Background;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Suborbital".to_string(),
            resolution: WindowResolution::new(800.0, 800.0),
            resizable: false,
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
        .add_plugin(score::ScorePlugin)
        .add_startup_system(setup)
        .add_system(animate_background)
        .run();
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: assets.background.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Background,
    ));
}

fn animate_background(mut background: Query<&mut Transform, With<Background>>, time: Res<Time>) {
    let mut transform = background.single_mut();
    transform.translation.x = (time.elapsed_seconds() * PI / 60.0).cos() * 693.0;
}
