use glam::Vec2;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui::layout::Rect;

use crate::camera::transform::CameraTransform;
use crate::graph::types::{GraphNode, NodeType};
use crate::render::themes::Theme;

pub struct NodeRenderer {
    theme: Theme,
}

impl NodeRenderer {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }

    pub fn render_node(
        &self,
        node: &GraphNode,
        camera: &CameraTransform,
        selected: bool,
        hovered: bool,
    ) -> Option<(u16, u16, Line<'static>)> {
        self.render_node_animated(node, camera, selected, hovered, false, 1.0)
    }

    pub fn render_node_animated(
        &self,
        node: &GraphNode,
        camera: &CameraTransform,
        selected: bool,
        hovered: bool,
        search_result: bool,
        anim_progress: f32,
    ) -> Option<(u16, u16, Line<'static>)> {
        let screen_pos = camera.world_to_screen(node.position);

        let x = screen_pos.x as i32;
        let y = screen_pos.y as i32;

        if x < 0 || y < 0 || x >= camera.viewport_width as i32 || y >= camera.viewport_height as i32 {
            return None;
        }

        let glyph = self.node_glyph(&node.node_type);
        let color = self.node_color(&node.node_type);

        let style = if selected {
            Style::default()
                .fg(self.theme.selected_color)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else if hovered {
            Style::default()
                .fg(self.theme.hovered_color)
                .add_modifier(Modifier::BOLD)
        } else if search_result {
            Style::default()
                .fg(Color::Rgb(255, 255, 100))
                .add_modifier(Modifier::BOLD)
        } else {
            let dimmed = if anim_progress < 1.0 {
                let (r, g, b) = color_to_rgb(color);
                let p = anim_progress;
                Color::Rgb((r as f32 * p) as u8, (g as f32 * p) as u8, (b as f32 * p) as u8)
            } else {
                color
            };
            Style::default().fg(dimmed)
        };

        let label = if node.label.len() > 20 {
            format!("{}...", &node.label[..17])
        } else {
            node.label.clone()
        };

        let display = format!("{} {}", glyph, label);
        let span = Span::styled(display, style);
        let line = Line::from(span);

        Some((x as u16, y as u16, line))
    }

    pub fn render_cluster_boundary(
        &self,
        frame: &mut Frame,
        area: Rect,
        center: Vec2,
        radius: f32,
        camera: &CameraTransform,
        label: &str,
        is_selected: bool,
    ) {
        let screen_center = camera.world_to_screen(center);
        let screen_radius = radius * camera.zoom;

        let min_x = (screen_center.x - screen_radius) as i32;
        let max_x = (screen_center.x + screen_radius) as i32;
        let min_y = (screen_center.y - screen_radius * 0.6) as i32;
        let max_y = (screen_center.y + screen_radius * 0.6) as i32;

        let border_color = if is_selected {
            self.theme.accent
        } else {
            self.theme.cluster_color
        };

        for x in min_x..=max_x {
            if x >= 0 && (x as u16) < area.width {
                if min_y >= 0 && (min_y as u16) < area.height {
                    let span = Span::styled("\u{2500}", Style::default().fg(border_color));
                    let line = Line::from(span);
                    let p = Paragraph::new(line);
                    frame.render_widget(p, Rect::new(x as u16, min_y as u16, 1, 1));
                }
                if max_y >= 0 && (max_y as u16) < area.height {
                    let span = Span::styled("\u{2500}", Style::default().fg(border_color));
                    let line = Line::from(span);
                    let p = Paragraph::new(line);
                    frame.render_widget(p, Rect::new(x as u16, max_y as u16, 1, 1));
                }
            }
        }

        for y in min_y..=max_y {
            if y >= 0 && (y as u16) < area.height {
                if min_x >= 0 && (min_x as u16) < area.width {
                    let span = Span::styled("\u{2502}", Style::default().fg(border_color));
                    let line = Line::from(span);
                    let p = Paragraph::new(line);
                    frame.render_widget(p, Rect::new(min_x as u16, y as u16, 1, 1));
                }
                if max_x >= 0 && (max_x as u16) < area.width {
                    let span = Span::styled("\u{2502}", Style::default().fg(border_color));
                    let line = Line::from(span);
                    let p = Paragraph::new(line);
                    frame.render_widget(p, Rect::new(max_x as u16, y as u16, 1, 1));
                }
            }
        }

        if min_x >= 0 && (min_x as u16) < area.width && min_y >= 0 && (min_y as u16) < area.height {
            let span = Span::styled("\u{250C}", Style::default().fg(border_color));
            let line = Line::from(span);
            let p = Paragraph::new(line);
            frame.render_widget(p, Rect::new(min_x as u16, min_y as u16, 1, 1));
        }
        if max_x >= 0 && (max_x as u16) < area.width && min_y >= 0 && (min_y as u16) < area.height {
            let span = Span::styled("\u{2510}", Style::default().fg(border_color));
            let line = Line::from(span);
            let p = Paragraph::new(line);
            frame.render_widget(p, Rect::new(max_x as u16, min_y as u16, 1, 1));
        }
        if min_x >= 0 && (min_x as u16) < area.width && max_y >= 0 && (max_y as u16) < area.height {
            let span = Span::styled("\u{2514}", Style::default().fg(border_color));
            let line = Line::from(span);
            let p = Paragraph::new(line);
            frame.render_widget(p, Rect::new(min_x as u16, max_y as u16, 1, 1));
        }
        if max_x >= 0 && (max_x as u16) < area.width && max_y >= 0 && (max_y as u16) < area.height {
            let span = Span::styled("\u{2518}", Style::default().fg(border_color));
            let line = Line::from(span);
            let p = Paragraph::new(line);
            frame.render_widget(p, Rect::new(max_x as u16, max_y as u16, 1, 1));
        }

        let label_x = min_x + 2;
        let label_y = min_y;
        if label_x >= 0 && (label_x as u16) < area.width && label_y >= 0 && (label_y as u16) < area.height {
            let display = format!(" {} ", label);
            let display_len = display.len() as u16;
            let span = Span::styled(display, Style::default().fg(border_color).add_modifier(Modifier::BOLD));
            let line = Line::from(span);
            let p = Paragraph::new(line);
            frame.render_widget(p, Rect::new(label_x as u16, label_y as u16, display_len, 1));
        }
    }

    fn node_glyph(&self, node_type: &NodeType) -> char {
        match node_type {
            NodeType::Root => '\u{25C8}',
            NodeType::Project => '\u{25C6}',
            NodeType::Task => '\u{25A0}',
            NodeType::Note => '\u{25CF}',
            NodeType::File => '\u{25C7}',
            NodeType::Person => '\u{25C9}',
            NodeType::Music => '\u{266B}',
            NodeType::Artist => '\u{266A}',
            NodeType::Album => '\u{25A3}',
            NodeType::Song => '\u{266C}',
            NodeType::Bookmark => '\u{2605}',
            NodeType::Knowledge => '\u{25CE}',
            NodeType::Event => '\u{25CB}',
            NodeType::Cluster => '\u{2B21}',
        }
    }

    fn node_color(&self, node_type: &NodeType) -> Color {
        match node_type {
            NodeType::Root => self.theme.node_colors.root,
            NodeType::Project => self.theme.node_colors.project,
            NodeType::Task => self.theme.node_colors.task,
            NodeType::Note => self.theme.node_colors.note,
            NodeType::File => self.theme.node_colors.file,
            NodeType::Person => self.theme.node_colors.person,
            NodeType::Music => self.theme.node_colors.music,
            NodeType::Artist => self.theme.node_colors.artist,
            NodeType::Album => self.theme.node_colors.album,
            NodeType::Song => self.theme.node_colors.song,
            NodeType::Bookmark => self.theme.node_colors.bookmark,
            NodeType::Knowledge => self.theme.node_colors.knowledge,
            NodeType::Event => self.theme.node_colors.event,
            NodeType::Cluster => self.theme.node_colors.cluster,
        }
    }
}

pub struct EdgeRenderer {
    theme: Theme,
}

impl EdgeRenderer {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }

    pub fn render_edge(
        &self,
        from: Vec2,
        to: Vec2,
        camera: &CameraTransform,
    ) -> Vec<(u16, u16, char, Color)> {
        let from_screen = camera.world_to_screen(from);
        let to_screen = camera.world_to_screen(to);

        let points = bresenham_line(
            from_screen.x as i32,
            from_screen.y as i32,
            to_screen.x as i32,
            to_screen.y as i32,
        );

        points
            .into_iter()
            .filter(|(x, y)| {
                *x >= 0 && *y >= 0 && (*x as f32) < camera.viewport_width && (*y as f32) < camera.viewport_height
            })
            .map(|(x, y)| {
                let dx = to_screen.x as i32 - from_screen.x as i32;
                let dy = to_screen.y as i32 - from_screen.y as i32;
                let glyph = if dx.abs() > dy.abs() {
                    '\u{2500}'
                } else {
                    '\u{2502}'
                };
                (x as u16, y as u16, glyph, self.theme.edge_color)
            })
            .collect()
    }
}

fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        points.push((x, y));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
        if points.len() > 500 {
            break;
        }
    }

    points
}

fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Red => (255, 0, 0),
        Color::Green => (0, 255, 0),
        Color::Blue => (0, 0, 255),
        Color::Yellow => (255, 255, 0),
        Color::Cyan => (0, 255, 255),
        Color::Magenta => (255, 0, 255),
        Color::White => (255, 255, 255),
        Color::Gray => (128, 128, 128),
        Color::DarkGray => (64, 64, 64),
        _ => (200, 200, 200),
    }
}
