mod pause_menu;

use bevy::prelude::*;

use crate::{game_play::pause_menu::PauseMenu, GameState};

pub struct GamePlay;

impl Plugin for GamePlay {
    fn build(&self, app: &mut App) {
        let state = GameState::GamePlay;
        app.add_state::<PauseState>()
            .add_plugin(PauseMenu)
            .add_system(spawn_player.in_schedule(OnEnter(state)))
            .add_system(despawn_player.in_schedule(OnExit(state)))
            .add_system(handle_escape.run_if(in_state(state)));
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
enum PauseState {
    #[default]
    Playing,
    Paused,
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

fn handle_escape(
    keys: Res<Input<KeyCode>>,
    state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    next_state.set(match state.0 {
        PauseState::Playing => PauseState::Paused,
        PauseState::Paused => PauseState::Playing,
    });
}
