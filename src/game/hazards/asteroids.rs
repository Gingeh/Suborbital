use bevy::ecs::system::Command;
use bevy::prelude::*;
use rand::{Rng, TryRngCore, rngs::OsRng};

use crate::{
    AppState, GameAssets,
    game::hazards::{HazardType, HitEvent},
    utils::Direction,
};

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_asteroids.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
struct Asteroid;

pub enum SpawnAsteroidCommand {
    Rock,
    Ice,
}

impl Command for SpawnAsteroidCommand {
    fn apply(self, world: &mut World) {
        let direction: Direction = OsRng.unwrap_err().random();

        let hazard_type = match self {
            Self::Rock => HazardType::Rock,
            Self::Ice => HazardType::Ice,
        };

        let sprite = match self {
            Self::Rock => world
                .get_resource::<GameAssets>()
                .unwrap()
                .rock_astroid
                .clone(),
            Self::Ice => world
                .get_resource::<GameAssets>()
                .unwrap()
                .ice_astroid
                .clone(),
        };

        world.spawn((
            Asteroid,
            direction,
            hazard_type,
            Sprite {
                image: sprite,
                custom_size: Some(Vec2 { x: 80.0, y: 80.0 }),
                ..default()
            },
            Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z)
                .with_rotation(direction.to_quat()),
            DespawnOnExit(AppState::Playing),
        ));
    }
}

fn update_asteroids(
    mut commands: Commands,
    assets: Res<GameAssets>,
    asteroids: Query<
        (Entity, &Direction, &HazardType, &mut Transform, &mut Sprite),
        With<Asteroid>,
    >,
    time: Res<Time>,
) {
    for (entity, &direction, &hazard_type, mut transform, mut sprite) in asteroids {
        transform.translation += direction.to_vec3() * time.delta_secs() * 200.0;
        if transform.translation.length() <= 70.0 {
            commands.entity(entity).despawn();
            commands.trigger(HitEvent {
                hazard_type,
                from_direction: direction,
            });
        } else if transform.translation.length() <= 100.0 && hazard_type == HazardType::Rock {
            sprite.image = assets.broken_rock_astroid.clone();
        }
    }
}
