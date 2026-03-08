use std::{f32::consts::TAU, time::Duration};

use bevy::prelude::*;
use rand::{Rng, distr::StandardUniform, prelude::Distribution};

use crate::{GameAssets, game::Shaking};

pub fn button(text: impl Into<String>, assets: &GameAssets) -> impl Bundle {
    (
        Button,
        Node {
            width: px(250),
            height: px(65),
            margin: px(20).all(),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: px(4).all(),
            ..default()
        },
        BackgroundColor(Color::WHITE),
        BorderColor::all(Color::BLACK),
        children![(
            Text::new(text),
            TextFont {
                font: assets.font.clone().into(),
                font_size: FontSize::Px(40.0),
                ..default()
            },
            TextColor(Color::BLACK),
        )],
    )
}

pub fn text_style(font_size: f32, assets: &GameAssets) -> impl Bundle {
    (
        TextFont {
            font: assets.font.clone().into(),
            font_size: FontSize::Px(font_size),
            ..default()
        },
        TextColor(Color::WHITE),
        TextShadow {
            color: Color::BLACK,
            offset: Vec2::new(4.0, 4.0),
        },
    )
}

pub fn shake_for_ms(millis: u64) -> Shaking {
    Shaking(Timer::new(Duration::from_millis(millis), TimerMode::Once))
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Distribution<Direction> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.random_range(0..4) {
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

    pub const fn to_u8(self) -> u8 {
        match self {
            Self::Up => 0,
            Self::Left => 1,
            Self::Down => 2,
            Self::Right => 3,
        }
    }

    pub fn to_quat(self) -> Quat {
        Quat::from_rotation_z(TAU * f32::from(self.to_u8()) / 4.0)
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
