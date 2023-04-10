use std::time::Duration;

use bevy::ecs::system::Command;
use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    game::{Game, Shaking},
    utils::Direction,
    AppState, GameAssets,
};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_satilites.in_set(OnUpdate(AppState::Playing)));
    }
}

#[derive(Component)]
struct Satilite;

#[derive(Component)]
enum SatiliteState {
    Idle,
    Charging,
    Firing,
    Retreating,
}

#[derive(Component)]
struct SatiliteTimer(Timer);

#[derive(Bundle)]
struct SatiliteBundle {
    satilite_marker: Satilite,
    game_marker: Game,
    direction: Direction,
    satilite_state: SatiliteState,
    timer: SatiliteTimer,
    #[bundle]
    sprite: SpriteBundle,
}

pub struct SpawnLaserCommand;

impl Command for SpawnLaserCommand {
    fn write(self, world: &mut World) {
        let mut rng = thread_rng();
        let direction: Direction = rng.gen();

        world.spawn(SatiliteBundle {
            satilite_marker: Satilite,
            game_marker: Game,
            direction,
            satilite_state: SatiliteState::Idle,
            timer: SatiliteTimer(Timer::from_seconds(1.5, TimerMode::Once)),
            sprite: SpriteBundle {
                texture: world
                    .get_resource::<GameAssets>()
                    .unwrap()
                    .satilite_idle
                    .clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 120.0, y: 120.0 }),
                    ..default()
                },
                transform: Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z)
                    .with_rotation(direction.to_quat()),
                ..default()
            },
        });
    }
}

fn update_satilites(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &mut SatiliteTimer,
        &mut SatiliteState,
        &mut Handle<Image>,
        &mut Transform,
        &Direction,
        Entity,
    )>,
    assets: Res<GameAssets>,
) {
    for (mut timer, mut state, mut sprite, mut transform, direction, entity) in query.iter_mut() {
        timer.0.tick(time.delta());

        match *state {
            SatiliteState::Idle => {
                transform.translation = transform
                    .translation
                    .lerp(direction.to_vec3() * -320.0 + Vec3::Z, 0.1);

                if timer.0.finished() {
                    *state = SatiliteState::Charging;
                    *sprite = assets.satilite_charging.clone();
                    timer.0.set_duration(Duration::from_secs_f32(0.5));
                    timer.0.reset();
                    commands
                        .entity(entity)
                        .insert(Shaking(Timer::from_seconds(1.0, TimerMode::Once)));
                }
            }
            SatiliteState::Charging => {
                if timer.0.finished() {
                    *state = SatiliteState::Firing;
                    timer.0.set_duration(Duration::from_secs_f32(0.5));
                    timer.0.reset();
                    //TODO: Spawn the laser
                }
            }
            SatiliteState::Firing => {
                if timer.0.finished() {
                    *state = SatiliteState::Retreating;
                    *sprite = assets.satilite_idle.clone();
                    timer.0.set_duration(Duration::from_secs_f32(1.0));
                    timer.0.reset();
                }
            }
            SatiliteState::Retreating => {
                transform.translation = transform
                    .translation
                    .lerp(direction.to_vec3() * -500.0 + Vec3::Z, 0.1);

                if timer.0.finished() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
