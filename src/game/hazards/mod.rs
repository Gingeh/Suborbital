use std::time::Duration;

use bevy::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;

use crate::{utils::Direction, AppState};

use super::score::Score;

mod asteroids;

#[derive(Resource, Deref, DerefMut)]
struct HazardTimer(Timer);

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum HazardType {
    Rock,
    Ice,
}

impl Distribution<HazardType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HazardType {
        match rng.gen_bool(0.5) {
            true => HazardType::Rock,
            false => HazardType::Ice,
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
            .add_system(spawn_hazards.in_set(OnUpdate(AppState::Playing)))
            .add_plugin(asteroids::AsteroidsPlugin);
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
        37.5 / (score.score as f32 + 50.0) + 0.25,
    ));

    let mut rng = thread_rng();
    let hazard_type: HazardType = rng.gen();

    match hazard_type {
        HazardType::Rock => commands.add(asteroids::SpawnAsteroidCommand::Rock),
        HazardType::Ice => commands.add(asteroids::SpawnAsteroidCommand::Ice),
    };
}
