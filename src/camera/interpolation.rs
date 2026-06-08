use glam::Vec2;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

pub fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t))
}

pub fn ease_in_quad(t: f32) -> f32 {
    t * t
}

pub fn ease_out_quad(t: f32) -> f32 {
    t * (2.0 - t)
}

pub fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
}

pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t = t - 1.0;
        4.0 * t * t * t + 1.0
    }
}

pub fn ease_out_elastic(t: f32) -> f32 {
    let p = 0.3;
    let s = p / 4.0;
    let t = t - 1.0;
    -(pow2(10.0 * t) * sin(((t - s) * std::f32::consts::TAU) / p))
}

pub fn ease_out_back(t: f32) -> f32 {
    let s = 1.70158;
    let t = t - 1.0;
    t * t * ((s + 1.0) * t + s) + 1.0
}

pub fn ease_in_out_back(t: f32) -> f32 {
    let s = 1.70158 * 1.525;
    if t < 0.5 {
        let t = t * 2.0;
        0.5 * (t * t * ((s + 1.0) * t - s))
    } else {
        let t = t * 2.0 - 2.0;
        0.5 * (t * t * ((s + 1.0) * t + s) + 2.0)
    }
}

pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub velocity: Vec2,
}

impl Spring {
    pub fn new(stiffness: f32, damping: f32) -> Self {
        Self {
            stiffness,
            damping,
            velocity: Vec2::ZERO,
        }
    }

    pub fn smooth() -> Self {
        Self::new(120.0, 14.0)
    }

    pub fn snappy() -> Self {
        Self::new(300.0, 20.0)
    }

    pub fn bouncy() -> Self {
        Self::new(180.0, 10.0)
    }

    pub fn update(&mut self, current: Vec2, target: Vec2, dt: f32) -> Vec2 {
        let displacement = current - target;
        let spring_force = -self.stiffness * displacement;
        let damping_force = -self.damping * self.velocity;
        let acceleration = spring_force + damping_force;
        self.velocity += acceleration * dt;
        current + self.velocity * dt
    }

    pub fn is_settled(&self, current: Vec2, target: Vec2, threshold: f32) -> bool {
        let displacement = current - target;
        displacement.length_squared() < threshold * threshold
            && self.velocity.length_squared() < threshold * threshold
    }
}

pub fn smooth_damp(
    current: f32,
    target: f32,
    velocity: &mut f32,
    smooth_time: f32,
    dt: f32,
) -> f32 {
    let omega = 2.0 / smooth_time.max(0.0001);
    let x = omega * dt;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = current - target;
    let temp = (*velocity + omega * change) * dt;
    *velocity = (*velocity - omega * temp) * exp;
    target + (change + temp) * exp
}

fn pow2(x: f32) -> f32 {
    x * x
}

fn sin(x: f32) -> f32 {
    x.sin()
}
