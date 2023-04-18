use bevy::prelude::*;

pub struct CueFocusCameraPlugin;

impl Plugin for CueFocusCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_camera_cues);
    }
}

#[derive(Component)]
pub struct CueFocusCamera;

#[derive(Component)]
pub struct CameraCue {
    pub weight: f32,
}

// Assumes 2d camera
fn apply_camera_cues(
    camera_cue_query: Query<(&Transform, &CameraCue)>,
    mut camera_query: Query<
        &mut Transform,
        (With<Camera>, With<CueFocusCamera>, Without<CameraCue>),
    >,
) {
    let (translation_sum, total_weight) = camera_cue_query.iter().fold(
        (Vec2::ZERO, 0.0),
        |(translation_sum, total_weight), (transform, cue)| {
            (
                translation_sum + transform.translation.truncate() * cue.weight,
                total_weight + cue.weight,
            )
        },
    );
    if total_weight == 0.0 {
        return;
    }

    let camera_cue_2d_translation = translation_sum / total_weight;
    let mut camera_transform = camera_query.single_mut();
    let rate = 0.1;
    camera_transform.translation = camera_transform
        .translation
        .truncate()
        .lerp(camera_cue_2d_translation, rate)
        .extend(camera_transform.translation.z);
}
