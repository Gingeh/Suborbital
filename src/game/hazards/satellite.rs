use std::{
    future::join,
    sync::atomic::{self, AtomicBool},
    time::Duration,
};

use bevy::{
    ecs::{
        lifecycle::HookContext,
        world::{DeferredWorld, WorldId},
    },
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_malek_async::{CreateEcsTask, EcsTask};
use futures_timer::Delay;
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
        app.insert_resource(OccupiedDirections([false; 4]));
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
struct SatelliteTask {
    _task: Task<()>,
}

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

    let task = AsyncComputeTaskPool::get().spawn(animate_satellite(world.id(), entity));

    let image = world.resource::<GameAssets>().satilite_idle.clone();
    world.commands().entity(entity).insert((
        direction,
        SatelliteTask { _task: task },
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

async fn repeat_for_duration<Fut: IntoFuture>(mut repeater: impl FnMut() -> Fut, dur: Duration) {
    let done = AtomicBool::new(false);
    join!(
        async {
            loop {
                repeater().await;
                Delay::new(Duration::from_millis(16)).await; // microsleep to avoid thrashing
                if done.load(atomic::Ordering::SeqCst) {
                    break;
                }
            }
        },
        async {
            Delay::new(dur).await;
            done.store(true, atomic::Ordering::SeqCst);
        }
    )
    .await;
}

async fn animate_satellite(world_id: WorldId, entity: Entity) {
    // move in for 1.5 secs
    let task_id = EcsTask::<Query<(&mut Transform, &Direction)>>::new(world_id);
    repeat_for_duration(
        || {
            task_id.clone().run_system(Update, |mut query| {
                let (mut transform, &direction) = query.get_mut(entity).unwrap();
                transform.translation = transform
                    .translation
                    .lerp(direction.to_vec3() * -320.0 + Vec3::Z, 0.1);
            })
        },
        Duration::from_secs_f32(1.5),
    )
    .await;

    // change sprite, start shaking, and wait 0.5 secs
    world_id
        .ecs_task::<(Commands, Query<&mut Sprite>, Res<GameAssets>)>()
        .run_system(Update, |(mut commands, mut query, assets)| {
            let mut sprite = query.get_mut(entity).unwrap();
            sprite.image = assets.satilite_charging.clone();
            commands
                .entity(entity)
                .insert(Shaking(Timer::from_seconds(0.5, TimerMode::Once)));
        })
        .await;
    Delay::new(Duration::from_secs_f32(0.5)).await;

    // fire the laser!
    world_id
        .ecs_task::<(
            Commands,
            Query<&Direction>,
            Res<GameAssets>,
            ResMut<Health>,
            Single<(Entity, &Direction), With<Spaceship>>,
        )>()
        .run_system(
            Update,
            |(mut commands, query, assets, mut health, spaceship)| {
                let (ship_entity, &ship_direction) = spaceship.into_inner();
                let &direction = query.get(entity).unwrap();
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
            },
        )
        .await;
    Delay::new(Duration::from_secs_f32(0.5)).await;

    // change sprite back and despawn laser
    world_id
        .ecs_task::<(Commands, Query<&mut Sprite>, Res<GameAssets>)>()
        .run_system(Update, |(mut commands, mut query, assets)| {
            let mut sprite = query.get_mut(entity).unwrap();
            sprite.image = assets.satilite_idle.clone();
            commands.entity(entity).despawn_children();
        })
        .await;

    // move out for 1 sec
    let task_id = EcsTask::<Query<(&mut Transform, &Direction)>>::new(world_id);
    repeat_for_duration(
        || {
            task_id.clone().run_system(Update, |mut query| {
                let (mut transform, &direction) = query.get_mut(entity).unwrap();
                transform.translation = transform
                    .translation
                    .lerp(direction.to_vec3() * -500.0 + Vec3::Z, 0.1);
            })
        },
        Duration::from_secs_f32(1.0),
    )
    .await;

    // despawn self
    world_id
        .ecs_task::<Commands>()
        .run_system(Update, |mut commands| {
            commands.entity(entity).despawn();
        })
        .await;
}
