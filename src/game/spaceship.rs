use bevy::prelude::*;

use crate::{AppState, GameAssets, utils::Direction};

#[derive(Component)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_spaceship)
            .add_systems(
                Update,
                (update_direction, apply_direction).run_if(in_state(AppState::Playing)),
            );
    }
}

fn spawn_spaceship(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Spaceship,
        Direction::Up,
        Sprite {
            image: assets.spaceship.clone(),
            custom_size: Some(Vec2 { x: 220.0, y: 220.0 }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 2.0),
        DespawnOnExit(AppState::Playing),
    ));
}

fn update_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut ship_direction: Single<&mut Direction, With<Spaceship>>,
) {
    if input.any_just_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        **ship_direction = ship_direction.rotate_ccw();
    } else if input.any_just_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        **ship_direction = ship_direction.rotate_cw();
    }
}

fn apply_direction(spaceship: Single<(&Direction, &mut Transform), With<Spaceship>>) {
    let (direction, mut transform) = spaceship.into_inner();
    let target_quat = direction.to_quat();
    transform.rotation = transform.rotation.slerp(target_quat, 0.3);
}
