use bevy::prelude::*;
use bevy_egui::egui;

pub trait UiDefault {
    fn ui_default() -> Self;
}

impl UiDefault for egui::Frame {
    fn ui_default() -> Self {
        Self {
            fill: BevyColor(Color::SILVER.with_a(0.7)).into(),
            outer_margin: egui::Margin::symmetric(500.0, 200.0),
            rounding: egui::Rounding::same(12.0),
            ..Default::default()
        }
    }
}

#[derive(Deref)]
struct BevyColor(Color);

impl From<BevyColor> for egui::Color32 {
    fn from(val: BevyColor) -> Self {
        // NOTE: 0o10 is 1 byte
        let rgba = val.as_rgba_u32();
        let r = (rgba & 0x000000FF) as u8;
        let g = ((rgba & 0x0000FF00) >> 0o10) as u8;
        let b = ((rgba & 0x00FF0000) >> 0o20) as u8;
        let a = ((rgba & 0xFF000000) >> 0o30) as u8;
        Self::from_rgba_premultiplied(r, g, b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_new_type_color() {
        let color = Color::Rgba {
            red: 1.0,
            green: 0.5,
            blue: 0.25,
            alpha: 0.125,
        };
        let actual = egui::Color32::from(BevyColor(color));
        let expected = egui::Color32::from_rgba_premultiplied(0xFF, 0x7f, 0x3f, 0x1f);

        assert_eq!(actual, expected, "colors were different",);
    }
}
