use bevy::prelude::*;
use std::fmt;

use crate::scratchpad_3d::MySphere;

pub struct RotatorPlugin;

impl Plugin for RotatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (run_sphere_rotator, run_rotator));
    }
}

#[derive(Component, Debug)]
pub struct Rotator {
    pub initialized: bool,
    pub key_y: KeyCode,
    pub key_z: KeyCode,
    pub key_x: KeyCode,
    pub key_i: KeyCode,
    pub key_o: KeyCode,
    pub key_p: KeyCode,
    pub key_shift_left: KeyCode,
    pub key_shift_right: KeyCode,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for Rotator {
    fn default() -> Self {
        Self {
            initialized: false,
            key_y: KeyCode::KeyY,
            key_z: KeyCode::KeyZ,
            key_x: KeyCode::KeyX,
            key_i: KeyCode::KeyI,
            key_o: KeyCode::KeyO,
            key_p: KeyCode::KeyP,
            key_shift_left: KeyCode::ShiftLeft,
            key_shift_right: KeyCode::ShiftRight,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl fmt::Display for Rotator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
Rotator Controls:
    {:?} \t- rotate around x
    {:?} \t- rotate around y
    {:?} \t- rotate around z",
            self.key_x, self.key_y, self.key_z,
        )
    }
}

fn run_sphere_rotator(
    key_input: Res<ButtonInput<KeyCode>>,
    mut sphere: Query<&mut Transform, With<MySphere>>,
    mut rotator: Query<&mut Rotator, With<Camera>>,
) {
    if let Ok(mut transform) = sphere.get_single_mut() {
        let q = rotator.get_single_mut();
        if let Ok(mut controller) = q {
            if !controller.initialized {
                let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
                controller.yaw = yaw;
                controller.pitch = pitch;
                controller.initialized = true;
                info!("{}", *controller);
            }

            let mut rotation = 0.03;
            if key_input.pressed(controller.key_shift_left)
                || key_input.pressed(controller.key_shift_right)
            {
                rotation = -rotation;
            }

            // Handle key input
            if key_input.pressed(controller.key_y) {
                transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation),
                );
            }
            if key_input.pressed(controller.key_z) {
                transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
                );
            }
            if key_input.pressed(controller.key_x) {
                transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_euler(EulerRot::XYZ, rotation, 0.0, 0.0),
                );
            }
        }
    };
}

#[allow(clippy::too_many_arguments)]
fn run_rotator(
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Rotator), With<Camera>>,
) {
    let q = query.get_single_mut();
    // println!("q: {:?}", q);
    if let Ok((mut transform, mut controller)) = q {
        if !controller.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.yaw = yaw;
            controller.pitch = pitch;
            controller.initialized = true;
            info!("{}", *controller);
        }

        // for p in key_input.get_pressed() {
        //     println!("p: {:?}", p);
        // }

        let mut rotation = 0.03;
        if key_input.pressed(controller.key_shift_left)
            || key_input.pressed(controller.key_shift_right)
        {
            rotation = -rotation;
        }

        // Handle key input
        if key_input.pressed(controller.key_i) {
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
            );
        }
        if key_input.pressed(controller.key_o) {
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation),
            );
        }
        if key_input.pressed(controller.key_p) {
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_euler(EulerRot::XYZ, rotation, 0.0, 0.0),
            );
        }
    }
}
