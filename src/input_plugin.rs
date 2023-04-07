use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementDirection>()
            .init_resource::<MovementModifier>()
            .add_systems(
                (
                    translate_input_to_movement_direction,
                    translate_input_to_movement_modifier,
                )
                    .in_base_set(CoreSet::PreUpdate),
            );
    }
}

#[derive(Resource, Default)]
pub struct MovementModifier {
    pub primary: bool,
}

#[derive(Resource, Default)]
pub struct MovementDirection {
    /// Not a normalized vector, but both axes are [-1.0, 1.0].
    pub direction: Vec2,
}

// TODO: make keys remappable
fn translate_input_to_movement_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut movement_direction: ResMut<MovementDirection>,
) {
    movement_direction.direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        movement_direction.direction += Vec2::Y;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement_direction.direction += Vec2::NEG_Y;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement_direction.direction += Vec2::X;
    }
    if keyboard_input.pressed(KeyCode::A) {
        movement_direction.direction += Vec2::NEG_X;
    }
}

fn translate_input_to_movement_modifier(
    keyboard_input: Res<Input<KeyCode>>,
    mut movement_modifier: ResMut<MovementModifier>,
) {
    movement_modifier.primary = keyboard_input.pressed(KeyCode::LShift);
}
