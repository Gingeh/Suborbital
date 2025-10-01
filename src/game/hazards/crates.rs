use bevy::ecs::system::Command;
use bevy::prelude::*;

use crate::{AppState, GameAssets, utils::Direction};

use super::{HazardType, HitMessage};

pub struct CratePlugin;

impl Plugin for CratePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_crates.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component)]
struct Crate;

pub struct SpawnCrateCommand;

impl Command for SpawnCrateCommand {
    fn apply(self, world: &mut World) {
        let direction: Direction = rand::random();

        world.spawn((
            Crate,
            direction,
            Sprite {
                image: world
                    .get_resource::<GameAssets>()
                    .unwrap()
                    .health_crate
                    .clone(),
                custom_size: Some(Vec2 { x: 50.0, y: 50.0 }),
                ..default()
            },
            Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z),
            DespawnOnExit(AppState::Playing),
        ));
    }
}

fn update_crates(
    mut commands: Commands,
    mut event_writer: MessageWriter<HitMessage>,
    crates: Query<(Entity, &Direction, &mut Transform), With<Crate>>,
    time: Res<Time>,
) {
    for (entity, &direction, mut transform) in crates {
        transform.translation += direction.to_vec3() * time.delta_secs() * 200.0;
        transform.rotation *= Quat::from_rotation_z(time.delta_secs() * 2.0);

        if transform.translation.length() <= 70.0 {
            commands.entity(entity).despawn();
            event_writer.write(HitMessage {
                hazard_type: HazardType::Crate,
                from_direction: direction,
            });
        }
    }
}
