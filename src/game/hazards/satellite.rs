use std::time::Duration;

use bevy::ecs::lifecycle;
use bevy::prelude::*;
use rand::{Rng, TryRngCore, rngs::OsRng};

use crate::{
    AppState, GameAssets,
    game::{Shaking, health::Health, score::ScoreEvent, spaceship::Spaceship},
    utils::{Direction, shake_for_ms},
};

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_satellites.run_if(in_state(AppState::Playing)),
        )
        .add_observer(spawn_observer);
    }
}

#[derive(Component)]
pub struct Satellite;

#[derive(Component)]
enum SatelliteState {
    Idle,
    Charging,
    Firing,
    Retreating,
}

#[derive(Component, Deref, DerefMut)]
struct SatelliteTimer(Timer);

fn spawn_observer(
    event: On<lifecycle::Add, Satellite>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let entity = event.entity;
    let direction: Direction = OsRng.unwrap_err().random();

    commands.entity(entity).insert((
        direction,
        SatelliteState::Idle,
        SatelliteTimer(Timer::from_seconds(1.5, TimerMode::Once)),
        Sprite {
            image: game_assets.satilite_idle.clone(),
            custom_size: Some(Vec2 { x: 120.0, y: 120.0 }),
            ..default()
        },
        Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z * 2.0)
            .with_rotation(direction.to_quat()),
        DespawnOnExit(AppState::Playing),
    ));
}

fn update_satellites(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(
        &mut SatelliteTimer,
        &mut SatelliteState,
        &mut Sprite,
        &mut Transform,
        &Direction,
        Entity,
    )>,
    assets: Res<GameAssets>,
    spaceship: Single<(Entity, &Direction), With<Spaceship>>,
    mut health: ResMut<Health>,
) {
    let (ship_entity, &ship_direction) = spaceship.into_inner();
    for (mut timer, mut state, mut sprite, mut transform, &direction, entity) in query {
        timer.tick(time.delta());

        match *state {
            SatelliteState::Idle => {
                transform.translation = transform
                    .translation
                    .lerp(direction.to_vec3() * -320.0 + Vec3::Z, 0.1);

                if timer.is_finished() {
                    *state = SatelliteState::Charging;
                    sprite.image = assets.satilite_charging.clone();
                    timer.set_duration(Duration::from_secs_f32(0.5));
                    timer.reset();
                    commands
                        .entity(entity)
                        .insert(Shaking(Timer::from_seconds(1.0, TimerMode::Once)));
                }
            }
            SatelliteState::Charging => {
                if timer.is_finished() {
                    *state = SatelliteState::Firing;
                    timer.set_duration(Duration::from_secs_f32(0.5));
                    timer.reset();

                    commands.entity(entity).with_child((
                        Sprite {
                            image: assets.laser.clone(),
                            custom_size: Some(Vec2 { x: 20.0, y: 300.0 }),
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(0.0, 200.0, -1.0)),
                    ));
                    if direction == ship_direction.rotate_cw() {
                        commands.trigger(ScoreEvent);
                    } else {
                        **health -= 1;
                        commands.entity(ship_entity).insert(shake_for_ms(100));
                    }
                }
            }
            SatelliteState::Firing => {
                if timer.is_finished() {
                    *state = SatelliteState::Retreating;
                    sprite.image = assets.satilite_idle.clone();
                    timer.set_duration(Duration::from_secs_f32(1.0));
                    timer.reset();
                    commands.entity(entity).despawn_children();
                }
            }
            SatelliteState::Retreating => {
                transform.translation = transform
                    .translation
                    .lerp(direction.to_vec3() * -500.0 + Vec3::Z, 0.1);

                if timer.is_finished() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
