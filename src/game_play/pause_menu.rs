use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{input_plugin::Pause, ui::UiDefault, GameState};

// NOTE: I can see the relative speed changing to be pretty annoying to manage for pausing, so it may be better to manually choose to not run in certain states
// ^^^^: I have already entered a scenario where it's a little finnicky to use properly, since there are 2 ways of unpausing the game

#[derive(Default)]
pub struct PauseMenu {
    disable_ui: bool,
}

impl Plugin for PauseMenu {
    fn build(&self, app: &mut App) {
        app.add_state::<PauseState>()
            .add_system(
                handle_pause
                    .run_if(in_state(GameState::GamePlay))
                    .run_if(|pause: Res<Pause>| **pause),
            )
            .add_system(clean_up.in_schedule(OnExit(GameState::GamePlay)));
        if self.disable_ui {
            return;
        }
        app.add_system(pause_menu.run_if(in_state(PauseState::Paused)));
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
    mut time: ResMut<Time>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    egui::CentralPanel::default()
        .frame(egui::Frame::ui_default())
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                let padding_l = 12.0;
                let padding_s = 8.0;
                ui.add_space(padding_l);
                ui.heading("Paused");
                ui.separator();
                ui.add_space(padding_l);
                if ui.button("Continue").clicked() {
                    continue_playing(&mut time, &mut next_pause_state);
                }
                ui.add_space(padding_s);
                if ui.button("Main Menu").clicked() {
                    next_game_state.set(GameState::MainMenu);
                }
            });
        });
}

fn handle_pause(
    state: Res<State<PauseState>>,
    mut time: ResMut<Time>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    match state.0 {
        PauseState::Playing => {
            time.set_relative_speed(0.0);
            next_state.set(PauseState::Paused);
        }
        PauseState::Paused => continue_playing(&mut time, &mut next_state),
    };
}

fn continue_playing(time: &mut ResMut<Time>, next_state: &mut ResMut<NextState<PauseState>>) {
    time.set_relative_speed(1.0);
    next_state.set(PauseState::Playing);
}

fn clean_up(mut time: ResMut<Time>, mut next_state: ResMut<NextState<PauseState>>) {
    time.set_relative_speed(1.0);
    next_state.set(PauseState::Playing);
}

#[cfg(test)]
mod tests {
    use bevy::time::TimePlugin;

    use super::*;

    #[test]
    fn handle_pause_runs_only_when_res_is_true() {
        let mut app = App::new();
        app.add_plugin(TimePlugin)
            .insert_resource(Pause(false))
            .add_state::<GameState>()
            .add_state::<PauseState>();
        app.world.resource_mut::<State<GameState>>().0 = GameState::GamePlay;
        app.add_plugin(PauseMenu { disable_ui: true });

        app.update();

        let pre_state = app.world.resource::<State<PauseState>>().0.clone();
        assert_eq!(pre_state, PauseState::Playing, "pre_state was not Playing");
        let pre_next_state = app.world.resource::<NextState<PauseState>>().0.clone();
        assert_eq!(pre_next_state, None, "pre_next_state was wrong value");

        app.world.resource_mut::<Pause>().0 = true;
        app.update();

        let next_state = app.world.resource::<NextState<PauseState>>().0.clone();
        assert_eq!(
            next_state,
            Some(PauseState::Paused),
            "next_state was wrong value",
        );

        app.world.resource_mut::<Pause>().0 = false;
        app.update();

        let post_pause_state = app.world.resource::<State<PauseState>>().0.clone();
        assert_eq!(
            post_pause_state,
            PauseState::Paused,
            "post_pause_state was wrong value",
        );
        let post_next_state = app.world.resource::<NextState<PauseState>>().0.clone();
        assert_eq!(post_next_state, None, "post_next_state was wrong value");
    }
}
