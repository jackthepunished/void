use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::Terminal;

use crate::camera::transform::CameraTransform;
use crate::core::input::{InputEvent, InputHandler, handle_key_event, handle_mouse_event};
use crate::core::world::WorldState;
use crate::render::Renderer;

pub struct App {
    world: WorldState,
    renderer: Renderer,
    input: InputHandler,
    running: bool,
    tick_rate: Duration,
    last_tick: Instant,
}

impl App {
    pub fn new() -> Result<Self> {
        let world = WorldState::new();
        let renderer = Renderer::new(world.theme.clone());
        let input = InputHandler::new();

        Ok(Self {
            world,
            renderer,
            input,
            running: true,
            tick_rate: Duration::from_millis(16),
            last_tick: Instant::now(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        loop {
            let now = Instant::now();
            let dt = now.duration_since(self.last_tick).as_secs_f32();
            self.last_tick = now;

            if dt > 0.0 && dt < 1.0 {
                self.world.camera.update(dt);
                self.world.update_animations(dt);
            }

            let search_results = self.world.search_results.clone();
            let breadcrumb = self.world.get_breadcrumb();
            let anim_progress = self.world.node_anim_progress.clone();

            terminal.draw(|frame| {
                let area = frame.area();
                let camera = CameraTransform {
                    position: self.world.camera.position,
                    zoom: self.world.camera.zoom,
                    rotation: 0.0,
                    viewport_width: area.width as f32,
                    viewport_height: area.height as f32,
                };

                self.renderer.render(
                    frame,
                    area,
                    &self.world.graph,
                    &camera,
                    self.world.selected_node,
                    self.world.hovered_node,
                    &self.world.search_query,
                    self.world.zoom_level,
                    &search_results,
                    &anim_progress,
                    &breadcrumb,
                );
            })?;

            tokio::select! {
                event = self.input.next_event() => {
                    match event {
                        Some(InputEvent::Key(key)) => {
                            if handle_key_event(key, &mut self.world) {
                                self.running = false;
                            }
                        }
                        Some(InputEvent::Mouse(mouse)) => {
                            handle_mouse_event(mouse, &mut self.world);
                        }
                        Some(InputEvent::Resize(w, h)) => {
                            terminal.resize(Rect::new(0, 0, w, h))?;
                        }
                        Some(InputEvent::Quit) => {
                            self.running = false;
                        }
                        None => {
                            self.running = false;
                        }
                        _ => {}
                    }
                }
                _ = tokio::time::sleep(self.tick_rate) => {}
            }

            if !self.running {
                break;
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().expect("Failed to create application")
    }
}
