use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{GameState, NewType};

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_system(main_menu.run_if(in_state(GameState::MainMenu)));
    }
}

fn main_menu(mut contexts: EguiContexts, mut next_state: ResMut<NextState<GameState>>) {
    let frame = egui::Frame {
        fill: NewType(Color::CYAN).into(),
        outer_margin: egui::Margin::symmetric(400.0, 100.0),
        rounding: egui::Rounding::same(12.0),
        ..default()
    };
    egui::CentralPanel::default()
        .frame(frame)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(12.0);
                ui.heading("Main Menu");
                ui.separator();
                ui.add_space(12.0);
                if ui.button("To Second Menu").clicked() {
                    next_state.set(GameState::GamePlay);
                }
            });
        });
}
