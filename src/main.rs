#![allow(clippy::type_complexity)]

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy::{asset::AssetMetaCheck, window::WindowResolution};

mod game;
mod gameover;
mod menu;
mod splash;
mod utils;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum AppState {
    #[default]
    Splash,
    Menu,
    Playing,
    GameOver,
}

#[derive(Resource)]
struct GameAssets {
    spaceship: Handle<Image>,
    broken_spaceship: Handle<Image>,
    rock_astroid: Handle<Image>,
    broken_rock_astroid: Handle<Image>,
    ice_astroid: Handle<Image>,
    background: Handle<Image>,
    font: Handle<Font>,
    bevy_logo: Handle<Image>,
    game_logo: Handle<Image>,
    clubbo: Handle<Image>,
    satilite_idle: Handle<Image>,
    satilite_charging: Handle<Image>,
    laser: Handle<Image>,
    heart: Handle<Image>,
    health_crate: Handle<Image>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();

        Self {
            spaceship: asset_server.load("ship.png"),
            broken_spaceship: asset_server.load("ship-broken.png"),
            rock_astroid: asset_server.load("rock.png"),
            broken_rock_astroid: asset_server.load("rock-broken.png"),
            ice_astroid: asset_server.load("ice.png"),
            background: asset_server.load("background.png"),
            font: asset_server.load("Overpass-SemiBold.subset.ttf"),
            bevy_logo: asset_server.load("bevy.png"),
            game_logo: asset_server.load("logo.png"),
            clubbo: asset_server.load("clubbo.png"),
            satilite_idle: asset_server.load("satilite-idle.png"),
            satilite_charging: asset_server.load("satilite-charging.png"),
            laser: asset_server.load("laser.png"),
            heart: asset_server.load("heart.png"),
            health_crate: asset_server.load("health-crate.png"),
        }
    }
}

#[derive(Component)]
struct Background;

fn main() {
    App::new()
        .insert_resource(ClearColor(Srgba::rgb_u8(0x2d, 0x1f, 0x4a).into()))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Suborbital".to_string(),
                        resolution: WindowResolution::new(800, 800),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .init_resource::<GameAssets>()
        .add_plugins((
            splash::SplashPlugin,
            menu::MenuPlugin,
            game::GamePlugin,
            gameover::GameOverPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_background)
        .run();
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(Camera2d);
    commands.spawn((Background, Sprite::from_image(assets.background.clone())));
}

fn animate_background(
    mut background_transform: Single<&mut Transform, With<Background>>,
    time: Res<Time>,
) {
    background_transform.translation.x = (time.elapsed_secs() * TAU / 120.0).cos() * 693.0;
}
