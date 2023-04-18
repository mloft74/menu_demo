use bevy::prelude::*;

pub struct MouseTrackingPlugin;

impl Plugin for MouseTrackingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldCursorPosition>()
            .add_system(cursor_to_world.in_base_set(CoreSet::PreUpdate));
    }
}

#[derive(Component)]
pub struct MouseTrackingCamera;

#[derive(Resource, Default, Deref)]
pub struct WorldCursorPosition(pub Option<Vec2>);

fn cursor_to_world(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MouseTrackingCamera>>,
    mut world_cursor_position: ResMut<WorldCursorPosition>,
) {
    let window = windows.single();
    world_cursor_position.0 = window.cursor_position().and_then(|position| {
        let (camera, camera_transform) = camera_query.single();
        camera.viewport_to_world_2d(camera_transform, position)
    });
}
