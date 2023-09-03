use bevy::ecs::system::Command;
use bevy::prelude::*;

use rand::prelude::*;

use crate::{game::Game, utils::Direction, AppState, GameAssets};

use super::{HazardType, HitEvent};

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_asteroids.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
struct Asteroid;

#[derive(Bundle)]
struct AsteroidBundle {
    asteroid_marker: Asteroid,
    game_marker: Game,
    direction: Direction,
    hazard_type: HazardType,
    #[bundle()]
    sprite: SpriteBundle,
}

pub enum SpawnAsteroidCommand {
    Rock,
    Ice,
}

impl Command for SpawnAsteroidCommand {
    fn apply(self, world: &mut World) {
        let mut rng = thread_rng();
        let direction: Direction = rng.gen();

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

        world.spawn(AsteroidBundle {
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
}

fn update_asteroids(
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
