use bevy::prelude::*;

use crate::{despawn_recursive_entities_with, GameState};

pub struct PlaceholderMapPlugin;

impl Plugin for PlaceholderMapPlugin {
    fn build(&self, app: &mut App) {
        let state = GameState::GamePlay;
        app.add_system(spawn_placeholder_map.in_schedule(OnEnter(state)))
            .add_system(
                despawn_recursive_entities_with::<PlaceholderMap>.in_schedule(OnExit(state)),
            );
    }
}

#[derive(Component)]
struct PlaceholderMap;

fn spawn_placeholder_map(mut commands: Commands) {
    commands
        .spawn((PlaceholderMap, SpatialBundle::default()))
        .with_children(|parent| {
            let dimension = 128.0;
            let tile_size = Vec2::splat(dimension);
            let tile_offset = tile_size / 2.0;
            let half_tile_axis_size = 4;
            let tile_axis_count = half_tile_axis_size * 2;

            let tile_axis_range = -half_tile_axis_size..half_tile_axis_size;

            let color_offset = half_tile_axis_size as f32;
            let color_scalar = (tile_axis_count as f32).recip();
            for tile_x in tile_axis_range.clone() {
                let tile_x = tile_x as f32;
                for tile_y in tile_axis_range.clone() {
                    let tile_y = tile_y as f32;
                    let tile_coord = Vec2::new(tile_x, tile_y);
                    let tile_location = tile_size * tile_coord + tile_offset;
                    let color_rg = (tile_coord + color_offset) * color_scalar;
                    parent.spawn(SpriteBundle {
                        transform: Transform::from_translation(tile_location.extend(0.0)),
                        sprite: Sprite {
                            custom_size: Some(tile_size),
                            color: Color::rgb(color_rg.x, color_rg.y, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                }
            }
        });
}
