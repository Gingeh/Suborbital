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
                custom_size: Some(Vec2 { x: 100.0, y: 100.0 }),
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
        let target_quat = Quat::from_rotation_z(direction.to_radians());
        transform.rotation = transform.rotation.slerp(target_quat, 0.3);
    }
}
