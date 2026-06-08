use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct CameraTransform {
    pub position: Vec2,
    pub zoom: f32,
    pub rotation: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl Default for CameraTransform {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            viewport_width: 120.0,
            viewport_height: 40.0,
        }
    }
}

impl CameraTransform {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            viewport_width,
            viewport_height,
            ..Default::default()
        }
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let offset = world_pos - self.position;
        let rotated = if self.rotation != 0.0 {
            let cos = self.rotation.cos();
            let sin = self.rotation.sin();
            Vec2::new(
                offset.x * cos - offset.y * sin,
                offset.x * sin + offset.y * cos,
            )
        } else {
            offset
        };
        let scaled = rotated * self.zoom;
        Vec2::new(
            scaled.x + self.viewport_width * 0.5,
            scaled.y + self.viewport_height * 0.5,
        )
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let centered = screen_pos - Vec2::new(self.viewport_width * 0.5, self.viewport_height * 0.5);
        let unscaled = if self.zoom != 0.0 {
            centered / self.zoom
        } else {
            centered
        };
        let rotated = if self.rotation != 0.0 {
            let cos = self.rotation.cos();
            let sin = self.rotation.sin();
            Vec2::new(
                unscaled.x * cos + unscaled.y * sin,
                -unscaled.x * sin + unscaled.y * cos,
            )
        } else {
            unscaled
        };
        rotated + self.position
    }

    pub fn visible_world_rect(&self) -> (Vec2, Vec2) {
        let half_width = self.viewport_width * 0.5 / self.zoom;
        let half_height = self.viewport_height * 0.5 / self.zoom;
        let min = self.position - Vec2::new(half_width, half_height);
        let max = self.position + Vec2::new(half_width, half_height);
        (min, max)
    }

    pub fn contains_world_pos(&self, pos: Vec2, margin: f32) -> bool {
        let (min, max) = self.visible_world_rect();
        pos.x >= min.x - margin
            && pos.x <= max.x + margin
            && pos.y >= min.y - margin
            && pos.y <= max.y + margin
    }

    pub fn focus(&mut self, target: Vec2) {
        self.position = target;
    }

    pub fn zoom_to(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.001, 1000.0);
    }

    pub fn move_by(&mut self, delta: Vec2) {
        self.position += delta / self.zoom;
    }
}
