mod pause_menu;
mod player_plugin;

use bevy::prelude::*;

use crate::game_play::{pause_menu::PauseMenu, player_plugin::PlayerPlugin};

pub struct GamePlay;

// consider changing to PluginGroup if valid
impl Plugin for GamePlay {
    fn build(&self, app: &mut App) {
        app.add_plugin(PauseMenu).add_plugin(PlayerPlugin);
    }
}
