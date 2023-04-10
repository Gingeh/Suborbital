use bevy::prelude::*;

use crate::{AppState, GameAssets};

use super::{spaceship::Health, Game};

#[derive(Component)]
struct HealthDisplay;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_health_display.in_schedule(OnEnter(AppState::Playing)))
            .add_system(update_health_display.in_set(OnUpdate(AppState::Playing)));
    }
}

fn spawn_health_display(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        HealthDisplay,
        Game,
    ));
}

fn update_health_display(
    mut commands: Commands,
    health_query: Query<&Health, Changed<Health>>,
    health_display_query: Query<Entity, With<HealthDisplay>>,
    assets: Res<GameAssets>,
) {
    let Ok(health) = health_query.get_single() else { return;};
    let health_display = health_display_query.single();

    commands.entity(health_display).despawn_descendants();
    commands.entity(health_display).with_children(|parent| {
        for _ in 0..health.0 {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                    margin: UiRect::left(Val::Px(10.0)),
                    ..default()
                },
                image: UiImage::new(assets.heart.clone()),
                ..default()
            });
        }
    });
}
