use std::time::Duration;

use bevy::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;

use crate::{utils::Direction, AppState};

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

impl Distribution<HazardType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HazardType {
        match rng.gen_range(0..10) {
            0..=3 => HazardType::Rock,  // 4/10 chance
            4..=6 => HazardType::Ice,   // 3/10 chance
            7..=8 => HazardType::Laser, // 2/10 chance
            _ => HazardType::Crate,     // 1/10 chance
        }
    }
}

pub struct HitEvent {
    pub hazard_type: HazardType,
    pub from_direction: Direction,
}

pub struct HazardsPlugin;

impl Plugin for HazardsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HazardTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_event::<HitEvent>()
            .add_systems(Update, spawn_hazards.run_if(in_state(AppState::Playing)))
            .add_plugin(asteroids::AsteroidsPlugin)
            .add_plugin(laser::LaserPlugin)
            .add_plugin(crates::CratePlugin);
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
        6.25 / (score.score as f32 + 5.0) + 0.75,
    ));

    let mut rng = thread_rng();
    let hazard_type: HazardType = rng.gen();

    match hazard_type {
        HazardType::Rock => commands.add(asteroids::SpawnAsteroidCommand::Rock),
        HazardType::Ice => commands.add(asteroids::SpawnAsteroidCommand::Ice),
        HazardType::Laser => commands.add(laser::SpawnLaserCommand),
        HazardType::Crate => commands.add(crates::SpawnCrateCommand),
    };
}
