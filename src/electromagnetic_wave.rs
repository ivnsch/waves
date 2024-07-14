use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{GREEN, WHITE},
    ecs::query::QuerySingleError,
    prelude::*,
};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

use crate::{
    curves_3d::draw_planar_fn_as_vert_vecs,
    electromagnetic_wave_gui::setup_electromagnetic_wave_gui,
    wave_gui::{
        focus, form_state_notifier_system, listen_gui_inputs, setup_wave_gui, text_listener,
        Amplitude, AngularFrequencyCoefficient, Frequency, GuiInputs, GuiInputsEvent, KCoefficient,
        Phase, WaveLength,
    },
};

#[allow(dead_code)]
pub fn add_electromagnetic_wave(app: &mut App) {
    app.add_event::<GuiInputsEvent>()
        .add_plugins(TextInputPlugin)
        .insert_resource(GuiInputs {
            amplitude: "1".to_owned(),
            wave_length: "2".to_owned(),
            frequency: "0.5".to_owned(),
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

#[allow(clippy::too_many_arguments)]
fn draw_electromagnetic_wave(
    gizmos: Gizmos,
    time: Res<Time>,
    amplitude: Query<&Amplitude>,
    wave_length: Query<&WaveLength>,
    frequency: Query<&Frequency>,
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
    frequency: Query<&Frequency>,
    k_coefficient: Query<&KCoefficient>,
    angular_frequency_coefficient: Query<&AngularFrequencyCoefficient>,
    phase: Query<&Phase>,
) -> Result<(), QuerySingleError> {
    let amplitude = amplitude.get_single()?;
    let wave_length = wave_length.get_single()?;
    let frequency = frequency.get_single()?;
    let k_coefficient = k_coefficient.get_single()?;
    let angular_frequency_coefficient = angular_frequency_coefficient.get_single()?;
    let phase = phase.get_single()?;

    let range = 20;

    let t = time.elapsed_seconds();
    // let t = 0.0; // not animated

    // equation of travelling wave: u(x,t)=Acos(kx−ωt)
    // nice explanation https://physics.stackexchange.com/a/259007
    let function = |x: f32| {
        let k = k_coefficient.0 * PI / wave_length.0; // wave cycles per unit distance
        let angular_frequency = angular_frequency_coefficient.0 * PI * frequency.0;
        let scalar = ((k * x) - angular_frequency * t + phase.0).cos();

        amplitude.0 * scalar
    };

    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, true, WHITE, function);
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, false, GREEN, function);

    Ok(())
}
