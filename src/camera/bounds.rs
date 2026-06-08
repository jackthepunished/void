use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct ViewportBounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl ViewportBounds {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half = size * 0.5;
        Self {
            min: center - half,
            max: center + half,
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn contains_with_margin(&self, point: Vec2, margin: f32) -> bool {
        point.x >= self.min.x - margin
            && point.x <= self.max.x + margin
            && point.y >= self.min.y - margin
            && point.y <= self.max.y + margin
    }

    pub fn intersects(&self, other: &ViewportBounds) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    pub fn expand(&self, amount: f32) -> Self {
        let v = Vec2::splat(amount);
        Self {
            min: self.min - v,
            max: self.max + v,
        }
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn expand_to_include(&self, point: Vec2) -> Self {
        Self {
            min: Vec2::new(self.min.x.min(point.x), self.min.y.min(point.y)),
            max: Vec2::new(self.max.x.max(point.x), self.max.y.max(point.y)),
        }
    }

    pub fn merge(&self, other: &ViewportBounds) -> Self {
        Self {
            min: Vec2::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            max: Vec2::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        }
    }
}
