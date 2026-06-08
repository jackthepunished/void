use glam::Vec2;
use petgraph::stable_graph::NodeIndex;

use super::types::WorldGraph;

pub struct LayoutConfig {
    pub repulsion_strength: f32,
    pub attraction_strength: f32,
    pub ideal_distance: f32,
    pub damping: f32,
    pub gravity: f32,
    pub center_gravity: Vec2,
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
            max_iterations: 100,
            convergence_threshold: 0.001,
        }
    }
}

pub struct ForceDirectedLayout {
    config: LayoutConfig,
    velocities: std::collections::HashMap<NodeIndex, Vec2>,
}

impl ForceDirectedLayout {
    pub fn new(config: LayoutConfig) -> Self {
        Self {
            config,
            velocities: std::collections::HashMap::new(),
        }
    }

    pub fn step(&mut self, graph: &mut WorldGraph) -> f32 {
        let node_indices: Vec<NodeIndex> = graph.graph.node_indices().collect();
        let mut forces: std::collections::HashMap<NodeIndex, Vec2> = std::collections::HashMap::new();

        for &idx in &node_indices {
            forces.entry(idx).or_insert(Vec2::ZERO);
        }

        for i in 0..node_indices.len() {
            for j in (i + 1)..node_indices.len() {
                let a = node_indices[i];
                let b = node_indices[j];

                let pos_a = graph.get_node(a).map(|n| n.position).unwrap_or(Vec2::ZERO);
                let pos_b = graph.get_node(b).map(|n| n.position).unwrap_or(Vec2::ZERO);

                let diff = pos_a - pos_b;
                let dist = diff.length().max(0.01);
                let dir = diff / dist;

                let repulsion = self.config.repulsion_strength / (dist * dist);
                let repulsion_force = dir * repulsion;

                *forces.entry(a).or_insert(Vec2::ZERO) += repulsion_force;
                *forces.entry(b).or_insert(Vec2::ZERO) -= repulsion_force;
            }
        }

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

                *forces.entry(source).or_insert(Vec2::ZERO) += attraction_force;
                *forces.entry(target).or_insert(Vec2::ZERO) -= attraction_force;
            }
        }

        for &idx in &node_indices {
            let pos = graph.get_node(idx).map(|n| n.position).unwrap_or(Vec2::ZERO);
            let to_center = self.config.center_gravity - pos;
            *forces.entry(idx).or_insert(Vec2::ZERO) += to_center * self.config.gravity;
        }

        let mut total_movement = 0.0;
        for &idx in &node_indices {
            let force = forces.get(&idx).copied().unwrap_or(Vec2::ZERO);
            let velocity = self.velocities.entry(idx).or_insert(Vec2::ZERO);

            *velocity = (*velocity + force) * self.config.damping;
            let speed = velocity.length();
            if speed > 5.0 {
                *velocity = *velocity / speed * 5.0;
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
    }
}

pub fn hierarchical_layout(graph: &WorldGraph, root: NodeIndex, horizontal_spacing: f32, vertical_spacing: f32) -> std::collections::HashMap<NodeIndex, Vec2> {
    let mut positions = std::collections::HashMap::new();
    let mut levels: std::collections::HashMap<u32, Vec<NodeIndex>> = std::collections::HashMap::new();

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
