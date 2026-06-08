use glam::Vec2;

pub struct CameraState {
    pub position: Vec2,
    pub zoom: f32,
    pub mouse_screen: Vec2,
    pub viewport_size: Vec2,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            mouse_screen: Vec2::ZERO,
            viewport_size: Vec2::new(120.0, 40.0),
        }
    }
}

impl CameraState {
    pub fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        let half_viewport = self.viewport_size * 0.5;
        let scaled = (screen - half_viewport) / self.zoom;
        scaled + self.position
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let offset = world_pos - self.position;
        let scaled = offset * self.zoom;
        let half_viewport = self.viewport_size * 0.5;
        scaled + half_viewport
    }

    pub fn visible_bounds(&self) -> (Vec2, Vec2) {
        let half = self.viewport_size * 0.5 / self.zoom;
        let min = self.position - half;
        let max = self.position + half;
        (min, max)
    }

    pub fn is_visible(&self, pos: Vec2, margin: f32) -> bool {
        let (min, max) = self.visible_bounds();
        pos.x >= min.x - margin
            && pos.x <= max.x + margin
            && pos.y >= min.y - margin
            && pos.y <= max.y + margin
    }
}

pub struct TimeState {
    pub elapsed: f32,
    pub delta: f32,
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            delta: 0.0,
        }
    }
}
