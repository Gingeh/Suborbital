use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{utils, AppState};
use std::{f32::consts::PI, time::Duration};

mod asteroids;
mod spaceship;

#[derive(Component)]
struct Game;

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

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum HazardType {
    Rock,
    Ice,
}

struct HitEvent {
    hazard_type: HazardType,
    from_direction: Direction,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HitEvent>()
            .insert_resource(asteroids::AsteroidTimer(Timer::new(
                Duration::from_secs(1),
                TimerMode::Repeating,
            )))
            .add_system(spaceship::spawn_spaceship.in_schedule(OnEnter(AppState::Playing)))
            .add_systems(
                (
                    spaceship::update_direction,
                    spaceship::apply_direction,
                    spaceship::handle_hits,
                    asteroids::spawn_asteroids,
                    asteroids::update_asteroids,
                    handle_shake,
                )
                    .in_set(OnUpdate(AppState::Playing)),
            )
            .add_system(utils::despawn_with::<Game>.in_schedule(OnExit(AppState::Playing)));
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
