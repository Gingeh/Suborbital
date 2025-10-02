use bevy::app::AppExit;
use bevy::prelude::*;

#[cfg(target_family = "wasm")]
use bevy::ecs::entity_disabling::Disabled;

use crate::{
    AppState, GameAssets,
    utils::{button, text_style},
};

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
    Clubbo,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)));
    }
}

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        DespawnOnExit(AppState::Menu),
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
                ImageNode::new(assets.game_logo.clone()),
                Node {
                    width: auto(),
                    height: px(300),
                    ..default()
                },
            ),
            (MenuButton::Play, button("Play", &assets)),
            (
                MenuButton::Quit,
                button("Quit", &assets),
                #[cfg(target_family = "wasm")]
                Disabled
            ),
        ],
    ));

    commands.spawn((
        MenuButton::Clubbo,
        DespawnOnExit(AppState::Menu),
        Button,
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            right: px(10),
            bottom: px(10),
            ..default()
        },
        children![
            (
                ImageNode::new(assets.clubbo.clone()),
                Node {
                    width: px(100),
                    height: px(100),
                    ..default()
                },
            ),
            (Text::new("Art by Clubbo"), text_style(30.0, &assets)),
            (Text::new("(Click Me!)"), text_style(20.0, &assets))
        ],
    ));
}

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match menu_button_action {
            MenuButton::Play => app_state.set(AppState::Playing),
            MenuButton::Quit => {
                app_exit_writer.write(AppExit::Success);
            }
            MenuButton::Clubbo => {
                _ = webbrowser::open("https://www.instagram.com/clubbo_cartoons/");
            }
        }
    }
}
