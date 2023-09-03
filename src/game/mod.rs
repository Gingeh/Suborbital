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
        app.add_plugins((
            spaceship::SpaceshipPlugin,
            hazards::HazardsPlugin,
            score::ScorePlugin,
            health::HealthPlugin,
        ))
        .add_systems(Update, handle_shake)
        .add_systems(OnExit(AppState::Playing), utils::despawn_with::<Game>);
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
