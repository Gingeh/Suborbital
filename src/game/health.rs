use bevy::prelude::*;

use crate::{AppState, GameAssets};

#[derive(Resource, Deref, DerefMut)]
pub struct Health(pub u32);

impl Default for Health {
    fn default() -> Self {
        Self(3)
    }
}

#[derive(Component)]
struct HealthDisplay;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Health>()
            .add_systems(
                OnEnter(AppState::Playing),
                ((spawn_health_display, reset_health), update_health_display).chain(),
            )
            .add_systems(
                Update,
                (update_health_display, reset_health)
                    .run_if(in_state(AppState::Playing))
                    .run_if(resource_changed::<Health>),
            );
    }
}

fn reset_health(mut health: ResMut<Health>) {
    *health = Health::default();
}

fn spawn_health_display(mut commands: Commands) {
    commands.spawn((
        HealthDisplay,
        Node {
            position_type: PositionType::Absolute,
            right: px(10),
            bottom: px(10),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));
}

fn update_health_display(
    mut commands: Commands,
    health: Res<Health>,
    health_display: Single<Entity, With<HealthDisplay>>,
    assets: Res<GameAssets>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if **health == 0 {
        app_state.set(AppState::GameOver);
        return;
    }
    commands.entity(*health_display).despawn_children();
    commands.entity(*health_display).with_children(|parent| {
        for _ in 0..**health {
            parent.spawn((
                ImageNode::new(assets.heart.clone()),
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
