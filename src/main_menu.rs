use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{ui::UiDefault, GameState};

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_system(main_menu.run_if(in_state(GameState::MainMenu)));
    }
}

fn main_menu(mut contexts: EguiContexts, mut next_state: ResMut<NextState<GameState>>) {
    egui::CentralPanel::default()
        .frame(egui::Frame::ui_default())
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                let padding = 12.0;
                ui.add_space(padding);
                ui.heading("Main Menu");
                ui.separator();
                ui.add_space(padding);
                if ui.button("Start").clicked() {
                    next_state.set(GameState::GamePlay);
                }
            });
        });
}
