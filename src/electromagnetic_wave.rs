use bevy::{
    color::palettes::css::{GREEN, WHITE},
    ecs::query::QuerySingleError,
    prelude::*,
};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use once_cell::sync::Lazy;
use uom::si::{
    electric_field::volt_per_meter,
    f32::{ElectricField, Frequency, Length, Velocity},
    frequency::hertz,
    length::meter,
    time::second,
    velocity::meter_per_second,
};

use crate::{
    curves_3d::draw_planar_fn_as_vert_vecs,
    electromagnetic_wave_gui::{
        listen_electromagnetic_wave_gui_inputs, setup_electromagnetic_wave_gui,
        setup_electromagnetic_wave_infos, ElectromagneticAmplitude,
    },
    wave::{calculate_u_raw, RawUserParameters},
    wave_gui::{
        focus, form_state_notifier_system, text_listener, Freq, GuiInputs, GuiInputsEvent, Phase,
        WarningMarker, WaveLength,
    },
};

static SPEED_OF_LIGHT: Lazy<Velocity> =
    Lazy::new(|| Velocity::new::<meter_per_second>(299_792_458.0));

#[allow(dead_code)]
pub fn add_electromagnetic_wave(app: &mut App) {
    let wave_length = Length::new::<meter>(1.0);

    // ensure c=fλ
    // note: for now *not* correcting new user inputs to speed of light
    let frequency = calculate_frequency(wave_length);

    app.add_event::<GuiInputsEvent>()
        .add_plugins(TextInputPlugin)
        .insert_resource(GuiInputs {
            amplitude: "1".to_owned(),
            wave_length: wave_length.get::<meter>().to_string(),
            frequency: frequency.get::<hertz>().to_string(),
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
                validate_inputs,
            ),
        )
        .add_systems(Startup, setup_electromagnetic_wave_infos)
        .add_systems(Startup, setup_electromagnetic_wave_gui);
}

fn calculate_frequency(wave_length: Length) -> Frequency {
    *SPEED_OF_LIGHT / wave_length
}

#[allow(dead_code)]
fn calculate_wave_length(frequency: Frequency) -> Length {
    *SPEED_OF_LIGHT / frequency
}

fn validate_inputs(
    frequency: Query<&Freq>,
    wave_length: Query<&WaveLength>,
    warning_query: Query<&mut Text, With<WarningMarker>>,
) {
    match validate_inputs_internal(frequency, wave_length, warning_query) {
        Ok(_) => {}
        Err(e) => match e {
            QuerySingleError::NoEntities(s) => {
                info!("No entity added yet: {}", s)
            }
            QuerySingleError::MultipleEntities(s) => {
                error!("Found multiple entities of a type: {}", s)
            }
        },
    }
}

fn validate_inputs_internal(
    frequency: Query<&Freq>,
    wave_length: Query<&WaveLength>,
    mut warning_query: Query<&mut Text, With<WarningMarker>>,
) -> Result<(), QuerySingleError> {
    let wave_length = wave_length.get_single()?;
    let frequency = frequency.get_single()?;

    let speed = frequency.0 * wave_length.0;

    let factor = speed / *SPEED_OF_LIGHT;

    let factor_number = factor.value;
    let speed_number = speed.get::<meter_per_second>();

    let warning = if speed != *SPEED_OF_LIGHT {
        Some(format!(
            "{}x speed of light, speed: {} m/s",
            factor_number, speed_number
        ))
    } else {
        None
    };

    let mut warning_label: Mut<Text> = warning_query.get_single_mut()?;
    match warning {
        Some(warning) => {
            warning_label.sections[0].value = warning.to_string();
        }
        None => {
            if !warning_label.sections[0].value.is_empty() {
                warning_label.sections[0].value = "".to_string();
            }
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn draw_electromagnetic_wave(
    gizmos: Gizmos,
    time: Res<Time>,
    amplitude: Query<&ElectromagneticAmplitude>,
    wave_length: Query<&WaveLength>,
    frequency: Query<&Freq>,
    phase: Query<&Phase>,
) {
    match draw_electromagnetic_wave_internal(gizmos, time, amplitude, wave_length, frequency, phase)
    {
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
    phase: Query<&Phase>,
) -> Result<(), QuerySingleError> {
    let user_pars = ElectromagneticWaveUserParameters {
        amplitude: *amplitude.get_single()?,
        wave_length: *wave_length.get_single()?,
        frequency: *frequency.get_single()?,
        phase: *phase.get_single()?,
    };

    let range = 20;

    let t = uom::si::f32::Time::new::<second>(time.elapsed_seconds());
    // let t = uom::si::f32::Time::new::<second>(0);  // not animated

    // electric
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, WHITE, |x: f32| {
        calculate_u(Length::new::<meter>(x), t, &user_pars, Vec3::Z).to_vec3()
    });

    // magnetic
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, GREEN, |x: f32| {
        calculate_u(Length::new::<meter>(x), t, &user_pars, Vec3::Y).to_vec3()
    });

    Ok(())
}

#[derive(Debug, Clone)]
pub struct ElectromagneticWaveUserParameters {
    pub amplitude: ElectromagneticAmplitude,
    pub wave_length: WaveLength,
    pub frequency: Freq,
    pub phase: Phase,
}

impl From<ElectromagneticWaveUserParameters> for RawUserParameters {
    fn from(p: ElectromagneticWaveUserParameters) -> Self {
        RawUserParameters {
            amplitude: p.amplitude.0.get::<volt_per_meter>(),
            wave_length: p.wave_length,
            frequency: p.frequency,
            phase: p.phase,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn calculate_u(
    x: Length,
    t: uom::si::f32::Time,
    up: &ElectromagneticWaveUserParameters,
    direction: Vec3,
) -> ElectricFieldVec3 {
    let raw = calculate_u_raw(x, t, &up.clone().into(), direction);
    // assumption: raw amplitude passed to calculate_u_raw (RawUserParameters) was in volt_per_meter
    ElectricFieldVec3 {
        x: ElectricField::new::<volt_per_meter>(raw.x),
        y: ElectricField::new::<volt_per_meter>(raw.y),
        z: ElectricField::new::<volt_per_meter>(raw.z),
    }
}

#[derive(Debug)]
pub struct ElectricFieldVec3 {
    pub x: ElectricField,
    pub y: ElectricField,
    pub z: ElectricField,
}

/// f32 vec
impl ElectricFieldVec3 {
    fn to_vec3(self) -> Vec3 {
        Vec3::new(
            self.x.get::<volt_per_meter>(),
            self.y.get::<volt_per_meter>(),
            self.z.get::<volt_per_meter>(),
        )
    }
}
