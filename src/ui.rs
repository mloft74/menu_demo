use bevy::prelude::*;
use bevy_egui::egui;

use crate::NewType;

pub trait UiDefault {
    fn ui_default() -> Self;
}

impl UiDefault for egui::Frame {
    fn ui_default() -> Self {
        Self {
            fill: NewType(Color::SILVER.with_a(0.7)).into(),
            outer_margin: egui::Margin::symmetric(500.0, 200.0),
            rounding: egui::Rounding::same(12.0),
            ..Default::default()
        }
    }
}
