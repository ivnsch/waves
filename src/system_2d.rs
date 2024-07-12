use bevy::prelude::*;

pub fn add_2d_space(app: &mut App) {
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_light))
        .add_systems(Update, setup_axes);
}

pub fn add_2d_axes(app: &mut App) {
    app.add_systems(Update, setup_axes);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

fn setup_axes(mut gizmos: Gizmos) {
    let size = 300.0;
    let zero = 0.0;
    // x
    gizmos.line_2d(
        Vec2 { x: -size, y: zero },
        Vec2 { x: size, y: zero },
        Color::GREEN,
    );
    // y
    gizmos.line_2d(
        Vec2 { x: zero, y: -size },
        Vec2 { x: zero, y: size },
        Color::RED,
    );
}
