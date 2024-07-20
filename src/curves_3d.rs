use crate::functions::draw_line_fn;
use bevy::{color::palettes::css::WHITE, prelude::*};

#[allow(dead_code)]
pub fn add_curves_3d_system(app: &mut App) {
    app.add_systems(Update, draw_square_fn);
    // app.add_systems(Update, draw_sin_as_vert_vecs);
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
    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, WHITE, |x| Vec3 {
        x: 0.0,
        y: 0.0,
        z: x.sin(),
    });
    // animate
    // let t = time.elapsed_seconds();
    // draw_fn(gizmos, -10 + t as i32, 10 + t as i32, |x| x.sin());
}

/// draws planar function as a sequence of vectors
pub fn draw_planar_fn_as_vert_vecs<F>(
    gizmos: &mut Gizmos,
    range_start: i32,
    range_end: i32,
    color: Srgba,
    function: F,
) where
    F: Fn(f32) -> Vec3,
{
    let x_scaling = 0.2;
    let z_scaling = 0.2;
    let y_scaling = 0.2;

    let mut value = range_start as f32;
    while value < range_end as f32 {
        let x = value;
        let vec = function(x);

        let scaled_x = x * x_scaling;
        let scaled_z = vec.z * z_scaling;
        let scaled_y = vec.y * y_scaling;

        // println!("x: {}, y: {}, z: {}", scaled_x, scaled_y, scaled_z);

        gizmos.line(
            Vec3::new(scaled_x, 0.0, 0.0),
            Vec3::new(scaled_x, scaled_z, scaled_y),
            color,
        );

        value += 0.1;
    }
}
