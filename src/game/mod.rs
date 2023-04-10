use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{utils, AppState};

pub mod hazards;
pub mod health;
pub mod score;
pub mod spaceship;

#[derive(Component)]
struct Game;

#[derive(Component)]
struct Shaking(Timer);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(spaceship::SpaceshipPlugin)
            .add_plugin(hazards::HazardsPlugin)
            .add_plugin(score::ScorePlugin)
            .add_plugin(health::HealthPlugin)
            .add_system(handle_shake)
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
        if shaking.0.just_finished() && shaking.0.mode() == TimerMode::Once {
            commands.entity(entity).remove::<Shaking>();
        } else {
            let progress = shaking.0.percent();
            transform.scale = Vec3::splat(f32::sin(progress * 2.0 * PI).mul_add(0.1, 1.0));
        }
    }
}
