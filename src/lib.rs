mod game_play;
mod input_plugin;
mod main_menu;
mod title;
mod ui;

use bevy::prelude::*;
use bevy_egui::{egui, EguiPlugin};

use crate::{game_play::GamePlay, input_plugin::InputPlugin, main_menu::MainMenu, title::Title};

pub struct MenuDemoGame;

impl Plugin for MenuDemoGame {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_plugin(InputPlugin)
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
    commands.spawn(Camera2dBundle::default());
}

#[derive(Deref)]
struct NewType<T>(T);

impl From<NewType<Color>> for egui::Color32 {
    fn from(val: NewType<Color>) -> Self {
        // NOTE: 0o10 is 1 byte
        let rgba = val.as_rgba_u32();
        let r = (rgba & 0x000000FF) as u8;
        let g = ((rgba & 0x0000FF00) >> 0o10) as u8;
        let b = ((rgba & 0x00FF0000) >> 0o20) as u8;
        let a = ((rgba & 0xFF000000) >> 0o30) as u8;
        Self::from_rgba_premultiplied(r, g, b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_new_type_color() {
        let color = Color::Rgba {
            red: 1.0,
            green: 0.5,
            blue: 0.25,
            alpha: 0.125,
        };
        let converted = egui::Color32::from(NewType(color));
        let expected = egui::Color32::from_rgba_premultiplied(0xFF, 0x7f, 0x3f, 0x1f);

        assert_eq!(expected, converted, "colors were different",);
    }
}
