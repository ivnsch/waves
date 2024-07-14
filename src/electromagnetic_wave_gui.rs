use bevy::prelude::*;

pub fn setup_electromagnetic_wave_gui(commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    add_info_labels(commands, &font);
}

fn add_info_labels(mut commands: Commands, font: &Handle<Font>) {
    commands.spawn(generate_info_label(&font, "move right: a", 0.0));
    commands.spawn(generate_info_label(&font, "move left: d", 20.0));
    commands.spawn(generate_info_label(&font, "zoom in: w", 40.0));
    commands.spawn(generate_info_label(&font, "zoom out: s", 60.0));
    commands.spawn(generate_info_label(
        &font,
        "rotate around z: i / shift-i",
        80.0,
    ));
    commands.spawn(generate_info_label(
        &font,
        "rotate around y: o / shift-o",
        100.0,
    ));
    commands.spawn(generate_info_label(
        &font,
        "rotate around x: p / shift-p",
        120.0,
    ));
}

fn generate_info_label(font: &Handle<Font>, label: &str, top: f32) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(top),
            left: Val::Px(10.0),
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}
