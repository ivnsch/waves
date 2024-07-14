use bevy::{color::palettes::css::GRAY, prelude::*};
use bevy_simple_text_input::TextInputInactive;

pub struct DefocusPlugin;

impl Plugin for DefocusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, defocus_on_esc);
    }
}

fn defocus_on_esc(
    key_input: Res<ButtonInput<KeyCode>>,
    mut text_input_query: Query<(&mut TextInputInactive, &mut BorderColor)>,
) {
    if key_input.pressed(KeyCode::Escape) {
        for (mut inactive, mut border_color) in &mut text_input_query {
            inactive.0 = true;
            *border_color = GRAY.into();
        }
    }
}
