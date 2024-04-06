use std::time::Duration;
use bevy::input::ButtonInput;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Camera, Commands, EventReader, KeyCode, MouseButton, Query, Res, ResMut, Resource, Time, Timer, TimerMode, Transform, With};
use crate::player::Player;
#[derive(Resource)]
pub(crate) struct MousePressed(pub bool);
pub(crate) fn handle_mouse(
    mut button_events: EventReader<MouseButtonInput>,
    mut motion_events: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut mouse_pressed: ResMut<MousePressed>,
) {
    for button_event in button_events.read() {
        if button_event.button != MouseButton::Left {
            continue;
        }
        *mouse_pressed = MousePressed(button_event.state.is_pressed());
    }
    if !mouse_pressed.0 {
        return;
    }
    let displacement = motion_events
        .read()
        .fold(0., |acc, mouse_motion| acc + mouse_motion.delta.x);
    let mut camera_transform = camera.single_mut();
    camera_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-displacement / 75.));
}
pub(crate) fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Player), With<Camera>>,
    mut commands: Commands,
) {
    let delta_time = time.delta_seconds();
    let mouse_sensitivity: f32 = 3.2;
    let arrow_key_sensitivity: f32 = 4.5;
    let acceleration = 05.0;
    let friction= 04.0;

    let mut delta_rotation = Vec2::ZERO;

    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        pitch += arrow_key_sensitivity;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        pitch -= arrow_key_sensitivity;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        yaw -= arrow_key_sensitivity;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        yaw += arrow_key_sensitivity;
    }

    pitch = pitch.clamp(-89.9, 89.9);


    for (mut transform, mut player) in query.iter_mut() {
        if player.float_timer.tick(time.delta()).just_finished() {
            player.can_float = false;
        }
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize() * player.speed;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize() * player.speed;
        }

        let velocity_change = direction - player.velocity;
        let acceleration_effect = velocity_change.clamp_length_max(acceleration * delta_time);
        let forward = transform.forward();
        let right = transform.right();
        player.velocity += (forward * direction.z + right * direction.x) * acceleration * delta_time;
        player.velocity = player.velocity.lerp(Vec3::ZERO, friction * delta_time);


        if direction == Vec3::ZERO || velocity_change.length() > player.speed * 0.9 {
            player.velocity = player.velocity.lerp(Vec3::ZERO, friction * delta_time);
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_on_ground {
                player.velocity.y = player.jump_force;
                player.is_on_ground = false;
                player.float_timer.reset();
            } else if player.can_float {
                if keyboard_input.pressed(KeyCode::Space) {
                    player.velocity.y += 0.5;
                }
            } else {
                player.can_float = true;
                player.float_timer = Timer::new(Duration::from_millis(200), TimerMode::default());
            }
        }

        if !player.is_flying {
            player.velocity.y += player.gravity * delta_time;
        }

        transform.translation += player.velocity * delta_time;

        if transform.translation.y <= 0.0 {
            player.is_on_ground = true;
            player.velocity.y = 0.0;
            transform.translation.y = 0.0;
        } else {
            player.is_on_ground = false;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize() * player.speed;
        }

        let forward = transform.forward();
        let right = transform.right();
        transform.translation += (forward * direction.z + right * direction.x) * delta_time;

        let rotation = Quat::from_axis_angle(Vec3::Y, yaw.to_radians())
            * Quat::from_axis_angle(Vec3::X, pitch.to_radians());
        transform.rotation = rotation;

    }
}
