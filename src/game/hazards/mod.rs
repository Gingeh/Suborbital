use std::time::Duration;

use bevy::prelude::*;
use rand::distr::StandardUniform;
use rand::prelude::*;

use crate::{AppState, utils::Direction};

use super::score::Score;

mod asteroids;
mod crates;
mod laser;

#[derive(Resource, Deref, DerefMut)]
struct HazardTimer(Timer);

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum HazardType {
    Rock,
    Ice,
    Laser,
    Crate,
}

impl Distribution<HazardType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HazardType {
        match rng.random_range(0..10) {
            0..=3 => HazardType::Rock,  // 4/10 chance
            4..=6 => HazardType::Ice,   // 3/10 chance
            7..=8 => HazardType::Laser, // 2/10 chance
            _ => HazardType::Crate,     // 1/10 chance
        }
    }
}

#[derive(Event)]
pub struct HitEvent {
    pub hazard_type: HazardType,
    pub from_direction: Direction,
}

pub struct HazardsPlugin;

impl Plugin for HazardsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HazardTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, spawn_hazards.run_if(in_state(AppState::Playing)))
            .add_plugins((
                asteroids::AsteroidsPlugin,
                laser::LaserPlugin,
                crates::CratePlugin,
            ));
    }
}

fn spawn_hazards(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<HazardTimer>,
    score: Res<Score>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }
    timer.set_duration(Duration::from_secs_f32(
        15.0 / (score.score as f32 + 10.0) + 0.5,
    ));
    let hazard_type: HazardType = rand::random();

    match hazard_type {
        HazardType::Rock => commands.queue(asteroids::SpawnAsteroidCommand::Rock),
        HazardType::Ice => commands.queue(asteroids::SpawnAsteroidCommand::Ice),
        HazardType::Laser => commands.queue(laser::SpawnLaserCommand),
        HazardType::Crate => commands.queue(crates::SpawnCrateCommand),
    }
}
