use glam::Vec2;

pub struct SpatialHash {
    cell_size: f32,
    cells: std::collections::HashMap<(i32, i32), Vec<uuid::Uuid>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: std::collections::HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn insert(&mut self, id: uuid::Uuid, position: Vec2) {
        let cell = self.position_to_cell(position);
        self.cells.entry(cell).or_default().push(id);
    }

    pub fn query_radius(&self, position: Vec2, radius: f32) -> Vec<uuid::Uuid> {
        let mut results = Vec::new();
        let min_cell = self.position_to_cell(position - Vec2::splat(radius));
        let max_cell = self.position_to_cell(position + Vec2::splat(radius));

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(ids) = self.cells.get(&(x, y)) {
                    results.extend(ids.iter().copied());
                }
            }
        }

        results
    }

    fn position_to_cell(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }
}
