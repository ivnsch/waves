use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_curves_2d_system(app: &mut App) {
    // app.add_systems(Update, draw_square_fn);
    app.add_systems(Update, draw_sin_fn);
}

#[allow(dead_code)]
fn draw_square_fn(gizmos: Gizmos) {
    draw_fn(gizmos, -10, 10, |x| x * x);
}

fn draw_sin_fn(gizmos: Gizmos, time: Res<Time>) {
    // println!("drawsign t: {}", t);
    let t = time.elapsed_seconds();
    // draw_fn(gizmos, -10, 10, |x| x.sin());
    // animate movement
    draw_fn(gizmos, -10 + t as i32, 10 + t as i32, |x| x.sin());
}

fn draw_fn(mut gizmos: Gizmos, range_start: i32, range_end: i32, function: fn(f32) -> f32) {
    let scaling = 20.0;
    let x_scaling = scaling;
    let y_scaling = scaling;

    let mut last_point = None;

    for i in range_start..range_end {
        let x = i as f32;
        let y = function(x);

        if let Some((last_x, last_y)) = last_point {
            gizmos.line_2d(
                Vec2::new(last_x * x_scaling, last_y * y_scaling),
                Vec2::new(x * x_scaling, y * y_scaling),
                Color::WHITE,
            );
        }

        last_point = Some((x, y));
    }
}
