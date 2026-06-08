use glam::Vec2;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::camera::transform::CameraTransform;
use crate::graph::types::{NodeType, WorldGraph};
use crate::render::node_renderer::{EdgeRenderer, NodeRenderer};
use crate::render::themes::Theme;

pub struct RenderContext<'a> {
    pub graph: &'a WorldGraph,
    pub camera: &'a CameraTransform,
    pub selected: Option<uuid::Uuid>,
    pub hovered: Option<uuid::Uuid>,
    pub search_query: &'a str,
    pub zoom_level: f32,
    pub search_results: &'a [uuid::Uuid],
    pub anim_progress: &'a std::collections::HashMap<uuid::Uuid, f32>,
    pub breadcrumb: &'a [String],
}

pub struct Renderer {
    node_renderer: NodeRenderer,
    edge_renderer: EdgeRenderer,
    theme: Theme,
}

impl Renderer {
    pub fn new(theme: Theme) -> Self {
        Self {
            node_renderer: NodeRenderer::new(theme.clone()),
            edge_renderer: EdgeRenderer::new(theme.clone()),
            theme,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, ctx: &RenderContext) {
        let block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(self.theme.background));
        frame.render_widget(Clear, area);
        frame.render_widget(block, area);

        self.render_edges(frame, area, ctx.graph, ctx.camera);
        self.render_void_ascii(frame, area, ctx.zoom_level);
        self.render_cluster_boundaries(frame, area, ctx);
        self.render_nodes(frame, area, ctx);
        self.render_hud(frame, area, ctx);
    }

    fn render_edges(
        &self,
        frame: &mut Frame,
        area: Rect,
        graph: &WorldGraph,
        camera: &CameraTransform,
    ) {
        let (min, max) = camera.visible_world_rect();
        let margin = 2.0;

        for edge_idx in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge_idx) {
                let from = graph.get_node(source).map(|n| n.position).unwrap_or(Vec2::ZERO);
                let to = graph.get_node(target).map(|n| n.position).unwrap_or(Vec2::ZERO);

                if !Self::line_visible(from, to, min, max, margin) {
                    continue;
                }

                let points = self.edge_renderer.render_edge(from, to, camera);
                for (x, y, glyph, color) in points {
                    let screen_x = area.x + x.min(area.width.saturating_sub(1));
                    let screen_y = area.y + y.min(area.height.saturating_sub(1));

                    let span = Span::styled(glyph.to_string(), Style::default().fg(color));
                    let line = Line::from(span);
                    let paragraph = Paragraph::new(line);
                    frame.render_widget(paragraph, Rect::new(screen_x, screen_y, 1, 1));
                }
            }
        }
    }

    fn line_visible(from: Vec2, to: Vec2, min: Vec2, max: Vec2, margin: f32) -> bool {
        let line_min = Vec2::new(from.x.min(to.x), from.y.min(to.y));
        let line_max = Vec2::new(from.x.max(to.x), from.y.max(to.y));
        line_min.x <= max.x + margin
            && line_max.x >= min.x - margin
            && line_min.y <= max.y + margin
            && line_max.y >= min.y - margin
    }

    fn render_void_ascii(
        &self,
        frame: &mut Frame,
        area: Rect,
        zoom_level: f32,
    ) {
        let threshold = 0.35;
        if zoom_level > threshold {
            return;
        }

        let intensity = (1.0 - zoom_level / threshold).clamp(0.0, 1.0);
        let alpha = (intensity * 40.0) as u8;

        let ascii_art = [
            r"██╗   ██╗ ██████╗ ██╗██████╗ ",
            r"██║   ██║██╔═══██╗██║██╔══██╗",
            r"██║   ██║██║   ██║██║██║  ██║",
            r"╚██╗ ██╔╝██║   ██║██║██║  ██║",
            r" ╚████╔╝ ╚██████╔╝██║██████╔╝",
            r"  ╚═══╝   ╚═════╝ ╚═╝╚═════╝",
        ];

        let art_width = ascii_art[0].len() as u16;
        let art_height = ascii_art.len() as u16;

        let center_x = area.width / 2;
        let center_y = area.height / 2;
        let start_x = center_x.saturating_sub(art_width / 2);
        let start_y = center_y.saturating_sub(art_height / 2);

        for (i, line) in ascii_art.iter().enumerate() {
            let y = start_y + i as u16;
            if y >= area.height {
                break;
            }

            let span = Span::styled(
                line.to_string(),
                Style::default().fg(Color::Rgb(alpha, alpha, (alpha as f32 * 1.2) as u8)),
            );
            let render_line = Line::from(span);
            let p = Paragraph::new(render_line);
            frame.render_widget(p, Rect::new(start_x, y, art_width, 1));
        }
    }

    fn render_cluster_boundaries(
        &self,
        frame: &mut Frame,
        area: Rect,
        ctx: &RenderContext,
    ) {
        let graph = ctx.graph;
        let camera = ctx.camera;
        let selected = ctx.selected;
        for idx in graph.graph.node_indices() {
            let node = match graph.get_node(idx) {
                Some(n) => n,
                None => continue,
            };

            if node.node_type != NodeType::Cluster {
                continue;
            }

            if !camera.contains_world_pos(node.position, 15.0) {
                continue;
            }

            let children = graph.children_of(idx);
            if children.is_empty() {
                continue;
            }

            let mut min_pos = node.position;
            let mut max_pos = node.position;

            for &child_idx in &children {
                if let Some(child) = graph.get_node(child_idx) {
                    min_pos.x = min_pos.x.min(child.position.x - 2.0);
                    min_pos.y = min_pos.y.min(child.position.y - 1.0);
                    max_pos.x = max_pos.x.max(child.position.x + 8.0);
                    max_pos.y = max_pos.y.max(child.position.y + 1.0);
                }
            }

            let center = (min_pos + max_pos) * 0.5;
            let radius = ((max_pos.x - min_pos.x).max(max_pos.y - min_pos.y)) * 0.5;
            let is_selected = selected.map_or(false, |s| s == node.id);

            self.node_renderer.render_cluster_boundary(
                frame,
                area,
                center,
                radius + 2.0,
                camera,
                &node.label,
                is_selected,
            );
        }
    }

    fn render_nodes(
        &self,
        frame: &mut Frame,
        area: Rect,
        ctx: &RenderContext,
    ) {
        let graph = ctx.graph;
        let camera = ctx.camera;
        let selected = ctx.selected;
        let hovered = ctx.hovered;
        let search_results = ctx.search_results;
        let anim_progress = ctx.anim_progress;
        let mut node_positions: Vec<(u16, u16, Line<'static>, u8)> = Vec::new();

        for idx in graph.graph.node_indices() {
            let node = match graph.get_node(idx) {
                Some(n) => n,
                None => continue,
            };

            if !camera.contains_world_pos(node.position, 5.0) {
                continue;
            }

            let is_selected = selected.map_or(false, |s| s == node.id);
            let is_hovered = hovered.map_or(false, |h| h == node.id);
            let is_search_result = search_results.contains(&node.id);
            let anim = anim_progress.get(&node.id).copied().unwrap_or(1.0);

            let priority = if is_selected { 3 } else if is_hovered { 2 } else if is_search_result { 1 } else { 0 };

            if let Some((x, y, line)) = self.node_renderer.render_node_animated(
                node, camera, is_selected, is_hovered, is_search_result, anim,
            ) {
                node_positions.push((x, y, line, priority));
            }
        }

        node_positions.sort_by_key(|p| p.3);

        for (x, y, line, _) in node_positions {
            let screen_x = area.x + x.min(area.width.saturating_sub(1));
            let screen_y = area.y + y.min(area.height.saturating_sub(1));

            let paragraph = Paragraph::new(line.clone());
            let line_width = line.width() as u16;
            let render_area = Rect::new(
                screen_x,
                screen_y,
                line_width.min(area.width.saturating_sub(x)),
                1,
            );
            frame.render_widget(paragraph, render_area);
        }
    }

    fn render_hud(
        &self,
        frame: &mut Frame,
        area: Rect,
        ctx: &RenderContext,
    ) {
        let camera = ctx.camera;
        let zoom_level = ctx.zoom_level;
        let node_count = ctx.graph.node_count();
        let search_query = ctx.search_query;
        let breadcrumb = ctx.breadcrumb;
        if !breadcrumb.is_empty() {
            let breadcrumb_text = breadcrumb.join(" > ");
            let breadcrumb_line = Line::from(vec![
                Span::styled(" ", Style::default().fg(self.theme.dim)),
                Span::styled(breadcrumb_text, Style::default().fg(self.theme.text_secondary)),
            ]);
            let breadcrumb_widget = Paragraph::new(breadcrumb_line);
            let breadcrumb_area = Rect::new(0, 0, area.width, 1);
            frame.render_widget(breadcrumb_widget, breadcrumb_area);
        }

        let zoom_text = format!("Zoom: {:.2}x", zoom_level);
        let nodes_text = format!("Nodes: {}", node_count);
        let pos_text = format!("Pos: ({:.1}, {:.1})", camera.position.x, camera.position.y);

        let help_text = "q:quit  arrows:navigate  enter:zoom in  backspace:zoom out  /:search";

        let status_line = Line::from(vec![
            Span::styled(" VOID ", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled(" | ", Style::default().fg(self.theme.dim)),
            Span::styled(zoom_text, Style::default().fg(self.theme.text_secondary)),
            Span::styled(" | ", Style::default().fg(self.theme.dim)),
            Span::styled(nodes_text, Style::default().fg(self.theme.text_secondary)),
            Span::styled(" | ", Style::default().fg(self.theme.dim)),
            Span::styled(pos_text, Style::default().fg(self.theme.text_secondary)),
        ]);

        let status = Paragraph::new(status_line);
        let status_area = Rect::new(0, area.height.saturating_sub(1), area.width, 1);
        frame.render_widget(status, status_area);

        let help_line = Line::from(vec![
            Span::styled(help_text, Style::default().fg(self.theme.dim)),
        ]);
        let help = Paragraph::new(help_line);
        let help_area = Rect::new(0, area.height.saturating_sub(2), area.width, 1);
        frame.render_widget(help, help_area);

        if !search_query.is_empty() {
            let display_query = if search_query.starts_with('/') {
                search_query[1..].to_string()
            } else {
                search_query.to_string()
            };
            let search_line = Line::from(vec![
                Span::styled(" /", Style::default().fg(self.theme.accent)),
                Span::styled(display_query, Style::default().fg(self.theme.text_primary)),
                Span::styled("_", Style::default().fg(self.theme.accent)),
            ]);
            let search = Paragraph::new(search_line);
            let search_area = Rect::new(0, area.height.saturating_sub(3), area.width, 1);
            frame.render_widget(search, search_area);
        }
    }
}
