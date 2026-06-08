use glam::Vec2;
use uuid::Uuid;

use crate::camera::focus::CameraController;
use crate::graph::types::{GraphNode, NodeType, Relation, WorldGraph};
use crate::render::themes::Theme;

pub struct WorldState {
    pub graph: WorldGraph,
    pub camera: CameraController,
    pub theme: Theme,
    pub selected_node: Option<Uuid>,
    pub hovered_node: Option<Uuid>,
    pub search_query: String,
    pub search_results: Vec<Uuid>,
    pub zoom_level: f32,
    pub root_node: Option<petgraph::stable_graph::NodeIndex>,
    pub focus_stack: Vec<Uuid>,
    pub expanded_nodes: std::collections::HashSet<Uuid>,
    pub animating_nodes: std::collections::HashSet<Uuid>,
    pub node_anim_progress: std::collections::HashMap<Uuid, f32>,
    pub last_mouse_pos: Option<(u16, u16)>,
}

impl WorldState {
    pub fn new() -> Self {
        let mut graph = WorldGraph::new();
        let theme = Theme::void_dark();

        let root = GraphNode::new("VOID", NodeType::Root);
        let root_idx = graph.add_node(root);

        let mut state = Self {
            graph,
            camera: CameraController::new(),
            theme,
            selected_node: None,
            hovered_node: None,
            search_query: String::new(),
            search_results: Vec::new(),
            zoom_level: 1.0,
            root_node: Some(root_idx),
            focus_stack: Vec::new(),
            expanded_nodes: std::collections::HashSet::new(),
            animating_nodes: std::collections::HashSet::new(),
            node_anim_progress: std::collections::HashMap::new(),
            last_mouse_pos: None,
        };

        state.create_demo_world();
        state
    }

    fn create_demo_world(&mut self) {
        let root_idx = self.root_node.unwrap();
        self.expanded_nodes.insert(self.graph.get_node(root_idx).unwrap().id);

        // ── Code ──────────────────────────────────────────────
        let code = self.add_child(root_idx, "Code", NodeType::Cluster);

        let void = self.add_child(code, "void", NodeType::Project);
        let void_core = self.add_child(void, "core engine", NodeType::Note);
        let void_graph = self.add_child(void, "graph renderer", NodeType::Note);
        let void_camera = self.add_child(void, "camera system", NodeType::Note);
        let void_plugins = self.add_child(void, "plugin api", NodeType::Note);
        let _void_anim = self.add_child(void, "animation engine", NodeType::Note);

        let renderer = self.add_child(code, "kanri", NodeType::Project);
        let _kanri_tui = self.add_child(renderer, "tui dashboard", NodeType::Note);
        let _kanri_api = self.add_child(renderer, "rest client", NodeType::Note);

        let dotfiles = self.add_child(code, "dotfiles", NodeType::Project);
        let _nvim = self.add_child(dotfiles, "neovim config", NodeType::File);
        let _tmux = self.add_child(dotfiles, "tmux config", NodeType::File);
        let _zsh = self.add_child(dotfiles, "zsh / starship", NodeType::File);

        let _wezterm = self.add_child(code, "wezterm config", NodeType::File);

        // ── Notes ─────────────────────────────────────────────
        let notes = self.add_child(root_idx, "Notes", NodeType::Cluster);

        let rust_notes = self.add_child(notes, "Rust", NodeType::Knowledge);
        let _ownership = self.add_child(rust_notes, "ownership & borrowing", NodeType::Note);
        let _lifetimes = self.add_child(rust_notes, "lifetime elision", NodeType::Note);
        let _traits = self.add_child(rust_notes, "trait objects vs generics", NodeType::Note);
        let _async = self.add_child(rust_notes, "async / Pin / Future", NodeType::Note);
        let _unsafe_note = self.add_child(rust_notes, "unsafe & FFI", NodeType::Note);

        let gfx_notes = self.add_child(notes, "Graphics", NodeType::Knowledge);
        let _vulkan = self.add_child(gfx_notes, "vulkan pipeline", NodeType::Note);
        let _shaders = self.add_child(gfx_notes, "wgsl / glsl", NodeType::Note);
        let _deferred = self.add_child(gfx_notes, "deferred rendering", NodeType::Note);
        let _raytrace = self.add_child(gfx_notes, "ray tracing basics", NodeType::Note);

        let sys_notes = self.add_child(notes, "Systems Design", NodeType::Knowledge);
        let _ecs = self.add_child(sys_notes, "ecs architecture", NodeType::Note);
        let _event_driven = self.add_child(sys_notes, "event-driven design", NodeType::Note);
        let _caches = self.add_child(sys_notes, "cache-friendly data layouts", NodeType::Note);

        let phil_notes = self.add_child(notes, "Ideas", NodeType::Knowledge);
        let _spatial = self.add_child(phil_notes, "spatial computing", NodeType::Note);
        let _ai_tools = self.add_child(phil_notes, "ai as navigator", NodeType::Note);
        let _terminal = self.add_child(phil_notes, "terminal as world", NodeType::Note);

        // ── Music ─────────────────────────────────────────────
        let music = self.add_child(root_idx, "Music", NodeType::Cluster);

        let nuja = self.add_child(music, "Nujabes", NodeType::Artist);
        let _modal = self.add_child(nuja, "Modal Soul", NodeType::Album);
        let _modal_s1 = self.add_child(_modal, "Aruarian Dance", NodeType::Song);
        let _modal_s2 = self.add_child(_modal, "Reflection Eternal", NodeType::Song);
        let _modal_s3 = self.add_child(_modal, "Feather (feat. Cise Starr & Akin)", NodeType::Song);
        let _modal_s4 = self.add_child(_modal, "Luv(sic) Part 3 (feat. Shing02)", NodeType::Song);
        let _hydeout = self.add_child(nuja, "Hydeout Productions 1st Collection", NodeType::Album);
        let _hydeout_s1 = self.add_child(_hydeout, "Counting Stars", NodeType::Song);
        let _hydeout_s2 = self.add_child(_hydeout, "Lady Brown (feat. Cise Starr)", NodeType::Song);

        let djo = self.add_child(music, "DJ Okawari", NodeType::Artist);
        let _mirror = self.add_child(djo, "Mirror", NodeType::Album);
        let _mirror_s1 = self.add_child(_mirror, "Flower Dance", NodeType::Song);
        let _mirror_s2 = self.add_child(_mirror, "Luv Letter", NodeType::Song);
        let _mirror_s3 = self.add_child(_mirror, "Moonlight", NodeType::Song);

        let lopa = self.add_child(music, "Lopazz", NodeType::Artist);
        let _casio = self.add_child(lopa, "Casiopea", NodeType::Album);
        let _casio_s1 = self.add_child(_casio, "Asayake", NodeType::Song);
        let _casio_s2 = self.add_child(_casio, "Galactic Funk", NodeType::Song);

        let tokubo = self.add_child(music, "Takashi Kokubo", NodeType::Artist);
        let _island = self.add_child(tokubo, "Island of Music", NodeType::Album);
        let _island_s1 = self.add_child(_island, "Ibiza", NodeType::Song);

        let nitsua = self.add_child(music, "Nitsua", NodeType::Artist);
        let _home = self.add_child(nitsua, "Home", NodeType::Album);
        let _home_s1 = self.add_child(_home, "Sunset", NodeType::Song);

        // ── People ────────────────────────────────────────────
        let people = self.add_child(root_idx, "People", NodeType::Cluster);

        let _friend1 = self.add_child(people, "Emre", NodeType::Person);
        let _friend2 = self.add_child(people, "Zeynep", NodeType::Person);
        let _friend3 = self.add_child(people, "Arda", NodeType::Person);
        let _mentor = self.add_child(people, "Hocalarimdan biri", NodeType::Person);

        // ── Tasks ─────────────────────────────────────────────
        let tasks = self.add_child(root_idx, "Tasks", NodeType::Cluster);

        let _task1 = self.add_child(tasks, "void: add wasmtime plugin loader", NodeType::Task);
        let _task2 = self.add_child(tasks, "void: implement node search", NodeType::Task);
        let _task3 = self.add_child(tasks, "void: write integration tests", NodeType::Task);
        let _task4 = self.add_child(tasks, "read 'Programming Rust' ch.12-15", NodeType::Task);
        let _task5 = self.add_child(tasks, "set up homelab backup cron", NodeType::Task);
        let _task6 = self.add_child(tasks, "fix tmux status bar colors", NodeType::Task);

        // ── Bookmarks ─────────────────────────────────────────
        let bookmarks = self.add_child(root_idx, "Bookmarks", NodeType::Cluster);

        let _bm1 = self.add_child(bookmarks, "ratatui docs", NodeType::Bookmark);
        let _bm2 = self.add_child(bookmarks, "petgraph crate", NodeType::Bookmark);
        let _bm3 = self.add_child(bookmarks, "wasmtime component model", NodeType::Bookmark);
        let _bm4 = self.add_child(bookmarks, "Bevy ECS patterns", NodeType::Bookmark);
        let _bm5 = self.add_child(bookmarks, "Amit Patel - game programming patterns", NodeType::Bookmark);
        let _bm6 = self.add_child(bookmarks, "Chandler Carruth - back to basics (cppcon)", NodeType::Bookmark);

        // ── Cross-references (relationships) ──────────────────
        self.add_ref(void_graph, _ecs);
        self.add_ref(void_camera, _event_driven);
        self.add_ref(void_plugins, _unsafe_note);
        self.add_ref(void_core, _async);
        self.add_ref(_deferred, _vulkan);
        self.add_ref(_spatial, void_camera);
        self.add_ref(_terminal, void_graph);
        self.add_ref(_ai_tools, void_plugins);
        self.add_ref(_modal_s3, _mirror_s1);
        self.add_ref(nuja, djo);
        self.add_ref(_friend1, _task1);
        self.add_ref(_friend2, _task4);

        self.layout_children_of(root_idx);
    }

    fn add_ref(&mut self, from: petgraph::stable_graph::NodeIndex, to: petgraph::stable_graph::NodeIndex) {
        self.graph.add_edge(from, to, crate::graph::types::GraphEdge {
            relation: Relation::References,
            weight: 0.5,
        });
    }

    fn add_child(&mut self, parent: petgraph::stable_graph::NodeIndex, label: &str, node_type: NodeType) -> petgraph::stable_graph::NodeIndex {
        let child = GraphNode::new(label, node_type);
        let child_idx = self.graph.add_node(child);
        self.graph.add_edge(parent, child_idx, crate::graph::types::GraphEdge {
            relation: Relation::Contains,
            weight: 1.0,
        });
        child_idx
    }

    fn layout_children_of(&mut self, parent: petgraph::stable_graph::NodeIndex) {
        let children = self.graph.children_of(parent);
        let parent_pos = self.graph.get_node(parent).map(|n| n.position).unwrap_or(Vec2::ZERO);
        let count = children.len() as f32;

        if count == 0.0 {
            return;
        }

        let radius = (count * 3.0).max(5.0);
        let angle_step = std::f32::consts::TAU / count;

        for (i, &child_idx) in children.iter().enumerate() {
            let angle = angle_step * i as f32;
            let offset = Vec2::new(angle.cos(), angle.sin()) * radius;
            if let Some(node) = self.graph.get_node_mut(child_idx) {
                node.position = parent_pos + offset;
            }

            self.layout_children_of(child_idx);
        }
    }

    pub fn select_node(&mut self, id: Uuid) {
        self.selected_node = Some(id);
        if let Some(idx) = self.graph.node_by_id(id) {
            if let Some(node) = self.graph.get_node(idx) {
                self.camera.focus(node.position);
                self.camera.zoom_to(2.0);
            }
        }
    }

    pub fn zoom_into_selected(&mut self) {
        if let Some(id) = self.selected_node {
            self.zoom_into_node(id);
        }
    }

    pub fn zoom_into_node(&mut self, id: Uuid) {
        let idx = match self.graph.node_by_id(id) {
            Some(idx) => idx,
            None => return,
        };

        let children = self.graph.children_of(idx);
        if children.is_empty() {
            return;
        }

        if let Some(parent_id) = self.selected_node {
            self.focus_stack.push(parent_id);
        }

        self.selected_node = Some(id);
        self.expanded_nodes.insert(id);

        if let Some(node) = self.graph.get_node(idx) {
            self.camera.focus(node.position);
            self.camera.zoom_to(3.0);
        }

        for &child_idx in &children {
            if let Some(child) = self.graph.get_node_mut(child_idx) {
                self.animating_nodes.insert(child.id);
                self.node_anim_progress.insert(child.id, 0.0);
            }
        }

        self.layout_children_of(idx);
    }

    pub fn zoom_to_parent(&mut self) {
        if let Some(current_id) = self.selected_node {
            let idx = match self.graph.node_by_id(current_id) {
                Some(idx) => idx,
                None => return,
            };

            if let Some(parent_idx) = self.graph.parent_of(idx) {
                if let Some(parent) = self.graph.get_node(parent_idx) {
                    let parent_id = parent.id;
                    self.selected_node = Some(parent_id);
                    self.camera.focus(parent.position);
                    self.camera.zoom_to(2.0);
                }
            } else if let Some(restored_id) = self.focus_stack.pop() {
                self.selected_node = Some(restored_id);
                if let Some(restored_idx) = self.graph.node_by_id(restored_id) {
                    if let Some(node) = self.graph.get_node(restored_idx) {
                        self.camera.focus(node.position);
                        self.camera.zoom_to(2.0);
                    }
                }
            } else {
                self.focus_root();
            }
        } else {
            self.focus_root();
        }
    }

    pub fn cycle_children(&mut self, forward: bool) {
        if let Some(id) = self.selected_node {
            let idx = match self.graph.node_by_id(id) {
                Some(idx) => idx,
                None => return,
            };

            let parent_idx = self.graph.parent_of(idx);
            if let Some(parent_idx) = parent_idx {
                let siblings = self.graph.children_of(parent_idx);
                if !siblings.is_empty() {
                    let pos = siblings.iter().position(|&s| s == idx).unwrap_or(0);
                    let next_pos = if forward {
                        (pos + 1) % siblings.len()
                    } else {
                        (pos + siblings.len() - 1) % siblings.len()
                    };
                    if let Some(&sibling_idx) = siblings.get(next_pos) {
                        if let Some(child) = self.graph.get_node(sibling_idx) {
                            self.selected_node = Some(child.id);
                            self.camera.focus(child.position);
                        }
                    }
                }
            } else {
                // No parent (e.g. root node is selected).
                // Let's select its first child to go down one level.
                let children = self.graph.children_of(idx);
                if let Some(&child_idx) = children.first() {
                    if let Some(child) = self.graph.get_node(child_idx) {
                        self.selected_node = Some(child.id);
                        self.camera.focus(child.position);
                    }
                }
            }
        } else {
            if let Some(root_idx) = self.root_node {
                let children = self.graph.children_of(root_idx);
                if let Some(&child_idx) = children.first() {
                    if let Some(child) = self.graph.get_node(child_idx) {
                        self.selected_node = Some(child.id);
                        self.camera.focus(child.position);
                    }
                }
            }
        }
    }

    pub fn deselect(&mut self) {
        self.selected_node = None;
        self.camera.zoom_to(1.0);
    }

    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.search_results.clear();

        if query.is_empty() {
            return;
        }

        let query_lower = query.to_lowercase();
        for idx in self.graph.graph.node_indices() {
            if let Some(node) = self.graph.get_node(idx) {
                if node.label.to_lowercase().contains(&query_lower) {
                    self.search_results.push(node.id);
                }
            }
        }
    }

    pub fn navigate_to_search_result(&mut self, index: usize) {
        if let Some(&id) = self.search_results.get(index) {
            self.select_node(id);
        }
    }

    pub fn zoom_in(&mut self) {
        self.camera.zoom_in(1.2);
        self.zoom_level = self.camera.target_zoom;
    }

    pub fn zoom_out(&mut self) {
        self.camera.zoom_out(1.2);
        self.zoom_level = self.camera.target_zoom;
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.camera.pan(delta);
    }

    pub fn focus_root(&mut self) {
        self.selected_node = None;
        self.focus_stack.clear();
        self.camera.zoom_to(1.0);
        if let Some(root_idx) = self.root_node {
            if let Some(node) = self.graph.get_node(root_idx) {
                self.camera.focus(node.position);
            }
        }
    }

    pub fn update_animations(&mut self, dt: f32) {
        let mut completed = Vec::new();

        for &id in &self.animating_nodes {
            if let Some(progress) = self.node_anim_progress.get_mut(&id) {
                *progress = (*progress + dt * 3.0).min(1.0);
                if *progress >= 1.0 {
                    completed.push(id);
                }
            }
        }

        for id in completed {
            self.animating_nodes.remove(&id);
            self.node_anim_progress.remove(&id);
        }

        self.zoom_level = self.camera.zoom;
    }

    pub fn get_animation_progress(&self, id: Uuid) -> f32 {
        self.node_anim_progress.get(&id).copied().unwrap_or(1.0)
    }

    pub fn is_node_expanded(&self, id: Uuid) -> bool {
        self.expanded_nodes.contains(&id)
    }

    pub fn current_focus_depth(&self) -> usize {
        self.focus_stack.len()
    }

    pub fn get_selected_children(&self) -> Vec<Uuid> {
        if let Some(id) = self.selected_node {
            if let Some(idx) = self.graph.node_by_id(id) {
                return self.graph.children_of(idx)
                    .iter()
                    .filter_map(|&child_idx| self.graph.get_node(child_idx).map(|n| n.id))
                    .collect();
            }
        }
        Vec::new()
    }

    pub fn get_breadcrumb(&self) -> Vec<String> {
        let mut breadcrumb = Vec::new();
        for &id in &self.focus_stack {
            if let Some(idx) = self.graph.node_by_id(id) {
                if let Some(node) = self.graph.get_node(idx) {
                    breadcrumb.push(node.label.clone());
                }
            }
        }
        if let Some(id) = self.selected_node {
            if let Some(idx) = self.graph.node_by_id(id) {
                if let Some(node) = self.graph.get_node(idx) {
                    breadcrumb.push(node.label.clone());
                }
            }
        }
        breadcrumb
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}
