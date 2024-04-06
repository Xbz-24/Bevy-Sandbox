use bevy::math::Vec3;
use bevy::prelude::{Component, Timer, TimerMode};

#[derive(Component)]
pub(crate) struct Player {
    pub(crate) speed: f32,
    pub(crate) is_flying: bool,
    pub(crate) is_on_ground: bool,
    pub(crate) jump_force: f32,
    pub(crate) gravity: f32,
    pub(crate) velocity: Vec3,
    pub(crate) float_timer: Timer,
    pub(crate) can_float: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 3.0,
            is_flying: false,
            is_on_ground: true,
            can_float: false,
            float_timer: Timer::from_seconds(1.0, TimerMode::default()),
            jump_force: 13.0,
            gravity: -9.81,
            velocity: Vec3::ZERO,
        }
    }
}
