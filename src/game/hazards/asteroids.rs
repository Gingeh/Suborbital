use bevy::ecs::lifecycle;
use bevy::prelude::*;
use rand::{Rng, TryRngCore, rngs::OsRng};

use crate::{
    AppState, GameAssets,
    game::{
        hazards::PreviousCorrectDirection, health::Health, score::ScoreEvent, spaceship::Spaceship,
    },
    utils::{Direction, shake_for_ms},
};

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_asteroids.run_if(in_state(AppState::Playing)))
            .add_observer(spawn_observer);
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum Asteroid {
    Rock,
    Ice,
}

const fn correct_ship_direction(hazard_direction: Direction, asteroid_type: Asteroid) -> Direction {
    match asteroid_type {
        Asteroid::Rock => hazard_direction.rotate_cw(),
        Asteroid::Ice => hazard_direction,
    }
}

fn spawn_observer(
    event: On<lifecycle::Add, Asteroid>,
    mut commands: Commands,
    asteroid_types: Query<&Asteroid>,
    game_assets: Res<GameAssets>,
    mut previous_correct_direction: ResMut<PreviousCorrectDirection>,
) {
    let entity = event.entity;
    let asteroid_type = asteroid_types.get(entity).unwrap();

    let sprite = match asteroid_type {
        Asteroid::Rock => game_assets.rock_asteroid.clone(),
        Asteroid::Ice => game_assets.ice_asteroid.clone(),
    };

    let mut direction: Direction = OsRng.unwrap_err().random();
    while **previous_correct_direction == correct_ship_direction(direction, *asteroid_type) {
        direction = OsRng.unwrap_err().random();
    }
    **previous_correct_direction = correct_ship_direction(direction, *asteroid_type);

    commands.entity(entity).insert((
        direction,
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

fn update_asteroids(
    mut commands: Commands,
    assets: Res<GameAssets>,
    asteroids: Query<(Entity, &Direction, &Asteroid, &mut Transform, &mut Sprite)>,
    time: Res<Time>,
    spaceship: Single<(Entity, &Direction), With<Spaceship>>,
    mut health: ResMut<Health>,
) {
    let (ship_entity, &ship_direction) = spaceship.into_inner();
    for (entity, &direction, &asteroid_type, mut transform, mut sprite) in asteroids {
        transform.translation += direction.to_vec3() * time.delta_secs() * 200.0;
        if transform.translation.length() <= 70.0 {
            commands.entity(entity).despawn();
            if ship_direction == correct_ship_direction(direction, asteroid_type) {
                commands.trigger(ScoreEvent);
            } else {
                **health -= 1;
                commands.entity(ship_entity).insert(shake_for_ms(100));
            }
        } else if transform.translation.length() <= 100.0 && asteroid_type == Asteroid::Rock {
            sprite.image = assets.broken_rock_asteroid.clone();
        }
    }
}
