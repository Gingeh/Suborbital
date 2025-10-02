use bevy::prelude::*;

use crate::{
    AppState, GameAssets,
    game::score::Score,
    utils::{button, text_style},
};

#[derive(Component)]
enum GameOverButton {
    Retry,
    Menu,
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup_menu)
            .add_systems(Update, menu_action.run_if(in_state(AppState::GameOver)));
    }
}

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>, score: Res<Score>) {
    commands.spawn((
        DespawnOnExit(AppState::GameOver),
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
                Text::new(format!("Score: {}", score.score)),
                text_style(60.0, &assets),
            ),
            (
                ImageNode::new(assets.broken_spaceship.clone()),
                Node {
                    width: auto(),
                    height: px(400),
                    ..default()
                },
            ),
            (GameOverButton::Retry, button("Retry", &assets)),
            (GameOverButton::Menu, button("Back to title", &assets)),
        ],
    ));
}

fn menu_action(
    interaction_query: Query<(&Interaction, &GameOverButton), Changed<Interaction>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match menu_button_action {
            GameOverButton::Retry => app_state.set(AppState::Playing),
            GameOverButton::Menu => app_state.set(AppState::Menu),
        }
    }
}
