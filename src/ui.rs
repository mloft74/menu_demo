use bevy::prelude::*;
use bevy_egui::egui;

use crate::NewType;

pub trait UiDefault {
    fn ui_default() -> Self;
}

impl UiDefault for egui::Frame {
    fn ui_default() -> Self {
        Self {
            fill: NewType(Color::CYAN).into(),
            outer_margin: egui::Margin::symmetric(400.0, 100.0),
            rounding: egui::Rounding::same(12.0),
            ..default()
        }
    }
}
