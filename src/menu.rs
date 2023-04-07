use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{utils, AppState, GameAssets};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(menu_action.in_set(OnUpdate(AppState::Menu)))
            .add_system(utils::despawn_with::<MainMenu>.in_schedule(OnExit(AppState::Menu)));
    }
}

fn setup_menu(mut commands: Commands, assets: Res<GameAssets>) {
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
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(400.0)),
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
                    parent.spawn(TextBundle::from_section("Quit", text_style));
                });
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButton::Play => app_state.set(AppState::Playing),
                MenuButton::Quit => app_exit_writer.send(AppExit),
            }
        }
    }
}
