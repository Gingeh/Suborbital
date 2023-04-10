use std::time::Duration;

use bevy::prelude::*;

use crate::{utils::Direction, AppState, GameAssets};

use super::{
    hazards::{HazardType, HitEvent},
    score::ScoreEvent,
    Game, Shaking,
};

#[derive(Component)]
pub struct Spaceship;

#[derive(Component)]
pub struct Health(u32);

#[derive(Bundle)]
struct SpaceshipBundle {
    spaceship_marker: Spaceship,
    game_marker: Game,
    direction: Direction,
    health: Health,
    #[bundle]
    sprite: SpriteBundle,
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_spaceship.in_schedule(OnEnter(AppState::Playing)))
            .add_systems(
                (update_direction, apply_direction, handle_hits)
                    .in_set(OnUpdate(AppState::Playing)),
            );
    }
}

fn spawn_spaceship(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(SpaceshipBundle {
        spaceship_marker: Spaceship,
        game_marker: Game,
        direction: Direction::Up,
        health: Health(3),
        sprite: SpriteBundle {
            texture: assets.spaceship.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 220.0, y: 220.0 }),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
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
            *direction = direction.rotate_ccw();
        }
    } else if input.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        for mut direction in directions.iter_mut() {
            *direction = direction.rotate_cw();
        }
    };
}

fn apply_direction(mut spaceships: Query<(&Direction, &mut Transform), With<Spaceship>>) {
    let (&direction, mut transform) = spaceships.single_mut();
    let target_quat = direction.to_quat();
    transform.rotation = transform.rotation.slerp(target_quat, 0.3);
}

fn handle_hits(
    mut commands: Commands,
    mut hit_event_reader: EventReader<HitEvent>,
    mut score_event_witer: EventWriter<ScoreEvent>,
    mut spaceships: Query<(Entity, &Direction, &mut Health), With<Spaceship>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for event in hit_event_reader.iter() {
        let (entity, &direction, mut health) = spaceships.single_mut();
        match event.hazard_type {
            HazardType::Rock => {
                if event.from_direction == direction.rotate_ccw() {
                    score_event_witer.send(ScoreEvent);
                } else {
                    health.0 -= 1;
                    commands.entity(entity).insert(Shaking(Timer::new(
                        Duration::from_millis(100),
                        TimerMode::Once,
                    )));
                }
            }
            HazardType::Ice => {
                if event.from_direction == direction {
                    score_event_witer.send(ScoreEvent);
                } else {
                    health.0 -= 1;
                    commands.entity(entity).insert(Shaking(Timer::new(
                        Duration::from_millis(100),
                        TimerMode::Once,
                    )));
                }
            }
            HazardType::Laser => {
                if event.from_direction == direction.rotate_cw() {
                    score_event_witer.send(ScoreEvent);
                } else {
                    health.0 -= 1;
                    commands.entity(entity).insert(Shaking(Timer::new(
                        Duration::from_millis(100),
                        TimerMode::Once,
                    )));
                }
            }
        }

        if health.0 == 0 {
            app_state.set(AppState::GameOver);
        }
    }
}
