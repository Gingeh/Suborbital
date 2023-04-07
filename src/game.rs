use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::{distributions::Standard, prelude::Distribution, thread_rng, Rng};

use crate::{utils, AppState, GameAssets};
use std::{f32::consts::PI, time::Duration};

#[derive(Component)]
struct Game;

#[derive(Component)]
struct Spaceship;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Left,
            2 => Direction::Down,
            _ => Direction::Right,
        }
    }
}

impl Direction {
    const fn rotate_cw(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }

    const fn rotate_ccw(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(
            PI * match self {
                Self::Up => 0.0,
                Self::Left => 0.5,
                Self::Down => 1.0,
                Self::Right => 1.5,
            },
        )
    }

    const fn to_vec3(self) -> Vec3 {
        match self {
            Self::Up => Vec3::Y,
            Self::Left => Vec3::NEG_X,
            Self::Down => Vec3::NEG_Y,
            Self::Right => Vec3::X,
        }
    }
}

#[derive(Component)]
struct Shaking(Timer);

#[derive(Component)]
struct Health(u32);

#[derive(Bundle)]
struct SpaceshipBundle {
    spaceship_marker: Spaceship,
    game_marker: Game,
    direction: Direction,
    health: Health,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum HazardType {
    Rock,
    Ice,
}

#[derive(Component)]
struct Asteroid;

#[derive(Resource, Deref, DerefMut)]
struct AsteroidTimer(Timer);

#[derive(Bundle)]
struct AsteroidBundle {
    asteroid_marker: Asteroid,
    game_marker: Game,
    direction: Direction,
    hazard_type: HazardType,
    #[bundle]
    sprite: SpriteBundle,
}

struct HitEvent {
    hazard_type: HazardType,
    from_direction: Direction,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitEvent>()
            .insert_resource(AsteroidTimer(Timer::new(
                Duration::from_secs(1),
                TimerMode::Repeating,
            )))
            .add_system(start_game.in_schedule(OnEnter(AppState::Playing)))
            .add_systems(
                (
                    update_direction,
                    apply_direction,
                    spawn_asteroids,
                    update_asteroids,
                    handle_hits,
                    handle_shake,
                )
                    .in_set(OnUpdate(AppState::Playing)),
            )
            .add_system(utils::despawn_with::<Game>.in_schedule(OnExit(AppState::Playing)));
    }
}

fn start_game(mut commands: Commands, assets: Res<GameAssets>) {
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

fn spawn_asteroids(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidTimer>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    let mut rng = thread_rng();
    let direction = rng.gen();
    let hazard_type = *[HazardType::Rock, HazardType::Ice]
        .choose(&mut rng)
        .expect("The array isn't empty");
    let sprite = match hazard_type {
        HazardType::Rock => assets.rock_astroid.clone(),
        HazardType::Ice => assets.ice_astroid.clone(),
    };

    commands.spawn(AsteroidBundle {
        asteroid_marker: Asteroid,
        game_marker: Game,
        direction,
        hazard_type,
        sprite: SpriteBundle {
            texture: sprite,
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 80.0, y: 80.0 }),
                ..default()
            },
            transform: Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z)
                .with_rotation(direction.to_quat()),
            ..default()
        },
    });
}

fn update_asteroids(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut event_writer: EventWriter<HitEvent>,
    mut asteroids: Query<
        (
            Entity,
            &Direction,
            &HazardType,
            &mut Transform,
            &mut Handle<Image>,
        ),
        With<Asteroid>,
    >,
    time: Res<Time>,
) {
    for (entity, &direction, &hazard_type, mut transform, mut texture) in asteroids.iter_mut() {
        transform.translation += direction.to_vec3() * time.delta_seconds() * 200.0;
        if transform.translation.length() <= 70.0 {
            commands.entity(entity).despawn();
            event_writer.send(HitEvent {
                hazard_type,
                from_direction: direction,
            });
        } else if transform.translation.length() <= 100.0 && hazard_type == HazardType::Rock {
            *texture = assets.broken_rock_astroid.clone();
        }
    }
}

fn handle_hits(
    mut commands: Commands,
    mut event_reader: EventReader<HitEvent>,
    mut spaceships: Query<(Entity, &Direction, &mut Health), With<Spaceship>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for event in event_reader.iter() {
        let (entity, &direction, mut health) = spaceships.single_mut();
        match event.hazard_type {
            HazardType::Rock => {
                if event.from_direction != direction.rotate_ccw() {
                    health.0 -= 1;
                    commands.entity(entity).insert(Shaking(Timer::new(
                        Duration::from_millis(100),
                        TimerMode::Once,
                    )));
                }
            }
            HazardType::Ice => {
                if event.from_direction != direction {
                    health.0 -= 1;
                    commands.entity(entity).insert(Shaking(Timer::new(
                        Duration::from_millis(100),
                        TimerMode::Once,
                    )));
                }
            }
        }

        if health.0 == 0 {
            app_state.set(AppState::Menu);
        }
    }
}

fn handle_shake(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Shaking)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut shaking) in query.iter_mut() {
        shaking.0.tick(time.delta());
        if shaking.0.just_finished() {
            commands.entity(entity).remove::<Shaking>();
        } else {
            let progress = shaking.0.percent();
            transform.scale = Vec3::splat(f32::sin(progress * 2.0 * PI).mul_add(0.1, 1.0));
        }
    }
}
