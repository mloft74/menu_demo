use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct MenuDemoGame;

impl Plugin for MenuDemoGame {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)
            .add_state::<GameState>()
            .add_system(setup_camera.on_startup())
            .add_system(ui_example);
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

fn ui_example(mut contexts: EguiContexts) {
    let window_frame = egui::Frame {
        shadow: egui::epaint::Shadow::NONE,
        fill: egui::Color32::from_gray(50),
        inner_margin: egui::Margin::same(8.0),
        ..default()
    };
    egui::Window::new("Unmovable hopefully")
        .movable(false)
        .resizable(false)
        .frame(window_frame)
        .current_pos((42.0, 87.5))
        .collapsible(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("heyo my dudes");
        });
    let frame = egui::Frame {
        fill: egui::Color32::from_rgba_premultiplied(200, 200, 200, 200),
        outer_margin: egui::Margin::same(300.0),
        ..default()
    };
    egui::CentralPanel::default()
        .frame(frame)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Hello!");
                ui.label("world");
            });
        });
}
