use std::time::Duration;

use crate::{score::Score, GameAssets};
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use super::{Direction, Game, HazardType, HitEvent};

#[derive(Component)]
pub struct Asteroid;

#[derive(Resource, Deref, DerefMut)]
pub struct AsteroidTimer(pub Timer);

#[derive(Bundle)]
struct AsteroidBundle {
    asteroid_marker: Asteroid,
    game_marker: Game,
    direction: Direction,
    hazard_type: HazardType,
    #[bundle]
    sprite: SpriteBundle,
}

pub(super) fn spawn_asteroids(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut timer: ResMut<AsteroidTimer>,
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
    let direction = rng.gen();
    let hazard_type = *[HazardType::Rock, HazardType::Ice]
        .choose(&mut rng)
        .expect("The array isn't empty");
    let sprite = match hazard_type {
        HazardType::Rock => assets.rock_astroid.clone(),
        HazardType::Ice => assets.ice_astroid.clone(),
    };

    commands.spawn(AsteroidBundle {
        asteroid_marker: Asteroid,
        game_marker: Game,
        direction,
        hazard_type,
        sprite: SpriteBundle {
            texture: sprite,
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 80.0, y: 80.0 }),
                ..default()
            },
            transform: Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z)
                .with_rotation(direction.to_quat()),
            ..default()
        },
    });
}

pub(super) fn update_asteroids(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut event_writer: EventWriter<HitEvent>,
    mut asteroids: Query<
        (
            Entity,
            &Direction,
            &HazardType,
            &mut Transform,
            &mut Handle<Image>,
        ),
        With<Asteroid>,
    >,
    time: Res<Time>,
) {
    for (entity, &direction, &hazard_type, mut transform, mut texture) in asteroids.iter_mut() {
        transform.translation += direction.to_vec3() * time.delta_seconds() * 200.0;
        if transform.translation.length() <= 70.0 {
            commands.entity(entity).despawn();
            event_writer.send(HitEvent {
                hazard_type,
                from_direction: direction,
            });
        } else if transform.translation.length() <= 100.0 && hazard_type == HazardType::Rock {
            *texture = assets.broken_rock_astroid.clone();
        }
    }
}
