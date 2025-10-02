use std::time::Duration;

use bevy::prelude::*;
use rand::{Rng, TryRngCore, rngs::OsRng};

use crate::{
    AppState,
    game::{
        hazards::{
            asteroids::{Asteroid, AsteroidsPlugin},
            crates::{Crate, CratePlugin},
            satellite::{Satellite, SatellitePlugin},
        },
        health::Health,
        score::Score,
    },
    utils::Direction,
};

mod asteroids;
mod crates;
mod satellite;

#[derive(Resource, Deref, DerefMut)]
struct HazardTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
pub struct PreviousCorrectDirection(pub Direction);

pub struct HazardsPlugin;

impl Plugin for HazardsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HazardTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(PreviousCorrectDirection(Direction::Up))
            .add_systems(Update, spawn_hazards.run_if(in_state(AppState::Playing)))
            .add_plugins((AsteroidsPlugin, SatellitePlugin, CratePlugin));
    }
}

fn spawn_hazards(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<HazardTimer>,
    score: Res<Score>,
    health: Res<Health>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }
    timer.set_duration(Duration::from_secs_f32(
        15.0 / (score.score as f32 + 10.0) + 0.5,
    ));

    match OsRng.unwrap_err().random_range(0..10) {
        0..=3 => {
            commands.spawn(Asteroid::Rock);
        }
        4..=6 => {
            commands.spawn(Asteroid::Ice);
        }
        7..=8 => {
            commands.spawn(Satellite);
        }
        _ => {
            if OsRng.unwrap_err().random_range(0..**health) == 0 {
                commands.spawn(Crate);
            }
        }
    }
}
