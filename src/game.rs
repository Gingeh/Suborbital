use bevy::prelude::*;

use crate::AppState;
use std::f32::consts::PI;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(new_game.in_schedule(OnEnter(AppState::Playing)))
            .add_systems((update_direction, apply_direction).in_set(OnUpdate(AppState::Playing)));
    }
}

#[derive(Component)]
struct Spaceship;

#[derive(Component, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn to_radians(self) -> f32 {
        PI * match self {
            Self::Up => 0.0,
            Self::Left => 0.5,
            Self::Down => 1.0,
            Self::Right => 1.5,
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

fn new_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spaceship_sprite = asset_server.load("spaceship.png");

    commands.spawn(SpaceshipBundle {
        spaceship_marker: Spaceship,
        direction: Direction::Up,
        sprite: SpriteBundle {
            texture: spaceship_sprite,
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 64.0, y: 64.0 }),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    });
}

fn update_direction(
    input: Res<Input<KeyCode>>,
    mut directions: Query<&mut Direction, With<Spaceship>>,
) {
    let new_direction = if input.any_just_pressed([KeyCode::W, KeyCode::Up]) {
        Direction::Up
    } else if input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        Direction::Left
    } else if input.any_just_pressed([KeyCode::S, KeyCode::Down]) {
        Direction::Down
    } else if input.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        Direction::Right
    } else {
        return;
    };

    for mut direction in directions.iter_mut() {
        *direction = new_direction
    }
}

fn apply_direction(mut spaceships: Query<(&Direction, &mut Transform), With<Spaceship>>) {
    for (&direction, mut transform) in spaceships.iter_mut() {
        let target_quat = Quat::from_rotation_z(direction.to_radians());
        transform.rotation = transform.rotation.slerp(target_quat, 0.5);
    }
}

