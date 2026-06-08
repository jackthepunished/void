use glam::Vec2;

use super::interpolation::{smooth_damp, Spring};

pub struct CameraController {
    pub position: Vec2,
    pub zoom: f32,
    pub target_position: Vec2,
    pub target_zoom: f32,
    velocity: Vec2,
    zoom_velocity: f32,
    position_spring: Spring,
    zoom_smooth_time: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            target_position: Vec2::ZERO,
            target_zoom: 1.0,
            velocity: Vec2::ZERO,
            zoom_velocity: 0.0,
            position_spring: Spring::smooth(),
            zoom_smooth_time: 0.3,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position = self.position_spring.update(self.position, self.target_position, dt);
        self.zoom = smooth_damp(self.zoom, self.target_zoom, &mut self.zoom_velocity, self.zoom_smooth_time, dt);
        self.zoom = self.zoom.clamp(0.001, 1000.0);
    }

    pub fn focus(&mut self, target: Vec2) {
        self.target_position = target;
    }

    pub fn focus_immediate(&mut self, target: Vec2) {
        self.position = target;
        self.target_position = target;
        self.velocity = Vec2::ZERO;
    }

    pub fn zoom_to(&mut self, zoom: f32) {
        self.target_zoom = zoom.clamp(0.001, 1000.0);
    }

    pub fn zoom_in(&mut self, factor: f32) {
        self.target_zoom = (self.target_zoom * factor).clamp(0.001, 1000.0);
    }

    pub fn zoom_out(&mut self, factor: f32) {
        self.target_zoom = (self.target_zoom / factor).clamp(0.001, 1000.0);
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.target_position += delta / self.zoom;
    }

    pub fn pan_immediate(&mut self, delta: Vec2) {
        let movement = delta / self.zoom;
        self.position += movement;
        self.target_position += movement;
    }

    pub fn is_settled(&self) -> bool {
        let pos_settled = self.position_spring.is_settled(self.position, self.target_position, 0.01);
        let zoom_diff = (self.zoom - self.target_zoom).abs();
        let zoom_vel = self.zoom_velocity.abs();
        pos_settled && zoom_diff < 0.001 && zoom_vel < 0.001
    }

    pub fn snap_to(&mut self, position: Vec2, zoom: f32) {
        self.position = position;
        self.target_position = position;
        self.zoom = zoom;
        self.target_zoom = zoom;
        self.velocity = Vec2::ZERO;
        self.zoom_velocity = 0.0;
    }
}
