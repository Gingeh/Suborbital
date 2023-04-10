use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

pub fn despawn_with<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
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
    pub const fn rotate_cw(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }

    pub const fn rotate_ccw(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn to_quat(self) -> Quat {
        Quat::from_rotation_z(
            PI * match self {
                Self::Up => 0.0,
                Self::Left => 0.5,
                Self::Down => 1.0,
                Self::Right => 1.5,
            },
        )
    }

    pub const fn to_vec3(self) -> Vec3 {
        match self {
            Self::Up => Vec3::Y,
            Self::Left => Vec3::NEG_X,
            Self::Down => Vec3::NEG_Y,
            Self::Right => Vec3::X,
        }
    }
}
