use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts};

use crate::{ui::UiDefault, GameState};

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_system(main_menu.run_if(in_state(GameState::MainMenu)));
    }
}

fn main_menu(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::ui_default())
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                let padding_l = 12.0;
                let padding_s = 8.0;
                ui.add_space(padding_l);
                ui.heading("Main Menu");
                ui.separator();
                ui.add_space(padding_l);
                if ui.button("Start").clicked() {
                    next_state.set(GameState::GamePlay);
                }
                ui.add_space(padding_s);
                if ui.button("Exit").clicked() {
                    exit.send(AppExit);
                }
            });
        });
}
