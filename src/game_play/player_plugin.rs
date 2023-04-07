use bevy::prelude::*;

use crate::{input_plugin::MovementDirection, GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let state = GameState::GamePlay;
        app.add_system(spawn_player.in_schedule(OnEnter(state)))
            .add_system(despawn_player.in_schedule(OnExit(state)))
            .add_system(move_player.run_if(in_state(state)));
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
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    movement_direction: Res<MovementDirection>,
) {
    for mut transform in &mut query {
        transform.translation +=
            movement_direction.direction.extend(0.0) * time.delta_seconds() * 60.0;
    }
}
