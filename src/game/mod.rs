use std::f32::consts::PI;

use bevy::prelude::*;

pub mod hazards;
pub mod health;
pub mod score;
pub mod spaceship;

#[derive(Component, Deref, DerefMut)]
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
        .add_systems(Update, handle_shake);
    }
}

fn handle_shake(
    mut commands: Commands,
    query: Query<(Entity, &mut Transform, &mut Shaking)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut shaking) in query {
        shaking.tick(time.delta());
        if shaking.just_finished() && shaking.mode() == TimerMode::Once {
            commands.entity(entity).try_remove::<Shaking>();
        } else {
            let progress = shaking.fraction();
            transform.scale = Vec3::splat(f32::sin(progress * 2.0 * PI).mul_add(0.1, 1.0));
        }
    }
}
