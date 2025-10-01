use bevy::prelude::*;

use crate::{AppState, GameAssets};

#[derive(Resource, Deref, DerefMut)]
pub struct Health(pub u32);

#[derive(Component)]
struct HealthDisplay;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Health(3))
            .add_systems(
                OnEnter(AppState::Playing),
                (spawn_health_display, update_health_display).chain(),
            )
            .add_systems(
                Update,
                update_health_display
                    .run_if(in_state(AppState::Playing))
                    .run_if(resource_changed::<Health>),
            );
    }
}

fn spawn_health_display(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: px(10),
            bottom: px(10),
            ..default()
        },
        HealthDisplay,
        DespawnOnExit(AppState::Playing),
    ));
}

fn update_health_display(
    mut commands: Commands,
    health: Res<Health>,
    health_display: Single<Entity, With<HealthDisplay>>,
    assets: Res<GameAssets>,
) {
    commands.entity(*health_display).despawn_children();
    commands.entity(*health_display).with_children(|parent| {
        for _ in 0..**health {
            parent.spawn((
                ImageNode {
                    image: assets.heart.clone(),
                    ..default()
                },
                Node {
                    width: px(50),
                    height: px(50),
                    margin: px(10).left(),
                    ..default()
                },
            ));
        }
    });
}
