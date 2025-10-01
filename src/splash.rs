use bevy::prelude::*;

use crate::{AppState, GameAssets, utils::text_style};

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), setup_splash)
            .add_systems(
                Update,
                countdown_splash_timer.run_if(in_state(AppState::Splash)),
            );
    }
}

fn setup_splash(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        DespawnOnExit(AppState::Splash),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                ImageNode {
                    image: assets.bevy_logo.clone(),
                    ..default()
                },
                Node {
                    width: auto(),
                    height: px(200),
                    ..default()
                },
            ),
            (Text::new("Made with Bevy"), text_style(&*assets, 40.0))
        ],
    ));

    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn countdown_splash_timer(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).is_finished() {
        game_state.set(AppState::Menu);
    }
}
