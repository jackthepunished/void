use glam::Vec2;

use crate::camera::interpolation::{ease_in_out_cubic, ease_out_back, ease_out_elastic, lerp, lerp_vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseOutBack,
    EaseOutElastic,
    Spring,
}

impl Default for Easing {
    fn default() -> Self {
        Self::EaseInOutCubic
    }
}

pub struct Tween<T> {
    pub from: T,
    pub to: T,
    pub duration: f32,
    pub elapsed: f32,
    pub easing: Easing,
}

impl Tween<Vec2> {
    pub fn new(from: Vec2, to: Vec2, duration: f32, easing: Easing) -> Self {
        Self {
            from,
            to,
            duration,
            elapsed: 0.0,
            easing,
        }
    }

    pub fn update(&mut self, dt: f32) -> Vec2 {
        self.elapsed += dt;
        let t = (self.elapsed / self.duration).clamp(0.0, 1.0);
        let eased = Self::ease_value(t, self.easing);
        lerp_vec2(self.from, self.to, eased)
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }

    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).clamp(0.0, 1.0)
    }

    fn ease_value(t: f32, easing: Easing) -> f32 {
        match easing {
            Easing::Linear => t,
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => t * (2.0 - t),
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => {
                let t = t - 1.0;
                t * t * t + 1.0
            }
            Easing::EaseInOutCubic => ease_in_out_cubic(t),
            Easing::EaseOutBack => ease_out_back(t),
            Easing::EaseOutElastic => ease_out_elastic(t),
            Easing::Spring => {
                let p = 0.3;
                let s = p / 4.0;
                let t = t - 1.0;
                -(2f32.powf(10.0 * t) * ((t - s) * std::f32::consts::TAU / p).sin())
            }
        }
    }
}

pub struct TweenF32 {
    pub from: f32,
    pub to: f32,
    pub duration: f32,
    pub elapsed: f32,
    pub easing: Easing,
}

impl TweenF32 {
    pub fn new(from: f32, to: f32, duration: f32, easing: Easing) -> Self {
        Self {
            from,
            to,
            duration,
            elapsed: 0.0,
            easing,
        }
    }

    pub fn update(&mut self, dt: f32) -> f32 {
        self.elapsed += dt;
        let t = (self.elapsed / self.duration).clamp(0.0, 1.0);
        let eased = Self::ease_value(t, self.easing);
        lerp(self.from, self.to, eased)
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }

    fn ease_value(t: f32, easing: Easing) -> f32 {
        match easing {
            Easing::Linear => t,
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => t * (2.0 - t),
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => {
                let t = t - 1.0;
                t * t * t + 1.0
            }
            Easing::EaseInOutCubic => ease_in_out_cubic(t),
            Easing::EaseOutBack => ease_out_back(t),
            Easing::EaseOutElastic => ease_out_elastic(t),
            Easing::Spring => {
                let p = 0.3;
                let s = p / 4.0;
                let t = t - 1.0;
                -(2f32.powf(10.0 * t) * ((t - s) * std::f32::consts::TAU / p).sin())
            }
        }
    }
}
