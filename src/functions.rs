use bevy::{
    color::palettes::css::WHITE,
    math::{Vec2, Vec3},
    prelude::Gizmos,
};

/// draws function as a line
pub fn draw_line_fn<F>(
    gizmos: &mut Gizmos,
    range_start: i32,
    range_end: i32,
    step_size: usize,
    scaling: f32,
    function: F,
) where
    F: Fn(f32) -> f32,
{
    let x_scaling = scaling;
    let z_scaling = scaling;

    let mut last_point = None;

    for x_int in (range_start..range_end).step_by(step_size) {
        let x = x_int as f32;
        let z = function(x);
        let y = 0.0;

        if let Some((last_x, last_z)) = last_point {
            gizmos.line(
                Vec3::new(last_x * x_scaling, last_z * z_scaling, y),
                Vec3::new(x * x_scaling, z * z_scaling, y),
                WHITE,
            );
        }

        last_point = Some((x, z));
    }
}

#[allow(dead_code)]
// 2d version. todo refactor with 3d
pub fn draw_line2d_fn<F>(
    gizmos: &mut Gizmos,
    range_start: i32,
    range_end: i32,
    step_size: usize,
    scaling: f32,
    function: F,
) where
    F: Fn(f32) -> f32,
{
    let x_scaling = scaling;
    let z_scaling = scaling;

    let mut last_point = None;

    for x_int in (range_start..range_end).step_by(step_size) {
        let x = x_int as f32;
        let z = function(x);

        if let Some((last_x, last_z)) = last_point {
            gizmos.line_2d(
                Vec2::new(last_x * x_scaling, last_z * z_scaling),
                Vec2::new(x * x_scaling, z * z_scaling),
                WHITE,
            );
        }

        last_point = Some((x, z));
    }
}
