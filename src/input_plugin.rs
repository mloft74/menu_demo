use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementDirection>()
            .init_resource::<MovementModifier>()
            .init_resource::<Pause>()
            .add_systems(
                (
                    determine_movement_direction,
                    determine_movement_modifier,
                    determine_pause,
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

/// Will be true if a pause key was just pressed.
#[derive(Resource, Default, Deref)]
pub struct Pause(bool);

// TODO: make keys remappable
fn determine_movement_direction(
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

fn determine_movement_modifier(
    keyboard_input: Res<Input<KeyCode>>,
    mut movement_modifier: ResMut<MovementModifier>,
) {
    movement_modifier.primary = keyboard_input.pressed(KeyCode::LShift);
}

fn determine_pause(keyboard_input: Res<Input<KeyCode>>, mut pause: ResMut<Pause>) {
    pause.0 = keyboard_input.just_pressed(KeyCode::Escape);
}
