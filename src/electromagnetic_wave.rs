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
    electromagnetic_wave_ui::{
        listen_electromagnetic_wave_ui_inputs, listen_polarity_ui_inputs,
        polarity_circular_button_handler, polarity_planar_button_handler,
        setup_electromagnetic_wave_infos, setup_electromagnetic_wave_ui, ElectromagneticAmplitude,
        PolarityInput, PolarityInputEvent,
    },
    wave::{calculate_u_raw, calculate_u_scalar_raw, RawUserParameters},
    wave_ui::{
        focus, form_state_notifier_system, text_listener, Freq, Phase, UiInputs, UiInputsEvent,
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

    app.add_event::<UiInputsEvent>()
        .add_event::<PolarityInputEvent>()
        .add_plugins(TextInputPlugin)
        .insert_resource(UiInputs {
            amplitude: "1".to_owned(),
            wave_length: wave_length.get::<meter>().to_string(),
            frequency: frequency.get::<hertz>().to_string(),
            phase: "0".to_owned(),
        })
        .insert_resource(PolarityInput::Planar)
        .add_systems(Update, focus.before(TextInputSystem))
        .add_systems(
            Update,
            (
                draw_planar_polarity_electromagnetic_wave
                    .pipe(handle_draw_electromagnetic_wave_result)
                    .run_if(is_planar_polarity_selected),
                draw_circular_polarity_electromagnetic_wave
                    .pipe(handle_draw_electromagnetic_wave_result)
                    .run_if(is_circular_polarity_selected),
                listen_electromagnetic_wave_ui_inputs,
                text_listener,
                form_state_notifier_system,
                validate_inputs.pipe(handle_validate_inputs_result),
                polarity_planar_button_handler,
                polarity_circular_button_handler,
                listen_polarity_ui_inputs,
            ),
        )
        .add_systems(Startup, setup_electromagnetic_wave_infos)
        .add_systems(Startup, setup_electromagnetic_wave_ui);
}

fn calculate_frequency(wave_length: Length) -> Frequency {
    *SPEED_OF_LIGHT / wave_length
}

#[allow(dead_code)]
fn calculate_wave_length(frequency: Frequency) -> Length {
    *SPEED_OF_LIGHT / frequency
}

fn is_planar_polarity_selected(polarity: Res<PolarityInput>) -> bool {
    match *polarity {
        PolarityInput::Planar => true,
        _ => false,
    }
}

fn is_circular_polarity_selected(polarity: Res<PolarityInput>) -> bool {
    match *polarity {
        PolarityInput::Circular => true,
        _ => false,
    }
}

fn handle_validate_inputs_result(In(result): In<Result<(), QuerySingleError>>) {
    match result {
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

fn validate_inputs(
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
fn handle_draw_electromagnetic_wave_result(In(result): In<Result<(), QuerySingleError>>) {
    match result {
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
fn draw_planar_polarity_electromagnetic_wave(
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
        calculate_u_planar(Length::new::<meter>(x), t, &user_pars, Vec3::Z).to_vec3()
    });

    // magnetic
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, GREEN, |x: f32| {
        calculate_u_planar(Length::new::<meter>(x), t, &user_pars, Vec3::Y).to_vec3()
    });

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn draw_circular_polarity_electromagnetic_wave(
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
        calculate_u_circular(
            Length::new::<meter>(x),
            t,
            &user_pars,
            Vec3::Y,
            Vec3::Z,
            true,
        )
        .to_vec3()
    });

    // magnetic
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, GREEN, |x: f32| {
        calculate_u_circular(
            Length::new::<meter>(x),
            t,
            &user_pars,
            Vec3::Z,
            Vec3::Y,
            false,
        )
        .to_vec3()
    });

    Ok(())
}

/// parameters the user sets via the ui
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

/// calculates wave function for a planar polarity wave
#[allow(clippy::too_many_arguments)]
fn calculate_u_planar(
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

/// calculates wave function for a circular polarity wave
/// u(x, y) = A(cos(kx - wt)y + sin (kx - wt)z)
/// see e.g. https://web.mit.edu/sahughes/www/8.022/lec21.pdf section 21.5
#[allow(clippy::too_many_arguments)]
fn calculate_u_circular(
    x: Length,
    t: uom::si::f32::Time,
    up: &ElectromagneticWaveUserParameters,
    unit_vector1: Vec3,
    unit_vector2: Vec3,
    sign_sin_cos: bool, // true for + between cos and sin terms, false for -
) -> ElectricFieldVec3 {
    let scalar = calculate_u_scalar_raw(x, t, &up.clone().into());
    let cos = scalar.cos();
    let sin = scalar.sin();

    let term1 = unit_vector1 * cos;
    let mut term2 = unit_vector2 * sin;

    if !sign_sin_cos {
        term2 = -term2;
    }
    let sub = term1 + term2;

    ElectricFieldVec3 {
        x: up.amplitude.0 * sub.x,
        y: up.amplitude.0 * sub.y,
        z: up.amplitude.0 * sub.z,
    }
}

/// 3d vector with domain specific units
#[derive(Debug)]
pub struct ElectricFieldVec3 {
    pub x: ElectricField,
    pub y: ElectricField,
    pub z: ElectricField,
}

/// f32 vec
impl ElectricFieldVec3 {
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            self.x.get::<volt_per_meter>(),
            self.y.get::<volt_per_meter>(),
            self.z.get::<volt_per_meter>(),
        )
    }
}
