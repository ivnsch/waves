use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_grid_2d_system(app: &mut App) {
    app.add_systems(Update, draw_lines);
}

fn draw_lines(mut gizmos: Gizmos) {
    let half_range = 300;
    let step_size = 10;
    let color = Color::DARK_GRAY;

    for line_pos_int in (-half_range..half_range).step_by(step_size) {
        let line_pos = line_pos_int as f32;

        // vertical lines
        gizmos.line_2d(
            Vec2::new(line_pos, -half_range as f32),
            Vec2::new(line_pos, half_range as f32),
            color,
        );

        // horizontal lines
        gizmos.line_2d(
            Vec2::new(-half_range as f32, line_pos),
            Vec2::new(half_range as f32, line_pos),
            color,
        );
    }
}
