mod pause_menu;
mod placeholder_map;
mod player_plugin;

use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::screens::game_play::{
    pause_menu::PauseMenu, placeholder_map::PlaceholderMapPlugin, player_plugin::PlayerPlugin,
};

pub struct GamePlay;

impl PluginGroup for GamePlay {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<GamePlay>()
            .add(PauseMenu::default())
            .add(PlayerPlugin)
            .add(PlaceholderMapPlugin)
    }
}
