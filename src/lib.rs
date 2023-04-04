use bevy::prelude::*;

pub struct MenuDemoGame;

impl Plugin for MenuDemoGame {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_state::<GameState>()
            .add_system(setup_camera.on_startup());
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    #[default]
    MainMenu,
    SecondMenu,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::PINK,
            custom_size: Some(Vec2::splat(50.0)),
            ..default()
        },
        ..default()
    });
}
