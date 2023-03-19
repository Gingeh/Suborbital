use bevy::prelude::*;

use crate::{utils, AppState};

#[derive(Component)]
struct SplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_splash.in_schedule(OnEnter(AppState::Splash)))
            .add_system(countdown_splash_timer.in_set(OnUpdate(AppState::Splash)))
            .add_system(utils::despawn_with::<SplashScreen>.in_schedule(OnExit(AppState::Splash)));
    }
}

fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("bevy.png");
    let font = asset_server.load("Overpass-SemiBold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            SplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(200.0)),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
            parent.spawn(TextBundle::from_section(
                "Made with Bevy",
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ));
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn countdown_splash_timer(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::Menu);
    }
}
