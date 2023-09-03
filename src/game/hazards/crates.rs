use bevy::ecs::system::Command;
use bevy::prelude::*;

use rand::prelude::*;

use crate::{game::Game, utils::Direction, AppState, GameAssets};

use super::{HazardType, HitEvent};

pub struct CratePlugin;

impl Plugin for CratePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_crates.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
struct Crate;

#[derive(Bundle)]
struct CrateBundle {
    crate_marker: Crate,
    game_marker: Game,
    direction: Direction,
    #[bundle()]
    sprite: SpriteBundle,
}

pub struct SpawnCrateCommand;

impl Command for SpawnCrateCommand {
    fn apply(self, world: &mut World) {
        let mut rng = thread_rng();
        let direction: Direction = rng.gen();

        world.spawn(CrateBundle {
            crate_marker: Crate,
            game_marker: Game,
            direction,
            sprite: SpriteBundle {
                texture: world
                    .get_resource::<GameAssets>()
                    .unwrap()
                    .health_crate
                    .clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 50.0, y: 50.0 }),
                    ..default()
                },
                transform: Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z),
                ..default()
            },
        });
    }
}

fn update_crates(
    mut commands: Commands,
    mut event_writer: EventWriter<HitEvent>,
    mut crates: Query<(Entity, &Direction, &mut Transform), With<Crate>>,
    time: Res<Time>,
) {
    for (entity, &direction, mut transform) in crates.iter_mut() {
        transform.translation += direction.to_vec3() * time.delta_seconds() * 200.0;
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * 2.0);

        if transform.translation.length() <= 70.0 {
            commands.entity(entity).despawn();
            event_writer.send(HitEvent {
                hazard_type: HazardType::Crate,
                from_direction: direction,
            });
        }
    }
}
