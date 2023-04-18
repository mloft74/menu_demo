#[allow(clippy::type_complexity)]
mod plugins;
mod screens;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::{
    plugins::{
        cue_focus_camera_plugin::{CueFocusCamera, CueFocusCameraPlugin},
        input_plugin::InputPlugin,
        mouse_tracking_plugin::{MouseTrackingCamera, MouseTrackingPlugin},
    },
    screens::{game_play::GamePlay, main_menu::MainMenu, title::Title},
};

pub struct MenuDemoGame;

impl Plugin for MenuDemoGame {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_plugin(CueFocusCameraPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(MouseTrackingPlugin)
            .add_state::<GameState>()
            .add_plugin(Title)
            .add_plugin(MainMenu)
            .add_plugins(GamePlay)
            .add_system(setup_camera.on_startup());
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum GameState {
    #[default]
    Title,
    MainMenu,
    GamePlay,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        CueFocusCamera,
        MouseTrackingCamera,
        Camera2dBundle::default(),
    ));
}

fn despawn_recursive_entities_with<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
