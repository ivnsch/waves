use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{BLUE, GRAY},
    ecs::query::QuerySingleError,
    prelude::*,
};
use bevy_simple_text_input::{TextInputInactive, TextInputPlugin, TextInputSystem};

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
    frequency: Query<&Frequency>,
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

    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, Color::WHITE, function);

    Ok(())
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

// TODO error handling (show on ui)
#[allow(clippy::too_many_arguments)]
fn listen_gui_inputs(
    mut events: EventReader<GuiInputsEvent>,
    mut commands: Commands,
    amplitude_query: Query<Entity, With<Amplitude>>,
    wave_length_query: Query<Entity, With<WaveLength>>,
    frequency_query: Query<Entity, With<Frequency>>,
    k_coefficient_query: Query<Entity, With<KCoefficient>>,
    angular_frequency_coefficient_query: Query<Entity, With<AngularFrequencyCoefficient>>,
    phase_query: Query<Entity, With<Phase>>,
) {
    for input in events.read() {
        // println!("got events in wave.rs: {:?}", input);
        match parse_float(&input.amplitude) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &amplitude_query);
                commands.spawn(Amplitude(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.wave_length) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &wave_length_query);
                commands.spawn(WaveLength(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.frequency) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &frequency_query);
                commands.spawn(Frequency(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.k_coefficient) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &k_coefficient_query);
                commands.spawn(KCoefficient(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.angular_frequency_coefficient) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &angular_frequency_coefficient_query);
                commands.spawn(AngularFrequencyCoefficient(f));
            }
            Err(err) => println!("error: {}", err),
        }
        match parse_float(&input.phase) {
            Ok(f) => {
                despawn_all_entities(&mut commands, &phase_query);
                commands.spawn(Phase(f));
            }
            Err(err) => println!("error: {}", err),
        }
    }
}

fn despawn_all_entities<T>(commands: &mut Commands, query: &Query<Entity, With<T>>)
where
    T: Component,
{
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn parse_float(str: &str) -> Result<f32, String> {
    let f = str.parse::<f32>();
    match f {
        Ok(f) => Ok(f),
        Err(e) => Err(format!("Failed to parse float: {}", e)),
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
                    *border_color = BLUE.into();
                } else {
                    inactive.0 = true;
                    *border_color = GRAY.into();
                }
            }
        }
    }
}

#[derive(Component, Debug)]
struct Amplitude(f32);

#[derive(Component, Debug)]
struct WaveLength(f32);

#[derive(Component, Debug)]
struct Frequency(f32);

#[derive(Component, Debug)]
struct KCoefficient(f32);

#[derive(Component, Debug)]
struct AngularFrequencyCoefficient(f32);

#[derive(Component, Debug)]
struct Phase(f32);
