use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::AppState;
use std::{f32::consts::PI, time::Duration};

#[derive(Component)]
struct Spaceship;

#[derive(Component, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Left,
            2 => Direction::Down,
            _ => Direction::Right,
        }
    }
}

impl Direction {
    fn rotate_cw(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }

    fn rotate_ccw(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(PI * match self {
            Self::Up => 0.0,
            Self::Left => 0.5,
            Self::Down => 1.0,
            Self::Right => 1.5,
        })
    }

    fn to_vec3(self) -> Vec3 {
        match self {
            Self::Up => Vec3::NEG_Y,
            Self::Left => Vec3::X,
            Self::Down => Vec3::Y,
            Self::Right => Vec3::NEG_X,
        }
    }
}

#[derive(Bundle)]
struct SpaceshipBundle {
    spaceship_marker: Spaceship,
    direction: Direction,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
struct Asteroid;

#[derive(Resource, Deref, DerefMut)]
struct AsteroidTimer(Timer);

#[derive(Bundle)]
struct AsteroidBundle {
    asteroid_marker: Asteroid,
    direction: Direction,
    #[bundle]
    sprite: SpriteBundle,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_game.in_schedule(OnEnter(AppState::Playing)))
            .insert_resource(AsteroidTimer(Timer::new(
                Duration::from_secs(1),
                TimerMode::Repeating,
            )))
            .add_systems(
                (
                    update_direction,
                    apply_direction,
                    spawn_asteroids,
                    update_asteroids,
                )
                    .in_set(OnUpdate(AppState::Playing)),
            );
    }
}

fn start_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spaceship_sprite = asset_server.load("spaceship.png");

    commands.spawn(SpaceshipBundle {
        spaceship_marker: Spaceship,
        direction: Direction::Up,
        sprite: SpriteBundle {
            texture: spaceship_sprite,
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 100.0, y: 100.0 }),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
    });
}

fn update_direction(
    input: Res<Input<KeyCode>>,
    mut directions: Query<&mut Direction, With<Spaceship>>,
) {
    if input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        for mut direction in directions.iter_mut() {
            *direction = direction.rotate_ccw()
        }
    } else if input.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        for mut direction in directions.iter_mut() {
            *direction = direction.rotate_cw()
        }
    };
}

fn apply_direction(mut spaceships: Query<(&Direction, &mut Transform), With<Spaceship>>) {
    for (&direction, mut transform) in spaceships.iter_mut() {
        let target_quat = direction.to_quat();
        transform.rotation = transform.rotation.slerp(target_quat, 0.3);
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidTimer>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    let direction: Direction = thread_rng().gen();
    let sprite = asset_server.load("asteroid.png");

    commands.spawn(AsteroidBundle {
        asteroid_marker: Asteroid,
        direction,
        sprite: SpriteBundle {
            texture: sprite,
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 50.0, y: 50.0 }),
                ..default()
            },
            transform: Transform::from_translation(direction.to_vec3() * -500.0).with_rotation(direction.to_quat()),
            ..default()
        },
    });
}

fn update_asteroids(mut asteroids: Query<(&Direction, &mut Transform), With<Asteroid>>, time: Res<Time>) {
    for (&direction, mut transform) in asteroids.iter_mut() {
        transform.translation += direction.to_vec3() * time.delta_seconds() * 200.0;
    }
}
