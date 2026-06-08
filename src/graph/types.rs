use glam::Vec2;
use petgraph::visit::EdgeRef;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: Uuid,
    pub label: String,
    pub node_type: NodeType,
    pub position: Vec2,
    pub depth: u32,
    pub collapsed: bool,
}

impl GraphNode {
    pub fn new(label: impl Into<String>, node_type: NodeType) -> Self {
        Self {
            id: Uuid::new_v4(),
            label: label.into(),
            node_type,
            position: Vec2::ZERO,
            depth: 0,
            collapsed: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum NodeType {
    Root,
    Project,
    Task,
    Note,
    File,
    Person,
    Music,
    Artist,
    Album,
    Song,
    Bookmark,
    Knowledge,
    Event,
    Cluster,
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub relation: Relation,
    pub weight: f32,
}

impl Default for GraphEdge {
    fn default() -> Self {
        Self {
            relation: Relation::RelatedTo,
            weight: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Relation {
    Contains,
    DependsOn,
    RelatedTo,
    CreatedBy,
    References,
    InspiredBy,
    ConnectedTo,
    ScheduledFor,
}

pub struct WorldGraph {
    pub graph: StableGraph<GraphNode, GraphEdge>,
    node_map: std::collections::HashMap<Uuid, NodeIndex>,
}

impl WorldGraph {
    pub fn new() -> Self {
        Self {
            graph: StableGraph::default(),
            node_map: std::collections::HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: GraphNode) -> NodeIndex {
        let id = node.id;
        let idx = self.graph.add_node(node);
        self.node_map.insert(id, idx);
        idx
    }

    pub fn add_edge(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        edge: GraphEdge,
    ) -> Option<petgraph::stable_graph::EdgeIndex> {
        Some(self.graph.add_edge(from, to, edge))
    }

    pub fn node_by_id(&self, id: Uuid) -> Option<NodeIndex> {
        self.node_map.get(&id).copied()
    }

    pub fn get_node(&self, idx: NodeIndex) -> Option<&GraphNode> {
        self.graph.node_weight(idx)
    }

    pub fn get_node_mut(&mut self, idx: NodeIndex) -> Option<&mut GraphNode> {
        self.graph.node_weight_mut(idx)
    }

    pub fn children_of(&self, idx: NodeIndex) -> Vec<NodeIndex> {
        self.graph
            .neighbors_directed(idx, petgraph::Direction::Outgoing)
            .collect()
    }

    pub fn parent_of(&self, idx: NodeIndex) -> Option<NodeIndex> {
        self.graph
            .neighbors_directed(idx, petgraph::Direction::Incoming)
            .next()
    }

    pub fn connected_to(&self, idx: NodeIndex) -> Vec<(NodeIndex, &GraphEdge)> {
        self.graph
            .edges(idx)
            .map(|e| (e.target(), e.weight()))
            .collect()
    }

    pub fn remove_node(&mut self, idx: NodeIndex) -> Option<GraphNode> {
        let node = self.graph.remove_node(idx)?;
        self.node_map.remove(&node.id);
        Some(node)
    }

    pub fn find_scc(&self) -> Vec<Vec<NodeIndex>> {
        petgraph::algo::kosaraju_scc(&self.graph)
    }

    pub fn bfs_from(&self, start: NodeIndex, max_depth: u32) -> Vec<(NodeIndex, u32)> {
        use std::collections::{VecDeque, HashSet};
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((idx, depth)) = queue.pop_front() {
            if depth > max_depth {
                continue;
            }
            result.push((idx, depth));

            for neighbor in self.graph.neighbors(idx) {
                if visited.insert(neighbor) {
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }

        result
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl Default for WorldGraph {
    fn default() -> Self {
        Self::new()
    }
}
