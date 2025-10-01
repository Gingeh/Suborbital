use std::time::Duration;

use bevy::prelude::*;

use crate::{AppState, GameAssets, game::health::Health, utils::Direction};

use super::{
    Shaking,
    hazards::{HazardType, HitMessage},
    score::ScoreMessage,
};

#[derive(Component)]
pub struct Spaceship;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_spaceship)
            .add_systems(
                Update,
                (update_direction, apply_direction, handle_hits)
                    .run_if(in_state(AppState::Playing)),
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

fn handle_hits(
    mut commands: Commands,
    mut hit_reader: MessageReader<HitMessage>,
    mut score_writer: MessageWriter<ScoreMessage>,
    spaceship: Single<(Entity, &Direction), With<Spaceship>>,
    mut health: ResMut<Health>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let shake_for_ms = |millis| Shaking(Timer::new(Duration::from_millis(millis), TimerMode::Once));

    let (entity, &direction) = spaceship.into_inner();
    for event in hit_reader.read() {
        match event.hazard_type {
            HazardType::Rock => {
                if event.from_direction == direction.rotate_ccw() {
                    score_writer.write(ScoreMessage);
                } else {
                    **health -= 1;
                    commands.entity(entity).insert(shake_for_ms(100));
                }
            }
            HazardType::Ice => {
                if event.from_direction == direction {
                    score_writer.write(ScoreMessage);
                } else {
                    **health -= 1;
                    commands.entity(entity).insert(shake_for_ms(100));
                }
            }
            HazardType::Laser => {
                if event.from_direction == direction.rotate_cw() {
                    score_writer.write(ScoreMessage);
                } else {
                    **health -= 1;
                    commands.entity(entity).insert(shake_for_ms(100));
                }
            }
            HazardType::Crate => {
                if event.from_direction == direction.rotate_cw().rotate_cw() {
                    **health += 1;
                    score_writer.write(ScoreMessage);
                    commands.entity(entity).insert(shake_for_ms(200));
                }
            }
        }

        if **health == 0 {
            app_state.set(AppState::GameOver);
        }
    }
}
