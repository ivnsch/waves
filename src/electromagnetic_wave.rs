use bevy::{
    color::palettes::css::{GREEN, WHITE},
    ecs::query::QuerySingleError,
    prelude::*,
};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use uom::si::{
    electric_field::volt_per_meter,
    f32::{ElectricField, Length},
    length::meter,
    time::second,
};

use crate::{
    curves_3d::draw_planar_fn_as_vert_vecs,
    electromagnetic_wave_gui::{
        listen_electromagnetic_wave_gui_inputs, setup_electromagnetic_wave_gui,
        ElectromagneticAmplitude,
    },
    wave::{calculate_u_raw, RawUserParameters},
    wave_gui::{
        focus, form_state_notifier_system, setup_wave_gui, text_listener,
        AngularFrequencyCoefficient, Freq, GuiInputs, GuiInputsEvent, KCoefficient, Phase,
        WaveLength,
    },
};

const SPEED_OF_LIGHT: f64 = 299_792_458.0; // m / s

#[allow(dead_code)]
pub fn add_electromagnetic_wave(app: &mut App) {
    let wave_length = 1.0; // 1 meter

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
                listen_electromagnetic_wave_gui_inputs,
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
    amplitude: Query<&ElectromagneticAmplitude>,
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
    amplitude: Query<&ElectromagneticAmplitude>,
    wave_length: Query<&WaveLength>,
    frequency: Query<&Freq>,
    k_coefficient: Query<&KCoefficient>,
    angular_frequency_coefficient: Query<&AngularFrequencyCoefficient>,
    phase: Query<&Phase>,
) -> Result<(), QuerySingleError> {
    let user_pars = ElectromagneticWaveUserParameters {
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
        |x: f32| calculate_u(Length::new::<meter>(x), t, &user_pars).get::<volt_per_meter>();

    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, true, WHITE, function);
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, false, GREEN, function);

    Ok(())
}

#[derive(Debug, Clone)]
pub struct ElectromagneticWaveUserParameters {
    pub amplitude: ElectromagneticAmplitude,
    pub wave_length: WaveLength,
    pub frequency: Freq,
    pub k_coefficient: KCoefficient,
    pub angular_frequency_coefficient: AngularFrequencyCoefficient,
    pub phase: Phase,
}

impl From<ElectromagneticWaveUserParameters> for RawUserParameters {
    fn from(p: ElectromagneticWaveUserParameters) -> Self {
        RawUserParameters {
            amplitude: p.amplitude.0.get::<volt_per_meter>(),
            wave_length: p.wave_length,
            frequency: p.frequency,
            k_coefficient: p.k_coefficient,
            angular_frequency_coefficient: p.angular_frequency_coefficient,
            phase: p.phase,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn calculate_u(
    x: Length,
    t: uom::si::f32::Time,
    up: &ElectromagneticWaveUserParameters,
) -> ElectricField {
    ElectricField::new::<volt_per_meter>(calculate_u_raw(x, t, &up.clone().into()))
}
