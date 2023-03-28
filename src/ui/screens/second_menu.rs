use bevy::prelude::*;

use crate::{
    ui::{button_plugin::OnPressed, screens::Padding},
    GameState,
};

pub struct SecondMenuScreen;

impl Plugin for SecondMenuScreen {
    fn build(&self, app: &mut App) {
        let state = GameState::SecondMenu;
        app.add_system(setup_second_menu.in_schedule(OnEnter(state)))
            .add_system(cleanup_second_menu.in_schedule(OnExit(state)));
    }
}

#[derive(Component)]
struct SecondMenu;

fn setup_second_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SecondMenu,
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
                        println!("pressed second button");
                        next_state.set(GameState::MainMenu);
                    })),
                    ButtonBundle {
                        background_color: Color::AZURE.into(),
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
                            "To Main Menu",
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

fn cleanup_second_menu(mut commands: Commands, query: Query<Entity, With<SecondMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
