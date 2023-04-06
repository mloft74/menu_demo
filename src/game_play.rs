use bevy::prelude::*;

use crate::GameState;

pub struct GamePlay;

impl Plugin for GamePlay {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::GamePlay)));
    }
}

#[derive(Component)]
struct Player;
fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 200.0)),
            sprite: Sprite {
                color: Color::PINK,
                custom_size: Some(Vec2::splat(50.0)),
                ..default()
            },
            ..default()
        },
    ));
}
