use glam::Vec2;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub node_type: NodeType,
    pub label: String,
    pub depth: u32,
    pub collapsed: bool,
}

impl Node {
    pub fn new(node_type: NodeType, label: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            node_type,
            label: label.into(),
            depth: 0,
            collapsed: false,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new(NodeType::Root, "")
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

impl Default for NodeType {
    fn default() -> Self {
        Self::Root
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub Vec2);

#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Clone, Copy, Default)]
pub struct Selectable {
    pub selected: bool,
    pub hovered: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct NodeAppearance {
    pub glyph: char,
    pub color: (u8, u8, u8),
}

impl Default for NodeAppearance {
    fn default() -> Self {
        Self {
            glyph: '●',
            color: (200, 200, 200),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AnimatePosition {
    pub target: Vec2,
    pub speed: f32,
    pub active: bool,
}

impl AnimatePosition {
    pub fn to(target: Vec2) -> Self {
        Self {
            target,
            speed: 8.0,
            active: true,
        }
    }
}

impl Default for AnimatePosition {
    fn default() -> Self {
        Self {
            target: Vec2::ZERO,
            speed: 8.0,
            active: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Zoomable {
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
    pub zoom_threshold: f32,
}

impl Default for Zoomable {
    fn default() -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            zoom_threshold: 1.0,
        }
    }
}
