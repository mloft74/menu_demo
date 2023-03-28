use bevy::prelude::*;

use crate::{
    ui::{button_plugin::OnPressed, screens::Padding},
    GameState,
};

pub struct MainMenuScreen;

impl Plugin for MainMenuScreen {
    fn build(&self, app: &mut App) {
        let state = GameState::MainMenu;
        app.add_system(setup_main_menu.in_schedule(OnEnter(state)))
            .add_system(cleanup_main_menu.in_schedule(OnExit(state)));
    }
}

#[derive(Component)]
struct MainMenu;

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            MainMenu,
            NodeBundle {
                background_color: Color::GRAY.into(),
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    OnPressed(Box::new(|next_state| {
                        println!("pressed main button");
                        next_state.set(GameState::SecondMenu);
                    })),
                    ButtonBundle {
                        background_color: Color::ALICE_BLUE.into(),
                        style: Style {
                            padding: Padding::all(Val::Px(8.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "To Second Menu",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
