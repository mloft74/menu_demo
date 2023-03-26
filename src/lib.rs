use bevy::prelude::*;

pub struct MenuDemoGame;

impl Plugin for MenuDemoGame {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_state::<GameState>()
            .add_system(setup_camera.on_startup())
            .add_system(setup_button.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(button_system);
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    MainMenu,
    SecondMenu,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);

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

fn button_system(
    mut button_interaction_query: ButtonInteractionQuery,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut button_interaction_query {
        let new_color = match interaction {
            Interaction::None => NORMAL_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::Clicked => {
                next_state.set(GameState::SecondMenu);
                PRESSED_BUTTON
            }
        };
        *color = new_color.into();
    }
}
type ButtonInteractionQuery<'a, 'b, 'c> =
    Query<'a, 'b, (&'c Interaction, &'c mut BackgroundColor), (Changed<Interaction>, With<Button>)>;
