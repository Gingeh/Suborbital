use bevy::prelude::*;

use crate::{game::score::Score, utils, AppState, GameAssets};

#[derive(Component)]
struct GameOver;

#[derive(Component)]
enum GameOverButton {
    Retry,
    Menu,
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup_menu)
            .add_systems(Update, menu_action.run_if(in_state(AppState::GameOver)))
            .add_systems(OnExit(AppState::GameOver), utils::despawn_with::<GameOver>);
    }
}

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>, score: Res<Score>) {
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 40.0,
        color: Color::BLACK,
    };

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
            GameOver,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections(vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: score.score.to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                },
            ]));
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(400.0)),
                    ..default()
                },
                image: UiImage::new(assets.broken_spaceship.clone()),
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                ..default()
                            },
                            GameOverButton::Retry,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Retry", text_style.clone()));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                ..default()
                            },
                            GameOverButton::Menu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Back to title",
                                text_style.clone(),
                            ));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &GameOverButton), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                GameOverButton::Retry => app_state.set(AppState::Playing),
                GameOverButton::Menu => app_state.set(AppState::Menu),
            }
        }
    }
}
