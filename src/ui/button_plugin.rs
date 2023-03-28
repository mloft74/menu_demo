// I don't know how I feel about this impl. Buttons can only change state with this impl.
// Maybe there's a way that `&mut Commands` or a `World` could get passed to a dyn trait object.
// Performance may become an issue there as it would be exclusive. Would have to profile to know if it's an issue.

use bevy::prelude::*;

use crate::GameState;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((button_color, button_pressed).chain());
    }
}

pub const NORMAL_BUTTON_BACKGROUND: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON_BACKGROUND: Color = Color::rgb(0.25, 0.25, 0.25);
const CLICKED_BUTTON_BACKGROUND: Color = Color::rgb(0.35, 0.35, 0.35);

#[derive(Component)]
pub struct OnPressed(pub OnPressedFn);
type OnPressedFn = Box<dyn Fn(&mut ResMut<NextState<GameState>>) + Send + Sync>;

fn button_color(mut button_interaction_query: ButtonColorQuery) {
    for (interaction, mut color) in &mut button_interaction_query {
        let new_color = match interaction {
            Interaction::None => NORMAL_BUTTON_BACKGROUND,
            Interaction::Hovered => HOVERED_BUTTON_BACKGROUND,
            Interaction::Clicked => CLICKED_BUTTON_BACKGROUND,
        };
        *color = new_color.into();
    }
}
type ButtonColorQuery<'a, 'b, 'c> =
    Query<'a, 'b, (&'c Interaction, &'c mut BackgroundColor), (Changed<Interaction>, With<Button>)>;

fn button_pressed(
    mut button_interaction_query: ButtonPressedQuery,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (&interaction, on_pressed) in &mut button_interaction_query {
        if interaction == Interaction::Clicked {
            on_pressed.0(&mut next_state);
        }
    }
}
type ButtonPressedQuery<'a, 'b, 'c> =
    Query<'a, 'b, (&'c Interaction, &'c OnPressed), (Changed<Interaction>, With<Button>)>;
