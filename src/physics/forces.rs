use glam::Vec2;

pub struct PhysicsConfig {
    pub repulsion_strength: f32,
    pub attraction_strength: f32,
    pub ideal_distance: f32,
    pub damping: f32,
    pub gravity: f32,
    pub center_gravity: Vec2,
    pub max_velocity: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            repulsion_strength: 50.0,
            attraction_strength: 0.01,
            ideal_distance: 5.0,
            damping: 0.9,
            gravity: 0.01,
            center_gravity: Vec2::ZERO,
            max_velocity: 5.0,
        }
    }
}

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

pub struct PhysicsEngine {
    config: PhysicsConfig,
    spatial_hash: SpatialHash,
}

impl PhysicsEngine {
    pub fn new(config: PhysicsConfig) -> Self {
        Self {
            config,
            spatial_hash: SpatialHash::new(10.0),
        }
    }

    pub fn compute_forces(
        &mut self,
        positions: &mut std::collections::HashMap<uuid::Uuid, Vec2>,
        velocities: &mut std::collections::HashMap<uuid::Uuid, Vec2>,
        dt: f32,
    ) {
        self.spatial_hash.clear();
        for (&id, &pos) in positions.iter() {
            self.spatial_hash.insert(id, pos);
        }

        let ids: Vec<uuid::Uuid> = positions.keys().copied().collect();
        let mut forces: std::collections::HashMap<uuid::Uuid, Vec2> = std::collections::HashMap::new();

        for &id in &ids {
            forces.entry(id).or_insert(Vec2::ZERO);
        }

        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                let a = ids[i];
                let b = ids[j];
                let pos_a = positions[&a];
                let pos_b = positions[&b];

                let diff = pos_a - pos_b;
                let dist = diff.length().max(0.01);
                let dir = diff / dist;

                let repulsion = self.config.repulsion_strength / (dist * dist);
                let repulsion_force = dir * repulsion;

                *forces.entry(a).or_default() += repulsion_force;
                *forces.entry(b).or_default() -= repulsion_force;
            }
        }

        for &id in &ids {
            let pos = positions[&id];
            let to_center = self.config.center_gravity - pos;
            *forces.entry(id).or_default() += to_center * self.config.gravity;
        }

        for &id in &ids {
            let force = forces[&id];
            let vel = velocities.entry(id).or_insert(Vec2::ZERO);
            *vel = (*vel + force * dt) * self.config.damping;
            let speed = vel.length();
            if speed > self.config.max_velocity {
                *vel = *vel / speed * self.config.max_velocity;
            }
        }
    }

    pub fn apply_velocities(
        &self,
        positions: &mut std::collections::HashMap<uuid::Uuid, Vec2>,
        velocities: &std::collections::HashMap<uuid::Uuid, Vec2>,
    ) {
        for (id, vel) in velocities {
            if let Some(pos) = positions.get_mut(id) {
                *pos += *vel;
            }
        }
    }
}
