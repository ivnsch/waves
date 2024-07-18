use bevy::prelude::*;
use uom::si::{
    angle::{radian, Angle},
    electric_field::volt_per_meter,
    f32::{ElectricField, Frequency, Length},
    frequency::hertz,
    length::meter,
};

use crate::wave_gui::{despawn_all_entities, parse_float, Freq, GuiInputsEvent, Phase, WaveLength};

pub fn setup_electromagnetic_wave_gui(commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    add_info_labels(commands, &font);
}

fn add_info_labels(mut commands: Commands, font: &Handle<Font>) {
    commands.spawn(generate_info_label(font, "move right: a", 0.0));
    commands.spawn(generate_info_label(font, "move left: d", 20.0));
    commands.spawn(generate_info_label(font, "zoom in: w", 40.0));
    commands.spawn(generate_info_label(font, "zoom out: s", 60.0));
    commands.spawn(generate_info_label(
        font,
        "rotate around z: i / shift-i",
        80.0,
    ));
    commands.spawn(generate_info_label(
        font,
        "rotate around y: o / shift-o",
        100.0,
    ));
    commands.spawn(generate_info_label(
        font,
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

/// processes the gui events
// TODO error handling (show on ui)
#[allow(clippy::too_many_arguments)]
pub fn listen_electromagnetic_wave_gui_inputs(
    mut events: EventReader<GuiInputsEvent>,
    mut commands: Commands,
    amplitude_query: Query<Entity, With<ElectromagneticAmplitude>>,
    wave_length_query: Query<Entity, With<WaveLength>>,
    frequency_query: Query<Entity, With<Freq>>,
    phase_query: Query<Entity, With<Phase>>,
) {
    for input in events.read() {
        // println!("got events in wave.rs: {:?}", input);
        match parse_float(&input.amplitude) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &amplitude_query);
                commands.spawn(ElectromagneticAmplitude(
                    ElectricField::new::<volt_per_meter>(f),
                ));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.wave_length) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &wave_length_query);
                commands.spawn(WaveLength(Length::new::<meter>(f)));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.frequency) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &frequency_query);
                commands.spawn(Freq(Frequency::new::<hertz>(f)));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.phase) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &phase_query);
                commands.spawn(Phase(Angle::new::<radian>(f)));
            }
            Err(err) => println!("error: {}", err),
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct ElectromagneticAmplitude(pub ElectricField);
