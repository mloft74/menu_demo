mod pause_menu;
mod player_plugin;

use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::game_play::{pause_menu::PauseMenu, player_plugin::PlayerPlugin};

pub struct GamePlay;

impl PluginGroup for GamePlay {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<GamePlay>()
            .add(PauseMenu::default())
            .add(PlayerPlugin)
    }
}
