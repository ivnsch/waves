use bevy::{
    color::palettes::css::{GREEN, WHITE},
    ecs::query::QuerySingleError,
    prelude::*,
};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use uom::si::{f32::Length, length::megameter, time::second};

use crate::{
    curves_3d::draw_planar_fn_as_vert_vecs,
    electromagnetic_wave_gui::setup_electromagnetic_wave_gui,
    wave::{calculate_u, UserParameters},
    wave_gui::{
        focus, form_state_notifier_system, listen_gui_inputs, setup_wave_gui, text_listener,
        Amplitude, AngularFrequencyCoefficient, Freq, GuiInputs, GuiInputsEvent, KCoefficient,
        Phase, WaveLength,
    },
};

// unit implicitly defined as 1 megameter (1000 km)
const SPEED_OF_LIGHT: f64 = 300.0; // 300 * 1000 km / s

#[allow(dead_code)]
pub fn add_electromagnetic_wave(app: &mut App) {
    let wave_length = 2.0; // 2 megameters

    // ensure c=fλ
    // note: for now *not* correcting new user inputs to speed of light
    let frequency = calculate_frequency(wave_length);

    app.add_event::<GuiInputsEvent>()
        .add_plugins(TextInputPlugin)
        .insert_resource(GuiInputs {
            amplitude: "1".to_owned(),
            wave_length: wave_length.to_string(),
            frequency: frequency.to_string(),
            k_coefficient: "2".to_owned(),
            angular_frequency_coefficient: "2".to_owned(),
            phase: "0".to_owned(),
        })
        .add_systems(Update, focus.before(TextInputSystem))
        .add_systems(
            Update,
            (
                draw_electromagnetic_wave,
                listen_gui_inputs,
                text_listener,
                form_state_notifier_system,
            ),
        )
        .add_systems(Startup, setup_electromagnetic_wave_gui)
        .add_systems(Startup, setup_wave_gui);
}

fn calculate_frequency(wave_length: f64) -> f64 {
    SPEED_OF_LIGHT / wave_length
}

#[allow(dead_code)]
fn calculate_wave_length(frequency: f64) -> f64 {
    SPEED_OF_LIGHT / frequency
}

#[allow(clippy::too_many_arguments)]
fn draw_electromagnetic_wave(
    gizmos: Gizmos,
    time: Res<Time>,
    amplitude: Query<&Amplitude>,
    wave_length: Query<&WaveLength>,
    frequency: Query<&Freq>,
    k_coefficient: Query<&KCoefficient>,
    angular_frequency_coefficient: Query<&AngularFrequencyCoefficient>,
    phase: Query<&Phase>,
) {
    match draw_electromagnetic_wave_internal(
        gizmos,
        time,
        amplitude,
        wave_length,
        frequency,
        k_coefficient,
        angular_frequency_coefficient,
        phase,
    ) {
        Ok(_) => {}
        Err(e) => match e {
            QuerySingleError::NoEntities(s) => {
                // this is logged 2x at the beginning (even if we set defaults in insert_resource). doesn't seem to be an issue.
                // after that it shouldn't appear again, because each field should always have a value.
                info!("No entity added yet: {}", s)
            }
            QuerySingleError::MultipleEntities(s) => {
                error!("Found multiple entities of a type: {}", s)
            }
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_electromagnetic_wave_internal(
    mut gizmos: Gizmos,
    time: Res<Time>,
    amplitude: Query<&Amplitude>,
    wave_length: Query<&WaveLength>,
    frequency: Query<&Freq>,
    k_coefficient: Query<&KCoefficient>,
    angular_frequency_coefficient: Query<&AngularFrequencyCoefficient>,
    phase: Query<&Phase>,
) -> Result<(), QuerySingleError> {
    let user_pars = UserParameters {
        amplitude: *amplitude.get_single()?,
        wave_length: *wave_length.get_single()?,
        frequency: *frequency.get_single()?,
        k_coefficient: *k_coefficient.get_single()?,
        angular_frequency_coefficient: *angular_frequency_coefficient.get_single()?,
        phase: *phase.get_single()?,
    };

    let range = 20;

    let t = uom::si::f32::Time::new::<second>(time.elapsed_seconds());
    // let t = uom::si::f32::Time::new::<second>(0);  // not animated

    let function =
        |x: f32| calculate_u(Length::new::<megameter>(x), t, &user_pars).get::<megameter>();

    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, true, WHITE, function);
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, false, GREEN, function);

    Ok(())
}
