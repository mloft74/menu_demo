use bevy::prelude::*;

pub struct MenuDemoGame;

impl Plugin for MenuDemoGame {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}
