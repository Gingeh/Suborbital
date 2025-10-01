use bevy::prelude::*;

use crate::{AppState, GameAssets, utils::text_style};

#[derive(Resource)]
pub struct Score {
    pub score: usize,
    pub high_score: usize,
}

#[derive(Message)]
pub struct ScoreMessage;

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct ScoreSpan;

#[derive(Component)]
struct HighScoreSpan;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score {
            score: 0,
            high_score: 0,
        })
        .add_message::<ScoreMessage>()
        .add_systems(OnExit(AppState::Splash), spawn_scoreboard)
        .add_systems(
            Update,
            (
                update_score,
                update_scoreboard.run_if(resource_changed::<Score>),
            ),
        )
        .add_systems(OnEnter(AppState::Playing), (show_score, reset_score))
        .add_systems(OnExit(AppState::Playing), hide_score);
    }
}

fn update_score(mut score: ResMut<Score>, mut score_events: MessageReader<ScoreMessage>) {
    score.score += score_events.len();
    if score.score > score.high_score {
        score.high_score = score.score;
    }
    score_events.clear();
}

fn reset_score(mut score: ResMut<Score>) {
    score.score = 0;
}

fn spawn_scoreboard(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(10),
            bottom: px(10),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                ScoreDisplay,
                Text::new("Score: "),
                text_style(&*assets, 40.0),
                Node {
                    display: Display::None,
                    ..default()
                },
                children![(ScoreSpan, TextSpan::default(), text_style(&*assets, 40.0))],
            ),
            (
                Text::new("High Score: "),
                text_style(&*assets, 40.0),
                children![(
                    HighScoreSpan,
                    TextSpan::default(),
                    text_style(&*assets, 40.0)
                )],
            )
        ],
    ));
}

fn update_scoreboard(
    score: Res<Score>,
    mut score_text: Single<&mut TextSpan, (With<ScoreSpan>, Without<HighScoreSpan>)>,
    mut high_score_text: Single<&mut TextSpan, (With<HighScoreSpan>, Without<ScoreSpan>)>,
) {
    ***score_text = score.score.to_string();
    ***high_score_text = score.high_score.to_string();
}

fn show_score(mut score_display: Single<&mut Node, With<ScoreDisplay>>) {
    score_display.display = Display::Flex;
}

fn hide_score(mut score_display: Single<&mut Node, With<ScoreDisplay>>) {
    score_display.display = Display::None;
}
