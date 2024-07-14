use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{GREEN, WHITE},
    prelude::*,
};

use crate::{
    curves_3d::draw_planar_fn_as_vert_vecs,
    electromagnetic_wave_gui::setup_electromagnetic_wave_gui,
};

#[allow(dead_code)]
pub fn add_electromagnetic_wave(app: &mut App) {
    app.add_systems(Update, draw_electromagnetic_wave)
        .add_systems(Startup, setup_electromagnetic_wave_gui);
}

fn draw_electromagnetic_wave(mut gizmos: Gizmos, time: Res<Time>) {
    let range = 20;

    let t = time.elapsed_seconds();
    // let t = 0.0; // not animated

    let function = |x: f32| {
        // for now not a vector. to draw the electric vs magnetic wave we just change parallel_z parameter
        let amplitude = 1.0;
        let wave_length = 3.0;
        let k = 2.0 * PI / wave_length;
        let frequency = 0.5;
        let angular_frequency = 2.0 * PI * frequency;
        let phase = 0.0;
        let scalar = ((k * x) - angular_frequency * t + phase).cos();
        // if (x % 20.0).abs() < 0.01 && x > 20.0 {
        // println!("t: {}, res: {}, x: {}", t, amplitude * scalar, x);
        // }

        amplitude * scalar
    };

    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, true, WHITE, function);
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, false, GREEN, function);
}
