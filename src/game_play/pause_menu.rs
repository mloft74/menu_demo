use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{game_play::PauseState, ui::UiDefault, GameState};

pub struct PauseMenu;

impl Plugin for PauseMenu {
    fn build(&self, app: &mut App) {
        app.add_system(pause_menu.run_if(in_state(PauseState::Paused)));
    }
}

fn pause_menu(
    mut contexts: EguiContexts,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::ui_default())
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                let padding = 12.0;
                ui.add_space(padding);
                ui.heading("Paused");
                ui.separator();
                ui.add_space(padding);
                if ui.button("Continue").clicked() {
                    next_pause_state.set(PauseState::Playing);
                }
                if ui.button("Main Menu").clicked() {
                    next_pause_state.set(PauseState::Playing);
                    next_game_state.set(GameState::MainMenu);
                }
            });
        });
}
