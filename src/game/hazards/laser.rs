use std::time::Duration;

use bevy::ecs::system::Command;
use bevy::prelude::*;

use crate::{AppState, GameAssets, game::Shaking, utils::Direction};

use super::{HazardType, HitEvent};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_satilites.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
struct Satellite;

#[derive(Component)]
enum SatelliteState {
    Idle,
    Charging,
    Firing,
    Retreating,
}

#[derive(Component, Deref, DerefMut)]
struct SatelliteTimer(Timer);

pub struct SpawnLaserCommand;

impl Command for SpawnLaserCommand {
    fn apply(self, world: &mut World) {
        let direction: Direction = rand::random();

        world.spawn((
            Satellite,
            direction,
            SatelliteState::Idle,
            SatelliteTimer(Timer::from_seconds(1.5, TimerMode::Once)),
            Sprite {
                image: world
                    .get_resource::<GameAssets>()
                    .unwrap()
                    .satilite_idle
                    .clone(),
                custom_size: Some(Vec2 { x: 120.0, y: 120.0 }),
                ..default()
            },
            Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z * 2.0)
                .with_rotation(direction.to_quat()),
            DespawnOnExit(AppState::Playing),
        ));
    }
}

fn update_satilites(
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
) {
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

                    commands.trigger(HitEvent {
                        from_direction: direction,
                        hazard_type: HazardType::Laser,
                    });
                }
            }
            SatelliteState::Firing => {
                if timer.is_finished() {
                    *state = SatelliteState::Retreating;
                    sprite.image = assets.satilite_idle.clone();
                    timer.set_duration(Duration::from_secs_f32(1.0));
                    timer.reset();
                    commands.entity(entity).despawn();
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
