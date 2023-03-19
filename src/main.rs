use bevy::prelude::*;
use menu_demo::MenuDemoGame;

fn main() {
    App::new().add_plugin(MenuDemoGame).run();
}
