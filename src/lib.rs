mod ui;

use bevy::prelude::*;

use crate::ui::{
    button_plugin::ButtonPlugin,
    screens::{main_menu::MainMenuScreen, second_menu::SecondMenuScreen},
};

pub struct MenuDemoGame;

impl Plugin for MenuDemoGame {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_state::<GameState>()
            .add_plugin(ButtonPlugin)
            .add_plugin(MainMenuScreen)
            .add_plugin(SecondMenuScreen)
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
}
