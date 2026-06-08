use glam::Vec2;

pub struct Constraint {
    pub entity_a: uuid::Uuid,
    pub entity_b: uuid::Uuid,
    pub min_distance: f32,
    pub max_distance: f32,
    pub strength: f32,
}

impl Constraint {
    pub fn new(entity_a: uuid::Uuid, entity_b: uuid::Uuid, min_distance: f32, max_distance: f32) -> Self {
        Self {
            entity_a,
            entity_b,
            min_distance,
            max_distance,
            strength: 1.0,
        }
    }

    pub fn spring(entity_a: uuid::Uuid, entity_b: uuid::Uuid, ideal_distance: f32, strength: f32) -> Self {
        Self {
            entity_a,
            entity_b,
            min_distance: ideal_distance * 0.5,
            max_distance: ideal_distance * 2.0,
            strength,
        }
    }
}

pub struct ConstraintSolver {
    iterations: usize,
}

impl ConstraintSolver {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    pub fn solve(
        &self,
        constraints: &[Constraint],
        positions: &mut std::collections::HashMap<uuid::Uuid, Vec2>,
    ) {
        for _ in 0..self.iterations {
            for constraint in constraints {
                let pos_a = positions.get(&constraint.entity_a).copied();
                let pos_b = positions.get(&constraint.entity_b).copied();

                if let (Some(pos_a), Some(pos_b)) = (pos_a, pos_b) {
                    let diff = pos_b - pos_a;
                    let dist = diff.length();
                    let dir = if dist > 0.001 { diff / dist } else { Vec2::X };

                    if dist < constraint.min_distance {
                        let correction = (constraint.min_distance - dist) * 0.5 * constraint.strength;
                        if let Some(p) = positions.get_mut(&constraint.entity_a) {
                            *p -= dir * correction;
                        }
                        if let Some(p) = positions.get_mut(&constraint.entity_b) {
                            *p += dir * correction;
                        }
                    } else if dist > constraint.max_distance {
                        let correction = (dist - constraint.max_distance) * 0.5 * constraint.strength;
                        if let Some(p) = positions.get_mut(&constraint.entity_a) {
                            *p += dir * correction;
                        }
                        if let Some(p) = positions.get_mut(&constraint.entity_b) {
                            *p -= dir * correction;
                        }
                    }
                }
            }
        }
    }
}
