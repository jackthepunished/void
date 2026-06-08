use std::collections::HashMap;

use glam::Vec2;
use petgraph::stable_graph::NodeIndex;
use uuid::Uuid;

use crate::physics::SpatialHash;
use super::types::WorldGraph;

pub struct LayoutConfig {
    pub repulsion_strength: f32,
    pub attraction_strength: f32,
    pub ideal_distance: f32,
    pub damping: f32,
    pub gravity: f32,
    pub center_gravity: Vec2,
    pub max_velocity: f32,
    pub max_iterations: usize,
    pub convergence_threshold: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            repulsion_strength: 50.0,
            attraction_strength: 0.01,
            ideal_distance: 5.0,
            damping: 0.9,
            gravity: 0.01,
            center_gravity: Vec2::ZERO,
            max_velocity: 5.0,
            max_iterations: 100,
            convergence_threshold: 0.001,
        }
    }
}

pub struct ForceDirectedLayout {
    config: LayoutConfig,
    velocities: HashMap<NodeIndex, Vec2>,
    spatial_hash: SpatialHash,
}

impl ForceDirectedLayout {
    pub fn new(config: LayoutConfig) -> Self {
        Self {
            config,
            velocities: HashMap::new(),
            spatial_hash: SpatialHash::new(10.0),
        }
    }

    pub fn step(&mut self, graph: &mut WorldGraph) -> f32 {
        let node_indices: Vec<NodeIndex> = graph.graph.node_indices().collect();
        if node_indices.is_empty() {
            return 0.0;
        }

        let mut forces: HashMap<NodeIndex, Vec2> = HashMap::new();
        let mut idx_to_id: HashMap<NodeIndex, Uuid> = HashMap::new();
        let mut id_to_idx: HashMap<Uuid, NodeIndex> = HashMap::new();
        let mut positions: HashMap<Uuid, Vec2> = HashMap::new();

        // Build spatial hash and index maps
        self.spatial_hash.clear();
        for &idx in &node_indices {
            forces.insert(idx, Vec2::ZERO);
            if let Some(node) = graph.get_node(idx) {
                idx_to_id.insert(idx, node.id);
                id_to_idx.insert(node.id, idx);
                positions.insert(node.id, node.position);
                self.spatial_hash.insert(node.id, node.position);
            }
        }

        // Repulsion via spatial hash neighbor queries
        let repulsion_radius = self.config.repulsion_strength.sqrt() * 2.0;
        for &idx in &node_indices {
            let Some(&id) = idx_to_id.get(&idx) else { continue };
            let Some(&pos_a) = positions.get(&id) else { continue };

            let neighbors = self.spatial_hash.query_radius(pos_a, repulsion_radius);
            for neighbor_id in neighbors {
                if neighbor_id <= id { continue; } // avoid double-counting
                let Some(&neighbor_idx) = id_to_idx.get(&neighbor_id) else { continue };
                let Some(&pos_b) = positions.get(&neighbor_id) else { continue };

                let diff = pos_a - pos_b;
                let dist = diff.length().max(0.01);
                let dir = diff / dist;

                let repulsion = self.config.repulsion_strength / (dist * dist);
                let repulsion_force = dir * repulsion;

                *forces.entry(idx).or_default() += repulsion_force;
                *forces.entry(neighbor_idx).or_default() -= repulsion_force;
            }
        }

        // Spring attraction along edges
        for edge_ref in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge_ref) {
                let pos_a = graph.get_node(source).map(|n| n.position).unwrap_or(Vec2::ZERO);
                let pos_b = graph.get_node(target).map(|n| n.position).unwrap_or(Vec2::ZERO);

                let diff = pos_b - pos_a;
                let dist = diff.length();
                let dir = if dist > 0.001 { diff / dist } else { Vec2::X };

                let displacement = dist - self.config.ideal_distance;
                let attraction = self.config.attraction_strength * displacement;
                let attraction_force = dir * attraction;

                *forces.entry(source).or_default() += attraction_force;
                *forces.entry(target).or_default() -= attraction_force;
            }
        }

        // Center gravity
        for &idx in &node_indices {
            let pos = graph.get_node(idx).map(|n| n.position).unwrap_or(Vec2::ZERO);
            let to_center = self.config.center_gravity - pos;
            *forces.entry(idx).or_default() += to_center * self.config.gravity;
        }

        // Integrate velocities and apply
        let mut total_movement = 0.0;
        for &idx in &node_indices {
            let force = forces.get(&idx).copied().unwrap_or(Vec2::ZERO);
            let velocity = self.velocities.entry(idx).or_insert(Vec2::ZERO);

            *velocity = (*velocity + force) * self.config.damping;
            let speed = velocity.length();
            if speed > self.config.max_velocity {
                *velocity = *velocity / speed * self.config.max_velocity;
            }

            total_movement += velocity.length();

            if let Some(node) = graph.get_node_mut(idx) {
                node.position += *velocity;
            }
        }

        total_movement / node_indices.len() as f32
    }

    pub fn layout(&mut self, graph: &mut WorldGraph) {
        for iteration in 0..self.config.max_iterations {
            let avg_movement = self.step(graph);
            if avg_movement < self.config.convergence_threshold {
                tracing::debug!("Layout converged after {} iterations", iteration + 1);
                break;
            }
        }
    }

    pub fn reset(&mut self) {
        self.velocities.clear();
        self.spatial_hash.clear();
    }
}

pub fn hierarchical_layout(graph: &WorldGraph, root: NodeIndex, horizontal_spacing: f32, vertical_spacing: f32) -> HashMap<NodeIndex, Vec2> {
    let mut positions = HashMap::new();
    let mut levels: HashMap<u32, Vec<NodeIndex>> = HashMap::new();

    let bfs = graph.bfs_from(root, u32::MAX);
    for (idx, depth) in bfs {
        levels.entry(depth).or_default().push(idx);
    }

    let max_level = levels.keys().max().copied().unwrap_or(0);

    for level in 0..=max_level {
        if let Some(nodes) = levels.get(&level) {
            let total_width = (nodes.len() as f32 - 1.0) * horizontal_spacing;
            let start_x = -total_width * 0.5;

            for (i, &idx) in nodes.iter().enumerate() {
                let x = start_x + i as f32 * horizontal_spacing;
                let y = level as f32 * vertical_spacing;
                positions.insert(idx, Vec2::new(x, y));
            }
        }
    }

    positions
}
