use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use futures::StreamExt;
use tokio::sync::mpsc;

pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Tick,
    Quit,
}

pub struct InputHandler {
    rx: mpsc::UnboundedReceiver<InputEvent>,
    _tx: mpsc::UnboundedSender<InputEvent>,
}

impl InputHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let mut stream = EventStream::new();
            while let Some(event) = stream.next().await {
                match event {
                    Ok(Event::Key(key)) => {
                        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                            let _ = tx_clone.send(InputEvent::Quit);
                        } else {
                            let _ = tx_clone.send(InputEvent::Key(key));
                        }
                    }
                    Ok(Event::Mouse(mouse)) => {
                        let _ = tx_clone.send(InputEvent::Mouse(mouse));
                    }
                    Ok(Event::Resize(w, h)) => {
                        let _ = tx_clone.send(InputEvent::Resize(w, h));
                    }
                    Err(_) => {
                        let _ = tx_clone.send(InputEvent::Quit);
                    }
                    _ => {}
                }
            }
        });

        Self { rx, _tx: tx }
    }

    pub async fn next_event(&mut self) -> Option<InputEvent> {
        self.rx.recv().await
    }
}

pub fn handle_key_event(key: KeyEvent, state: &mut crate::core::world::WorldState) -> bool {
    if !state.search_query.is_empty() {
        return handle_search_input(key, state);
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            if state.selected_node.is_some() {
                state.deselect();
                false
            } else {
                true
            }
        }
        KeyCode::Char('+') | KeyCode::Char('=') => {
            state.zoom_in();
            false
        }
        KeyCode::Char('-') => {
            state.zoom_out();
            false
        }
        KeyCode::Up => {
            state.pan(glam::Vec2::new(0.0, -2.0));
            false
        }
        KeyCode::Down => {
            state.pan(glam::Vec2::new(0.0, 2.0));
            false
        }
        KeyCode::Left => {
            state.cycle_children(false);
            false
        }
        KeyCode::Right => {
            state.cycle_children(true);
            false
        }
        KeyCode::Home => {
            state.focus_root();
            false
        }
        KeyCode::Enter => {
            state.zoom_into_selected();
            false
        }
        KeyCode::Backspace => {
            if state.selected_node.is_some() {
                state.zoom_to_parent();
            }
            false
        }
        KeyCode::Char('/') => {
            state.search_query.clear();
            state.search_query.push('/');
            false
        }
        KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.search_query.clear();
            state.search_query.push('/');
            false
        }
        _ => false,
    }
}

fn handle_search_input(key: KeyEvent, state: &mut crate::core::world::WorldState) -> bool {
    match key.code {
        KeyCode::Esc => {
            state.search_query.clear();
            state.search_results.clear();
            false
        }
        KeyCode::Enter => {
            if !state.search_results.is_empty() {
                state.navigate_to_search_result(0);
            }
            state.search_query.clear();
            false
        }
        KeyCode::Tab => {
            if !state.search_results.is_empty() {
                let next = if let Some(sel) = state.selected_node {
                    state.search_results.iter().position(|&id| id == sel)
                        .map(|i| (i + 1) % state.search_results.len())
                        .unwrap_or(0)
                } else {
                    0
                };
                state.navigate_to_search_result(next);
            }
            false
        }
        KeyCode::Backspace => {
            state.search_query.pop();
            if state.search_query.len() <= 1 {
                state.search_query.clear();
                state.search_results.clear();
            } else {
                let query = state.search_query[1..].to_string();
                state.search(&query);
            }
            false
        }
        KeyCode::Char(c) => {
            state.search_query.push(c);
            let query = state.search_query[1..].to_string();
            state.search(&query);
            false
        }
        _ => false,
    }
}

pub fn handle_mouse_event(mouse: MouseEvent, state: &mut crate::core::world::WorldState) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            state.last_mouse_pos = Some((mouse.column, mouse.row));

            let screen_pos = glam::Vec2::new(mouse.column as f32, mouse.row as f32);
            let world_pos = state.camera.position + (screen_pos - glam::Vec2::new(60.0, 20.0)) / state.camera.zoom;

            let mut closest_id = None;
            let mut closest_dist = f32::MAX;

            for idx in state.graph.graph.node_indices() {
                if let Some(node) = state.graph.get_node(idx) {
                    let dist = (node.position - world_pos).length();
                    if dist < closest_dist && dist < 2.0 {
                        closest_dist = dist;
                        closest_id = Some(node.id);
                    }
                }
            }

            if let Some(id) = closest_id {
                state.select_node(id);
            } else {
                state.deselect();
            }
        }
        MouseEventKind::Down(MouseButton::Right) => {
            state.zoom_into_selected();
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            let current = (mouse.column, mouse.row);
            if let Some(last) = state.last_mouse_pos {
                let dx = current.0 as f32 - last.0 as f32;
                let dy = current.1 as f32 - last.1 as f32;
                let delta = glam::Vec2::new(-dx, -dy) / state.camera.zoom;
                state.pan(delta);
            }
            state.last_mouse_pos = Some(current);
        }
        MouseEventKind::Up(MouseButton::Left) => {
            state.last_mouse_pos = None;
        }
        MouseEventKind::ScrollUp => {
            state.zoom_in();
        }
        MouseEventKind::ScrollDown => {
            state.zoom_out();
        }
        _ => {}
    }
}
