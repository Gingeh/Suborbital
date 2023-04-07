use bevy::prelude::*;

use crate::{AppState, GameAssets};

#[derive(Resource)]
pub struct Score {
    pub score: usize,
    pub high_score: usize,
}

pub enum ScoreEvent {
    Increment,
    Reset,
}

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct HighScoreDisplay;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score {
            score: 0,
            high_score: 0,
        })
        .add_event::<ScoreEvent>()
        .add_system(spawn_scoreboard.in_schedule(OnExit(AppState::Splash)))
        .add_systems((update_score, update_scoreboard))
        .add_system(show_score.in_schedule(OnEnter(AppState::Playing)))
        .add_system(hide_score.in_schedule(OnExit(AppState::Playing)));
    }
}

fn update_score(mut score: ResMut<Score>, mut score_events: EventReader<ScoreEvent>) {
    for event in score_events.iter() {
        match event {
            ScoreEvent::Increment => {
                score.score += 1;
                if score.score > score.high_score {
                    score.high_score = score.score;
                }
            }
            ScoreEvent::Reset => {
                score.score = 0;
            }
        }
    }
}

fn spawn_scoreboard(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections(vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: String::new(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ])
                .with_style(Style {
                    display: Display::None,
                    ..default()
                }),
                ScoreDisplay,
            ));
            parent.spawn((
                TextBundle::from_sections(vec![
                    TextSection {
                        value: "High Score: ".to_string(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: String::new(),
                        style: TextStyle {
                            font: assets.font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ]),
                HighScoreDisplay,
            ));
        });
}

fn update_scoreboard(
    score: ResMut<Score>,
    mut score_text: Query<&mut Text, (With<ScoreDisplay>, Without<HighScoreDisplay>)>,
    mut high_score_text: Query<&mut Text, (With<HighScoreDisplay>, Without<ScoreDisplay>)>,
) {
    for mut text in score_text.iter_mut() {
        text.sections[1].value = score.score.to_string();
    }
    for mut text in high_score_text.iter_mut() {
        text.sections[1].value = score.high_score.to_string();
    }
}

fn show_score(mut score: Query<&mut Style, With<ScoreDisplay>>) {
    for mut style in score.iter_mut() {
        style.display = Display::Flex;
    }
}

fn hide_score(mut score: Query<&mut Style, With<ScoreDisplay>>) {
    for mut style in score.iter_mut() {
        style.display = Display::None;
    }
}
