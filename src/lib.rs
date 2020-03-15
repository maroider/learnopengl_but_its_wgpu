use std::fmt::Debug;

use amethyst_input::{Button, InputEvent, InputHandler, ScrollDirection, StringBindings};
use shrev::{EventChannel, ReaderId};
use ultraviolet::{Mat4, Rotor3, Vec3};

pub struct Camera {
    pub translation: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
    pub is_controlled: bool,

    pub event_reader: ReaderId<InputEvent<StringBindings>>,
}

impl Camera {
    pub fn get_view_projection_matrix(&self, width: f32, height: f32, near: f32, far: f32) -> Mat4 {
        ultraviolet::projection::perspective_vk(self.zoom, width / height, near, far)
            * (Rotor3::from_rotation_xy(self.roll)
                * Rotor3::from_rotation_yz(self.pitch)
                * Rotor3::from_rotation_xz(self.yaw))
            .into_matrix()
            .into_homogeneous()
            * Mat4::from_translation(self.translation)
    }

    pub fn get_direction_vector(&self) -> Vec3 {
        Vec3 {
            x: -self.yaw.sin() * self.pitch.cos(),
            y: -self.pitch.sin(),
            z: -self.yaw.cos() * self.pitch.cos(),
        }
        .normalized()
    }

    pub fn on_event(&mut self, event_channel: &EventChannel<InputEvent<StringBindings>>) {
        if !self.is_controlled {
            return;
        }

        for event in event_channel.read(&mut self.event_reader) {
            match event {
                InputEvent::MouseMoved { delta_x, delta_y } => {
                    self.pitch += delta_y * self.mouse_sensitivity;
                    self.yaw += -(delta_x * self.mouse_sensitivity);

                    if self.pitch > std::f32::consts::FRAC_PI_2 {
                        self.pitch = std::f32::consts::FRAC_PI_2;
                    } else if self.pitch < -std::f32::consts::FRAC_PI_2 {
                        self.pitch = -std::f32::consts::FRAC_PI_2;
                    }
                    if self.yaw > 2.0 * std::f32::consts::PI {
                        self.yaw -= 2.0 * std::f32::consts::PI;
                    } else if self.yaw < 0.0 {
                        self.yaw = 2.0 * std::f32::consts::PI - self.yaw;
                    }
                }
                InputEvent::MouseWheelMoved(direction) => match direction {
                    ScrollDirection::ScrollUp => {
                        if self.zoom > 0.2 {
                            self.zoom -= 0.1;
                        } else {
                            self.zoom = 0.1;
                        }
                    }
                    ScrollDirection::ScrollDown => {
                        if self.zoom < std::f32::consts::PI - 0.1 {
                            self.zoom += 0.1;
                        } else {
                            self.zoom = std::f32::consts::PI - 0.1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn update(&mut self, input_handler: &InputHandler<StringBindings>) {
        let forward_displacement = (if input_handler.button_is_down(Button::ScanCode(0x11)) {
            -1.0
        } else {
            0.0
        } + if input_handler.button_is_down(Button::ScanCode(0x1F)) {
            1.0
        } else {
            0.0
        }) * self.movement_speed;
        let sideways_displacement = (if input_handler.button_is_down(Button::ScanCode(0x1E)) {
            -1.0
        } else {
            0.0
        } + if input_handler.button_is_down(Button::ScanCode(0x20)) {
            1.0
        } else {
            0.0
        }) * self.movement_speed;
        let vertical_displacement = (if input_handler.button_is_down(Button::ScanCode(0x39)) {
            -1.0
        } else {
            0.0
        } + if input_handler.button_is_down(Button::ScanCode(0x2e)) {
            1.0
        } else {
            0.0
        }) * self.movement_speed;

        let yaw = self.yaw - std::f32::consts::FRAC_PI_2;

        let x_displacement = -yaw.cos() * forward_displacement + yaw.sin() * sideways_displacement;
        let y_displacement = -self.pitch.sin() * forward_displacement + vertical_displacement;
        let z_displacement = yaw.sin() * forward_displacement + yaw.cos() * sideways_displacement;

        self.translation += Vec3::new(x_displacement, y_displacement, z_displacement);
    }
}

/// Ensures that the amount of memory (in bytes) consumed by the wrapped type `T` will always be a multiple of 16.
///
/// ```
/// # use std::mem;
/// #
/// # use learnopengl_but_its_wgpu::Align16;
/// #
/// assert_eq!(4, mem::size_of::<u32>()));
/// assert_eq!(16, mem::size_of::<Align16<u32>>()));
/// assert_eq!(16, mem::size_of::<u128>()));
/// assert_eq!(16, mem::size_of::<Align16<u128>>()));
/// assert_eq!(20, mem::size_of::<[u32; 5]>()));
/// assert_eq!(32, mem::size_of::<Align16<[u32; 5]>>()));
/// ```
#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Align16<T: Clone + Copy + Debug>(pub T);

impl<T> From<T> for Align16<T>
where
    T: Clone + Copy + Debug,
{
    fn from(from: T) -> Self {
        Self(from)
    }
}
