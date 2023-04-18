use bevy::prelude::*;

use crate::{
    despawn_recursive_entities_with,
    plugins::{
        cue_focus_camera_plugin::CameraCue, input_plugin::MovementDirection,
        mouse_tracking_plugin::WorldCursorPosition,
    },
    screens::game_play::pause_menu::PauseState,
    GameState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let game_state = GameState::GamePlay;
        app.add_systems((spawn_player, spawn_player_cursor).in_schedule(OnEnter(game_state)))
            .add_systems(
                (
                    despawn_recursive_entities_with::<Player>,
                    despawn_recursive_entities_with::<PlayerCursor>,
                )
                    .in_schedule(OnExit(game_state)),
            )
            .add_systems(
                (move_player, move_player_cursor)
                    .distributive_run_if(in_state(game_state))
                    .distributive_run_if(in_state(PauseState::Playing)),
            );
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        CameraCue { weight: 2.0 },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::ZERO),
            sprite: Sprite {
                color: Color::PINK,
                custom_size: Some(Vec2::splat(50.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    movement_direction: Res<MovementDirection>,
) {
    for mut transform in &mut query {
        transform.translation +=
            movement_direction.direction.extend(0.0) * time.delta_seconds() * 256.0;
    }
}

// consider moving cursor code to another file

#[derive(Component)]
struct PlayerCursor;

fn spawn_player_cursor(mut commands: Commands, cursor_position: Res<WorldCursorPosition>) {
    commands.spawn((
        PlayerCursor,
        CameraCue { weight: 1.0 },
        SpriteBundle {
            transform: Transform::from_translation(
                cursor_position.map_or(Vec3::ZERO, |pos| pos.extend(0.0)),
            ),
            sprite: Sprite {
                color: Color::PINK,
                custom_size: Some(Vec2::splat(10.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn move_player_cursor(
    mut query: Query<&mut Transform, With<PlayerCursor>>,
    cursor_position: Res<WorldCursorPosition>,
) {
    if let Some(pos) = cursor_position.0 {
        for mut transform in &mut query {
            transform.translation = pos.extend(0.0);
        }
    }
}
