use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_simple_text_input::{TextInputInactive, TextInputPlugin};

use crate::wave_gui::{
    form_state_notifier_system, setup_wave_gui, text_listener, GuiInputs, GuiInputsEvent,
};

#[allow(dead_code)]
pub fn add_wave_2d_system(app: &mut App) {
    app.add_event::<GuiInputsEvent>()
        .add_plugins(TextInputPlugin)
        .insert_resource(GuiInputs {
            amplitude: "1".to_owned(),
            wave_length: "2".to_owned(),
        })
        .add_systems(Startup, setup_wave_gui)
        .add_systems(
            Update,
            (
                draw_wave,
                listen_gui_inputs,
                text_listener,
                focus,
                form_state_notifier_system,
            ),
        );
}

fn draw_wave(
    mut gizmos: Gizmos,
    time: Res<Time>,
    amplitude: Query<&Amplitude>,
    wave_length: Query<&WaveLength>,
) {
    for amplitude in amplitude.iter() {
        for wave_length in wave_length.iter() {
            let range = 20;

            let t = time.elapsed_seconds() as f32;
            // let t = 0.0; // not animated

            // equation of travelling wave: u(x,t)=Acos(kx−ωt)
            // nice explanation https://physics.stackexchange.com/a/259007
            let function = |x: f32| {
                // let amplitude = 1.0;
                // let wave_length = 3.0;
                let k = 2.0 * PI / wave_length.0; // wave cycles per unit distance
                                                  // let k = 2.0 * PI / wave_length.0; // wave cycles per unit distance
                let frequency = 0.5;
                let angular_frequency = 2.0 * PI * frequency;
                let phase = 0.0;
                let scalar = ((k * x) - angular_frequency * t + phase).cos();

                amplitude.0 * scalar
            };

            draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, Color::WHITE, function);
        }
    }
}

/// draws planar function as a sequence of vectors,
fn draw_planar_fn_as_vert_vecs<F>(
    gizmos: &mut Gizmos,
    range_start: i32,
    range_end: i32,
    color: Color,
    function: F,
) where
    F: Fn(f32) -> f32,
{
    let scaling = 50.0;
    let x_scaling = scaling;
    let y_scaling = scaling;

    let mut last_point = None;

    let mut value = range_start as f32;
    while value < range_end as f32 {
        let x = value as f32;
        let y = function(x);

        if let Some((last_x, last_y)) = last_point {
            vert_x_arrow_out(last_x * x_scaling, last_y * y_scaling, gizmos, color);
            vert_x_arrow_out(x * x_scaling, y * y_scaling, gizmos, color);
        }

        last_point = Some((x, y));
        value += 0.1;
    }
}

fn vert_x_arrow_out(x: f32, y: f32, gizmos: &mut Gizmos, color: Color) {
    gizmos.arrow_2d(Vec2::new(x, 0.0), Vec2::new(x, y), color);
}

fn listen_gui_inputs(
    mut events: EventReader<GuiInputsEvent>,
    mut commands: Commands,
    query: Query<Entity, With<Amplitude>>,
    query_w: Query<Entity, With<WaveLength>>,
) {
    for input in events.read() {
        // println!("got events in wave.rs: {:?}", input);
        match process_amplitude_str(&input.amplitude) {
            Ok(a) => {
                for e in query.iter() {
                    commands.entity(e).despawn_recursive();
                }
                commands.spawn(a);
            }
            Err(err) => println!("error: {}", err), // TODO error handling
        }
        match process_wave_length_str(&input.wave_length) {
            Ok(w) => {
                for e in query_w.iter() {
                    commands.entity(e).despawn_recursive();
                }
                commands.spawn(w);
            }
            Err(err) => println!("error: {}", err), // TODO error handling
        }
    }
}

fn process_amplitude_str(str: &str) -> Result<Amplitude, String> {
    let a = str.parse::<f32>();
    match a {
        Ok(a) => Ok(Amplitude(a)),
        Err(e) => Err(format!("Failed to parse input: {}", e)),
    }
}

fn process_wave_length_str(str: &str) -> Result<WaveLength, String> {
    let a = str.parse::<f32>();
    match a {
        Ok(a) => Ok(WaveLength(a)),
        Err(e) => Err(format!("Failed to parse input: {}", e)),
    }
}

fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = Color::BLUE.into();
                } else {
                    inactive.0 = true;
                    *border_color = Color::GRAY.into();
                }
            }
        }
    }
}

#[derive(Component, Debug)]
struct Amplitude(f32);

#[derive(Component, Debug)]
struct WaveLength(f32);
