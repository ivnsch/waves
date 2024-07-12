use std::f32::consts::PI;

use bevy::prelude::*;

use crate::functions::draw_line_fn;

#[allow(dead_code)]
pub fn add_curves_3d_system(app: &mut App) {
    // app.add_systems(Update, draw_square_fn);
    // app.add_systems(Update, draw_sin_as_vert_vecs);
    app.add_systems(Update, draw_electromagnetic_wave);
}

#[allow(dead_code)]
fn draw_square_fn(mut gizmos: Gizmos) {
    draw_line_fn(&mut gizmos, -10, 10, 1, 0.2, |x| x * x);
}

#[allow(dead_code)]
fn draw_sin_fn(mut gizmos: Gizmos, _time: Res<Time>) {
    draw_line_fn(&mut gizmos, -10, 10, 1, 0.2, |x| x.sin());
    // animate
    // let t = time.elapsed_seconds();
    // draw_fn(gizmos, -10 + t as i32, 10 + t as i32, |x| x.sin());
}

#[allow(dead_code)]
fn draw_sin_as_vert_vecs(mut gizmos: Gizmos, _time: Res<Time>) {
    let range = 20;
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, true, Color::WHITE, |x| x.sin());
    // animate
    // let t = time.elapsed_seconds();
    // draw_fn(gizmos, -10 + t as i32, 10 + t as i32, |x| x.sin());
}

fn draw_electromagnetic_wave(mut gizmos: Gizmos, time: Res<Time>) {
    let range = 20;

    let t = time.elapsed_seconds() as f32;
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

    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, true, Color::WHITE, function);
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, false, Color::GREEN, function);
}

/// draws planar function as a sequence of vectors,
/// planar here meaning specifically on xz plane (parallel_z == true) or xy plane (parallel_z == false)
fn draw_planar_fn_as_vert_vecs<F>(
    gizmos: &mut Gizmos,
    range_start: i32,
    range_end: i32,
    parallel_z: bool, // for now just z (true), y (false)
    color: Color,
    function: F,
) where
    F: Fn(f32) -> f32,
{
    let x_scaling = 0.2;
    let z_scaling = 0.2;
    let y_scaling = 0.2;

    let mut last_point = None;

    let mut value = range_start as f32;
    while value < range_end as f32 {
        let x = value as f32;
        let z = function(x);
        let y = 0.0;
        let (z, y) = if parallel_z { (z, y) } else { (y, z) };

        if let Some((last_x, last_z)) = last_point {
            vert_x_arrow_out(
                last_x * x_scaling,
                last_z * z_scaling,
                y * y_scaling,
                gizmos,
                color,
            );
            vert_x_arrow_out(x * x_scaling, z * z_scaling, y * y_scaling, gizmos, color);
        }

        last_point = Some((x, z));
        value += 0.1;
    }
}

fn vert_x_arrow_out(x: f32, y: f32, z: f32, gizmos: &mut Gizmos, color: Color) {
    gizmos.arrow(Vec3::new(x, 0.0, 0.0), Vec3::new(x, y, z), color);
}
