use bevy::prelude::*;

// https://github.com/bevyengine/bevy/blob/v0.10.0/examples/ui/button.rs

pub struct MenuDemoGame;

impl Plugin for MenuDemoGame {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_startup_system(setup_camera)
            .add_startup_system(setup_button)
            .add_system(button_system);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

type ButtonInteractionQuery<'a, 'b, 'c> = Query<
    'a,
    'b,
    (&'c Interaction, &'c mut BackgroundColor, &'c Children),
    (Changed<Interaction>, With<Button>),
>;

fn button_system(
    mut button_interaction_query: ButtonInteractionQuery,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut button_interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let section = &mut text.sections[0];
        match interaction {
            Interaction::Clicked => {
                section.value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                section.value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                section.value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            background_color: Color::GRAY.into(),
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    background_color: NORMAL_BUTTON.into(),
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Button",
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
