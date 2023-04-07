use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{ui::UiDefault, GameState};

// TODO: I can see the relative speed changing to be pretty annoying to manage for pausing, so it may be better to manually choose to not run in certain states
pub struct PauseMenu;

impl Plugin for PauseMenu {
    fn build(&self, app: &mut App) {
        app.add_state::<PauseState>()
            .add_system(pause_menu.run_if(in_state(PauseState::Paused)))
            .add_system(handle_escape.run_if(in_state(GameState::GamePlay)))
            .add_system(clean_up.in_schedule(OnExit(GameState::GamePlay)));
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum PauseState {
    #[default]
    Playing,
    Paused,
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
                    next_game_state.set(GameState::MainMenu);
                }
            });
        });
}

// TODO: move input checking to InputPlugin
fn handle_escape(
    keys: Res<Input<KeyCode>>,
    state: Res<State<PauseState>>,
    mut time: ResMut<Time>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    next_state.set(match state.0 {
        PauseState::Playing => {
            time.set_relative_speed(0.0);
            PauseState::Paused
        }
        PauseState::Paused => {
            time.set_relative_speed(1.0);
            PauseState::Playing
        }
    });
}

fn clean_up(mut time: ResMut<Time>, mut next_state: ResMut<NextState<PauseState>>) {
    time.set_relative_speed(1.0);
    next_state.set(PauseState::Playing);
}
