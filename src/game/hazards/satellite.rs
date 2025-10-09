use std::time::Duration;

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use rand::{Rng, TryRngCore, rngs::OsRng};

use crate::{
    AppState, GameAssets,
    game::{
        Shaking, hazards::PreviousCorrectDirection, health::Health, score::ScoreEvent,
        spaceship::Spaceship,
    },
    utils::{Direction, shake_for_ms},
};

pub struct SatellitePlugin;

impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OccupiedDirections([false; 4]))
            .add_systems(
                Update,
                update_satellites.run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Resource)]
struct OccupiedDirections([bool; 4]);

impl OccupiedDirections {
    const fn is_occupied(&self, direction: Direction) -> bool {
        self.0[direction.to_u8() as usize]
    }
    const fn set_occupied(&mut self, direction: Direction, occupied: bool) {
        self.0[direction.to_u8() as usize] = occupied;
    }
}

#[derive(Component)]
enum SatelliteState {
    Idle,
    Charging,
    Firing,
    Retreating,
}

#[derive(Component, Deref, DerefMut)]
struct SatelliteTimer(Timer);

const fn correct_ship_direction(hazard_direction: Direction) -> Direction {
    hazard_direction.rotate_ccw()
}

#[derive(Component)]
#[component(on_add = spawn_hook, on_remove = despawn_hook)]
pub struct Satellite;

fn spawn_hook(mut world: DeferredWorld, context: HookContext) {
    let entity = context.entity;

    let occupied_directions = world.resource::<OccupiedDirections>();
    let previous_correct_direction = world.resource::<PreviousCorrectDirection>();

    let mut direction: Direction = OsRng.unwrap_err().random();
    while **previous_correct_direction == correct_ship_direction(direction)
        || occupied_directions.is_occupied(direction)
    {
        direction = OsRng.unwrap_err().random();
    }

    **world.resource_mut::<PreviousCorrectDirection>() = correct_ship_direction(direction);
    world
        .resource_mut::<OccupiedDirections>()
        .set_occupied(direction, true);

    let image = world.resource::<GameAssets>().satilite_idle.clone();
    world.commands().entity(entity).insert((
        direction,
        SatelliteState::Idle,
        SatelliteTimer(Timer::from_seconds(1.5, TimerMode::Once)),
        Sprite {
            image,
            custom_size: Some(Vec2 { x: 120.0, y: 120.0 }),
            ..default()
        },
        Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z * 2.0)
            .with_rotation(direction.to_quat()),
        DespawnOnExit(AppState::Playing),
    ));
}

fn despawn_hook(mut world: DeferredWorld, context: HookContext) {
    let entity = context.entity;
    let direction = *world.get(entity).unwrap();
    world
        .resource_mut::<OccupiedDirections>()
        .set_occupied(direction, false);
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
                    if ship_direction == correct_ship_direction(direction) {
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
