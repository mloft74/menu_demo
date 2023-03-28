pub mod main_menu;
pub mod second_menu;

use bevy::ui::{UiRect, Val};

struct Padding;

impl Padding {
    pub fn all(val: Val) -> UiRect {
        UiRect {
            top: val,
            bottom: val,
            left: val,
            right: val,
        }
    }
}
