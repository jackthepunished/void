use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub dim: Color,
    pub node_colors: NodeColors,
    pub edge_color: Color,
    pub selected_color: Color,
    pub hovered_color: Color,
    pub cluster_color: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
}

#[derive(Debug, Clone)]
pub struct NodeColors {
    pub root: Color,
    pub project: Color,
    pub task: Color,
    pub note: Color,
    pub file: Color,
    pub person: Color,
    pub music: Color,
    pub artist: Color,
    pub album: Color,
    pub song: Color,
    pub bookmark: Color,
    pub knowledge: Color,
    pub event: Color,
    pub cluster: Color,
    pub custom: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::void_dark()
    }
}

impl Theme {
    pub fn void_dark() -> Self {
        Self {
            background: Color::Rgb(10, 10, 15),
            foreground: Color::Rgb(200, 200, 210),
            accent: Color::Rgb(120, 180, 255),
            dim: Color::Rgb(60, 60, 80),
            node_colors: NodeColors {
                root: Color::Rgb(255, 200, 100),
                project: Color::Rgb(100, 200, 150),
                task: Color::Rgb(200, 100, 100),
                note: Color::Rgb(150, 150, 200),
                file: Color::Rgb(180, 180, 180),
                person: Color::Rgb(200, 150, 200),
                music: Color::Rgb(255, 150, 200),
                artist: Color::Rgb(255, 180, 100),
                album: Color::Rgb(180, 100, 255),
                song: Color::Rgb(100, 200, 255),
                bookmark: Color::Rgb(200, 200, 100),
                knowledge: Color::Rgb(100, 180, 200),
                event: Color::Rgb(150, 200, 100),
                cluster: Color::Rgb(80, 80, 120),
                custom: Color::Rgb(150, 150, 150),
            },
            edge_color: Color::Rgb(50, 50, 70),
            selected_color: Color::Rgb(255, 255, 100),
            hovered_color: Color::Rgb(180, 220, 255),
            cluster_color: Color::Rgb(30, 30, 50),
            text_primary: Color::Rgb(220, 220, 230),
            text_secondary: Color::Rgb(120, 120, 140),
        }
    }

    pub fn nebula() -> Self {
        Self {
            background: Color::Rgb(5, 5, 20),
            foreground: Color::Rgb(180, 190, 220),
            accent: Color::Rgb(150, 100, 255),
            dim: Color::Rgb(40, 40, 70),
            node_colors: NodeColors {
                root: Color::Rgb(255, 180, 80),
                project: Color::Rgb(80, 220, 170),
                task: Color::Rgb(220, 80, 80),
                note: Color::Rgb(130, 130, 220),
                file: Color::Rgb(160, 160, 180),
                person: Color::Rgb(220, 130, 220),
                music: Color::Rgb(255, 130, 220),
                artist: Color::Rgb(255, 160, 80),
                album: Color::Rgb(160, 80, 255),
                song: Color::Rgb(80, 220, 255),
                bookmark: Color::Rgb(220, 220, 80),
                knowledge: Color::Rgb(80, 200, 220),
                event: Color::Rgb(130, 220, 80),
                cluster: Color::Rgb(60, 60, 100),
                custom: Color::Rgb(130, 130, 150),
            },
            edge_color: Color::Rgb(40, 40, 65),
            selected_color: Color::Rgb(255, 255, 120),
            hovered_color: Color::Rgb(200, 200, 255),
            cluster_color: Color::Rgb(20, 20, 45),
            text_primary: Color::Rgb(200, 200, 230),
            text_secondary: Color::Rgb(100, 100, 130),
        }
    }
}
