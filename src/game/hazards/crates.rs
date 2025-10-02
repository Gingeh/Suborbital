use bevy::ecs::lifecycle;
use bevy::prelude::*;
use rand::{Rng, TryRngCore, rngs::OsRng};

use crate::{
    AppState, GameAssets,
    game::hazards::{HazardType, HitEvent},
    utils::Direction,
};

pub struct CratePlugin;

impl Plugin for CratePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_crates.run_if(in_state(AppState::Playing)))
            .add_observer(spawn_observer);
    }
}

#[derive(Component)]
pub struct Crate;

fn spawn_observer(
    event: On<lifecycle::Add, Crate>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let entity = event.entity;
    let direction: Direction = OsRng.unwrap_err().random();

    commands.entity(entity).insert((
        direction,
        Sprite {
            image: game_assets.health_crate.clone(),
            custom_size: Some(Vec2 { x: 50.0, y: 50.0 }),
            ..default()
        },
        Transform::from_translation(direction.to_vec3() * -500.0 + Vec3::Z),
        DespawnOnExit(AppState::Playing),
    ));
}

fn update_crates(
    mut commands: Commands,
    crates: Query<(Entity, &Direction, &mut Transform), With<Crate>>,
    time: Res<Time>,
) {
    for (entity, &direction, mut transform) in crates {
        transform.translation += direction.to_vec3() * time.delta_secs() * 200.0;
        transform.rotation *= Quat::from_rotation_z(time.delta_secs() * 2.0);

        if transform.translation.length() <= 70.0 {
            commands.entity(entity).despawn();
            commands.trigger(HitEvent {
                hazard_type: HazardType::Crate,
                from_direction: direction,
            });
        }
    }
}
