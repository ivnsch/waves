use std::f32::consts::PI;

use crate::wave_gui::{
    focus, form_state_notifier_system, listen_gui_inputs, setup_wave_gui, text_listener, Amplitude,
    AngularFrequencyCoefficient, Freq, GuiInputs, GuiInputsEvent, KCoefficient, Phase, WaveLength,
};
use bevy::{ecs::query::QuerySingleError, prelude::*};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use uom::si::{angle::radian, f32::Length, frequency::hertz, length::kilometer, time::second};

#[allow(dead_code)]
pub fn add_wave_2d_system(app: &mut App) {
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
        .add_systems(Startup, setup_wave_gui)
        .add_systems(Update, focus.before(TextInputSystem))
        .add_systems(
            Update,
            (
                draw_wave,
                listen_gui_inputs,
                text_listener,
                form_state_notifier_system,
            ),
        );
}

#[allow(clippy::too_many_arguments)]
fn draw_wave(
    gizmos: Gizmos,
    time: Res<Time>,
    amplitude: Query<&Amplitude>,
    wave_length: Query<&WaveLength>,
    frequency: Query<&Freq>,
    k_coefficient: Query<&KCoefficient>,
    angular_frequency_coefficient: Query<&AngularFrequencyCoefficient>,
    phase: Query<&Phase>,
) {
    match draw_wave_internal(
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
fn draw_wave_internal(
    mut gizmos: Gizmos,
    time: Res<Time>,
    amplitude: Query<&Amplitude>,
    wave_length: Query<&WaveLength>,
    frequency: Query<&Freq>,
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

    let t = uom::si::f32::Time::new::<second>(time.elapsed_seconds());
    // let t = uom::si::f32::Time::new::<second>(0);  // not animated

    // equation of travelling wave: u(x,t)=Acos(kx−ωt)
    // nice explanation https://physics.stackexchange.com/a/259007
    let function = |x: f32| {
        calculate_u(
            Length::new::<kilometer>(x),
            t,
            amplitude,
            wave_length,
            frequency,
            k_coefficient,
            angular_frequency_coefficient,
            phase,
        )
        .get::<kilometer>()
    };

    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, Color::WHITE, function);

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn calculate_u(
    x: Length,
    t: uom::si::f32::Time,
    amplitude: &Amplitude,
    wave_length: &WaveLength,
    frequency: &Freq,
    k_coefficient: &KCoefficient,
    angular_frequency_coefficient: &AngularFrequencyCoefficient,
    phase: &Phase,
) -> Length {
    let k: Length =
        Length::new::<kilometer>(k_coefficient.0 * PI / wave_length.0.get::<kilometer>()); // wave cycles per unit distance

    let angular_frequency = angular_frequency_coefficient.0 * PI * frequency.0.get::<hertz>();

    let scalar = ((k.get::<kilometer>() * x.get::<kilometer>())
        - (angular_frequency * t.get::<second>())
        + phase.0.get::<radian>())
    .cos();

    Length::new::<kilometer>(amplitude.0.get::<kilometer>() * scalar)
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
        let x = value;
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
    gizmos.line_2d(Vec2::new(x, 0.0), Vec2::new(x, y), color);
}
