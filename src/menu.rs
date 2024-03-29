use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{utils, AppState, GameAssets};

#[derive(Component)]
struct MainMenu;

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
            .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), utils::despawn_with::<MainMenu>);
    }
}

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Px(300.0),
                    ..default()
                },
                image: UiImage::new(assets.game_logo.clone()),
                ..default()
            });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        ..default()
                    },
                    MenuButton::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Play", text_style.clone()));
                });
            #[cfg(not(target_family = "wasm"))]
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        ..default()
                    },
                    MenuButton::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Quit", text_style.clone()));
                });
        });

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    right: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            MainMenu,
            MenuButton::Clubbo,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                image: UiImage::new(assets.clubbo.clone()),
                ..default()
            });
            parent.spawn(TextBundle::from_section(
                "Art by Clubbo",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "(Click Me!)",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButton::Play => app_state.set(AppState::Playing),
                MenuButton::Quit => app_exit_writer.send(AppExit),
                MenuButton::Clubbo => {
                    if let Err(error) =
                        webbrowser::open("https://www.instagram.com/clubbo_cartoons/")
                    {
                        error!("Failed to open browser: {}", error);
                    }
                }
            }
        }
    }
}
