use std::f32::consts::PI;

use crate::camera_controller::{CameraController, CameraControllerPlugin};
use crate::rotator::{Rotator, RotatorPlugin};
use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};

#[allow(dead_code)]
pub fn add_3d_space(app: &mut App) {
    app.add_plugins((DefaultPlugins, CameraControllerPlugin, RotatorPlugin))
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_light,
                setup_x_axis_label,
                setup_y_axis_label,
                setup_z_axis_label,
            ),
        )
        .add_systems(Update, (setup_axes, setup_global_axes));
}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

fn setup_global_axes(mut gizmos: Gizmos) {
    let size = 2.0;
    let zero = 0.0;
    // x
    gizmos.line(
        Vec3 {
            x: -size,
            y: zero,
            z: zero,
        },
        Vec3 {
            x: size,
            y: zero,
            z: zero,
        },
        Color::GREEN,
    );
    // y
    gizmos.line(
        Vec3 {
            x: zero,
            y: -size,
            z: zero,
        },
        Vec3 {
            x: zero,
            y: size,
            z: zero,
        },
        Color::RED,
    );
    // z
    gizmos.line(
        Vec3 {
            x: zero,
            y: zero,
            z: -size,
        },
        Vec3 {
            x: zero,
            y: zero,
            z: size,
        },
        Color::BLUE,
    );
}

fn setup_axes(mut gizmos: Gizmos) {
    let size = 2.0;
    let zero = 0.0;
    // x
    gizmos.line(
        Vec3 {
            x: -size,
            y: zero,
            z: zero,
        },
        Vec3 {
            x: size,
            y: zero,
            z: zero,
        },
        Color::GREEN,
    );
    // y
    gizmos.line(
        Vec3 {
            x: zero,
            y: -size,
            z: zero,
        },
        Vec3 {
            x: zero,
            y: size,
            z: zero,
        },
        Color::RED,
    );
    // z
    gizmos.line(
        Vec3 {
            x: zero,
            y: zero,
            z: -size,
        },
        Vec3 {
            x: zero,
            y: zero,
            z: size,
        },
        Color::BLUE,
    );
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
        Rotator::default(),
    ));
}

#[derive(Component)]
struct Cube;

fn generate_axis_label(
    label: &str,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) -> CubeWithMaterial {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // Light
    commands.spawn(DirectionalLightBundle::default());
    // commands.spawn(DirectionalLightBundle {
    //     transform: Transform {
    //         translation: Vec3 {
    //             x: -10.0,
    //             y: 0.0,
    //             z: 0.0,
    //         },
    //         rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
    //         scale: Vec3::ONE,
    //     },
    //     directional_light: DirectionalLight {
    //         color: Color::rgb(1.0, 1.0, 1.0),
    //         ..DirectionalLight::default()
    //     },
    //     ..DirectionalLightBundle::default()
    // });

    let texture_camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // Cover the whole image
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            TargetCamera(texture_camera),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 320.0,
                    color: Color::BLACK,
                    ..default()
                },
            ));
        });

    let cube_size = 0.05;
    let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,

        ..default()
    });

    CubeWithMaterial {
        cube: cube_handle,
        material: material_handle,
    }
}

fn setup_x_axis_label(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    let cube = generate_axis_label("x", &mut commands, meshes, materials, images);
    commands.spawn((
        to_pbr_bundle(
            cube,
            Transform::from_xyz(2.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(-PI)),
        ),
        Cube,
    ));
}

fn setup_y_axis_label(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    let cube = generate_axis_label("y", &mut commands, meshes, materials, images);
    commands.spawn((
        to_pbr_bundle(
            cube,
            Transform::from_xyz(0.0, 0.0, 2.0).with_rotation(Quat::from_rotation_x(PI / 2.0)),
        ),
        Cube,
    ));
}

fn setup_z_axis_label(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    let cube = generate_axis_label("z", &mut commands, meshes, materials, images);
    commands.spawn((
        to_pbr_bundle(
            cube,
            Transform::from_xyz(0.0, 2.0, 0.0).with_rotation(Quat::from_rotation_x(-PI)),
        ),
        Cube,
    ));
}

fn to_pbr_bundle(cube: CubeWithMaterial, transform: Transform) -> PbrBundle {
    PbrBundle {
        mesh: cube.cube,
        material: cube.material,
        transform,
        ..default()
    }
}

struct CubeWithMaterial {
    cube: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}
