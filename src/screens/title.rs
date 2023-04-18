use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_egui::EguiContexts;

use crate::{ui::UiDefault, GameState};

pub struct Title;

impl Plugin for Title {
    fn build(&self, app: &mut App) {
        app.add_systems((title, press_any_key).distributive_run_if(in_state(GameState::Title)));
    }
}

fn title(mut contexts: EguiContexts) {
    use bevy_egui::egui::*;
    let frame = Frame {
        fill: Color32::from_black_alpha(0x00),
        ..UiDefault::ui_default()
    };
    CentralPanel::default()
        .frame(frame)
        .show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                let color = Color32::from_white_alpha(0xFF);
                ui.label(
                    RichText::new("Menu Demo")
                        .font(FontId::proportional(40.0))
                        .color(color),
                );
                ui.add_space(16.0);
                ui.label(RichText::new("Press any key").color(color));
            });
        });
}

fn press_any_key(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    fn pressed(state: ButtonState) -> bool {
        state == ButtonState::Pressed
    }
    if keyboard_events.iter().map(|input| input.state).any(pressed)
        || mouse_button_events
            .iter()
            .map(|input| input.state)
            .any(pressed)
    {
        next_state.set(GameState::MainMenu);
    }
}
