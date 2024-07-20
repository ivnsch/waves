use std::f32::consts::PI;

use crate::wave_gui::{
    focus, form_state_notifier_system, listen_wave_gui_inputs, setup_wave_gui, text_listener,
    Amplitude, Freq, GuiInputs, GuiInputsEvent, Phase, WaveLength,
};
use bevy::{color::palettes::css::WHITE, ecs::query::QuerySingleError, prelude::*};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use uom::si::{
    angle::radian,
    f32::{Frequency, Length},
    frequency::hertz,
    length::meter,
    time::second,
};

#[allow(dead_code)]
pub fn add_wave_2d_system(app: &mut App) {
    app.add_event::<GuiInputsEvent>()
        .add_plugins(TextInputPlugin)
        .insert_resource(GuiInputs {
            amplitude: "1".to_owned(),
            wave_length: "2".to_owned(),
            frequency: "0.5".to_owned(),
            phase: "0".to_owned(),
        })
        .add_systems(Startup, setup_wave_gui)
        .add_systems(Update, focus.before(TextInputSystem))
        .add_systems(
            Update,
            (
                draw_wave,
                listen_wave_gui_inputs,
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
    phase: Query<&Phase>,
) {
    match draw_wave_internal(gizmos, time, amplitude, wave_length, frequency, phase) {
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
    phase: Query<&Phase>,
) -> Result<(), QuerySingleError> {
    let user_pars = WaveUserParameters {
        amplitude: *amplitude.get_single()?,
        wave_length: *wave_length.get_single()?,
        frequency: *frequency.get_single()?,
        phase: *phase.get_single()?,
    };

    let range = 20;

    let t = uom::si::f32::Time::new::<second>(time.elapsed_seconds());
    // let t = uom::si::f32::Time::new::<second>(0);  // not animated

    let function = |x: f32| {
        let vec3 = calculate_u(Length::new::<meter>(x), t, &user_pars, Vec3::Y).to_vec3();
        Vec2::new(vec3.x, vec3.y)
    };

    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, WHITE, function);

    Ok(())
}

#[derive(Debug, Clone)]
pub struct WaveUserParameters {
    pub amplitude: Amplitude,
    pub wave_length: WaveLength,
    pub frequency: Freq,
    pub phase: Phase,
}

/// to reuse wave calculation for different domains (currently electromagnetic / non electromagnetic)
/// not meant to be instantiated directly
#[derive(Debug)]
pub struct RawUserParameters {
    pub amplitude: f32,
    pub wave_length: WaveLength,
    pub frequency: Freq,
    pub phase: Phase,
}

impl From<WaveUserParameters> for RawUserParameters {
    fn from(p: WaveUserParameters) -> Self {
        RawUserParameters {
            amplitude: p.amplitude.0.get::<meter>(),
            wave_length: p.wave_length,
            frequency: p.frequency,
            phase: p.phase,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn calculate_u(
    x: Length,
    t: uom::si::f32::Time,
    up: &WaveUserParameters,
    direction: Vec3,
) -> LengthVec3 {
    let raw = calculate_u_raw(x, t, &up.clone().into(), direction);
    // assumption: raw amplitude passed to calculate_u_raw (RawUserParameters) was in meter
    LengthVec3 {
        x: Length::new::<meter>(raw.x),
        y: Length::new::<meter>(raw.y),
        z: Length::new::<meter>(raw.z),
    }
}

#[derive(Debug)]
pub struct LengthVec3 {
    pub x: Length,
    pub y: Length,
    pub z: Length,
}

/// f32 vec
impl LengthVec3 {
    fn to_vec3(self) -> Vec3 {
        Vec3::new(
            self.x.get::<meter>(),
            self.y.get::<meter>(),
            self.z.get::<meter>(),
        )
    }
}

/// equation of travelling wave: u(x,t)=Acos(kx−ωt)
/// nice explanation https://physics.stackexchange.com/a/259007
/// returns a unit-less vector to be shared between electromagnetic and non electromagnetic wave
#[allow(clippy::too_many_arguments)]
pub fn calculate_u_raw(
    x: Length,
    t: uom::si::f32::Time,
    up: &RawUserParameters,
    unit_vector: Vec3,
) -> Vec3 {
    let screen_speed_pars = to_screen_speed(up);
    // println!("screen_speed_pars: {:?}", screen_speed_pars);

    // wave cycles per unit distance
    // there might be reciprocal units on uom? (1/meter here), for now implicit
    let k = 2.0 * PI / screen_speed_pars.wave_length.get::<meter>();

    let angular_frequency = 2.0 * PI * screen_speed_pars.frequency.get::<hertz>();

    let scalar = ((k * x.get::<meter>()) - (angular_frequency * t.get::<second>())
        + up.phase.0.get::<radian>())
    .cos();

    unit_vector * (up.amplitude * scalar)
}

#[derive(Debug)]
struct ScreenSpeedParameters {
    wave_length: Length,
    frequency: Frequency,
}

/// slow down for animation
fn to_screen_speed(up: &RawUserParameters) -> ScreenSpeedParameters {
    let speed_factor: f32 = 0.00000001;
    // v = fλ -> v * factor = (fλ) * factor
    ScreenSpeedParameters {
        // actually, scale down only frequency,
        // scaling down distance here will give very small decimals, which can't be rendered properly
        // there's probably a better solution for this (scaling the coordinate space perhaps), but this seems fine for now
        // wave_length: up.wave_length.0 * speed_factor,
        wave_length: up.wave_length.0,
        frequency: up.frequency.0 * speed_factor,
    }
}

// /// draws planar function as a sequence o/ }

/// draws planar function as a sequence of vectors
pub fn draw_planar_fn_as_vert_vecs<F>(
    gizmos: &mut Gizmos,
    range_start: i32,
    range_end: i32,
    color: Srgba,
    function: F,
) where
    F: Fn(f32) -> Vec2,
{
    let x_scaling = 50.0;
    let y_scaling = 50.0;

    let mut value = range_start as f32;
    while value < range_end as f32 {
        let x = value;
        let vec = function(x);

        let scaled_x = x * x_scaling;
        let scaled_y = vec.y * y_scaling;

        println!("x: {}, y: {}", scaled_x, scaled_y);

        gizmos.line_2d(
            Vec2::new(scaled_x, 0.0),
            Vec2::new(scaled_x, scaled_y),
            color,
        );

        value += 0.1;
    }
}
