use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

use crate::renderer::{Rect, Renderer};

pub type ClickHandler = Arc<dyn Fn(u64, &str, f64, f64) + Send + Sync>;
pub type RebuildFn = Arc<dyn Fn(f32, f32) -> Vec<Rect> + Send + Sync>;

pub struct AppState {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    rects: Arc<Mutex<Vec<Rect>>>,
    mouse_pos: (f64, f64),
    on_click: Option<ClickHandler>,
    rebuild: Option<RebuildFn>,
    viewport: (f32, f32),
    text_focus_id: Option<u64>,
    text_input_buffer: String,
    scroll_state: std::collections::HashMap<u64, (f32, f32)>,
    hovered_id: Option<u64>,
    active_id: Option<u64>,
    mouse_down: bool,
}

impl AppState {
    fn hit_test(&self, x: f64, y: f64) -> Option<(usize, u64, String)> {
        let rects = self.rects.lock().unwrap();
        for (i, rect) in rects.iter().enumerate().rev() {
            if rect.disabled { continue; }
            if x >= rect.x as f64
                && x <= (rect.x + rect.w) as f64
                && y >= rect.y as f64
                && y <= (rect.y + rect.h) as f64
            {
                return Some((i, rect.id, rect.on_click_id.clone()));
            }
        }
        None
    }

    fn hit_test_id(&self, x: f64, y: f64) -> Option<u64> {
        let rects = self.rects.lock().unwrap();
        for rect in rects.iter().rev() {
            if x >= rect.x as f64
                && x <= (rect.x + rect.w) as f64
                && y >= rect.y as f64
                && y <= (rect.y + rect.h) as f64
            {
                return Some(rect.id);
            }
        }
        None
    }

    fn handle_click(&mut self, button: MouseButton, state: ElementState) {
        if button != MouseButton::Left { return; }

        match state {
            ElementState::Pressed => {
                self.mouse_down = true;
                self.active_id = self.hit_test_id(self.mouse_pos.0, self.mouse_pos.1);

                if let Some((idx, id, cb_id)) = self.hit_test(self.mouse_pos.0, self.mouse_pos.1) {
                    let rects_lock = self.rects.lock().unwrap();
                    if idx < rects_lock.len() {
                        let rect = &rects_lock[idx];
                        println!("[Input] Clicked '{}' (id={}) cb='{}' at ({:.0}, {:.0})",
                            rect.node_type, id, cb_id, rect.x, rect.y);

                        if rect.node_type == "TextInput" {
                            self.text_focus_id = Some(rect.id);
                            self.text_input_buffer = rect.text_content.clone();
                        } else {
                            self.text_focus_id = None;
                        }
                    }
                } else {
                    self.text_focus_id = None;
                }
            }
            ElementState::Released => {
                self.mouse_down = false;
                // Fire click event if released on same element
                if let Some(active_id) = self.active_id {
                    if let Some((_, _, cb_id)) = self.hit_test(self.mouse_pos.0, self.mouse_pos.1) {
                        if !cb_id.is_empty() {
                            if let Some(ref handler) = self.on_click {
                                handler(active_id, &cb_id, self.mouse_pos.0, self.mouse_pos.1);
                            }
                        }
                    }
                }
                self.active_id = None;
            }
        }
    }

    fn handle_keyboard_input(&mut self, key: Key) {
        let focus_id = match self.text_focus_id {
            Some(id) => id,
            None => return,
        };

        match key {
            Key::Named(NamedKey::Enter) => {
                let cb_id = {
                    let rects = self.rects.lock().unwrap();
                    rects.iter()
                        .find(|r| r.id == focus_id)
                        .map(|r| r.on_change_text_id.clone())
                        .unwrap_or_default()
                };
                if !cb_id.is_empty() {
                    if let Some(ref handler) = self.on_click {
                        handler(focus_id, &cb_id, 0.0, 0.0);
                    }
                }
                self.text_focus_id = None;
            }
            Key::Named(NamedKey::Backspace) => {
                self.text_input_buffer.pop();
                self.update_text_content(focus_id);
            }
            Key::Character(ref ch) if !ch.is_empty() => {
                self.text_input_buffer.push_str(ch);
                self.update_text_content(focus_id);
            }
            _ => {}
        }
    }

    fn update_text_content(&self, id: u64) {
        let mut rects = self.rects.lock().unwrap();
        if let Some(rect) = rects.iter_mut().find(|r| r.id == id) {
            rect.text_content = self.text_input_buffer.clone();
        }
    }

    fn handle_mouse_wheel(&mut self, delta: MouseScrollDelta) {
        let scroll_id = self.hovered_id.and_then(|id| {
            let rects = self.rects.lock().unwrap();
            rects.iter().find(|r| r.id == id)
                .filter(|r| r.node_type == "ScrollView")
                .map(|r| r.id)
        });

        if let Some(id) = scroll_id {
            let delta_y = match delta {
                MouseScrollDelta::LineDelta(_, y) => y * 20.0,
                MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
            };
            let entry = self.scroll_state.entry(id).or_insert((0.0, 0.0));
            entry.1 += delta_y;
            entry.1 = entry.1.clamp(-2000.0, 0.0);
            self.apply_scroll_offsets();
        }
    }

    fn apply_scroll_offsets(&self) {
        let mut rects = self.rects.lock().unwrap();
        for rect in rects.iter_mut() {
            if let Some(&(ox, oy)) = self.scroll_state.get(&rect.id) {
                rect.scroll_offset_x = ox;
                rect.scroll_offset_y = oy;
            }
        }
    }

    fn request_rebuild(&mut self) {
        let (w, h) = self.viewport;
        if let Some(ref rebuild_fn) = self.rebuild {
            let new_rects = rebuild_fn(w, h);
            *self.rects.lock().unwrap() = new_rects;
        }
    }
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes()
            .with_title("Axo Framework")
            .with_inner_size(PhysicalSize::new(1024, 768));

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        let size = window.inner_size();
        self.viewport = (size.width as f32, size.height as f32);
        let renderer = pollster::block_on(Renderer::new(Arc::clone(&window)));

        self.window = Some(window);
        self.renderer = Some(renderer);

        self.request_rebuild();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height);
                }
                self.viewport = (size.width as f32, size.height as f32);
                self.request_rebuild();
            }
            WindowEvent::CursorMoved { position: PhysicalPosition { x, y }, .. } => {
                self.mouse_pos = (x, y);
                self.hovered_id = self.hit_test_id(x, y);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.handle_click(button, state);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.handle_mouse_wheel(delta);
            }
            WindowEvent::KeyboardInput { event: ke, .. }
                if ke.state == winit::event::ElementState::Pressed => {
                    self.handle_keyboard_input(ke.logical_key);
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    let rects = self.rects.lock().unwrap().clone();
                    match renderer.render(&rects, self.hovered_id, self.active_id) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            if let Some(window) = &self.window {
                                renderer.resize(window.inner_size().width, window.inner_size().height);
                            }
                        }
                        Err(e) => eprintln!("[Renderer] Surface error: {:?}", e),
                    }
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

pub fn run_with_rects(rects: Vec<Rect>) {
    run_with_shared_rects(Arc::new(Mutex::new(rects)));
}

pub fn run_with_shared_rects(rects: Arc<Mutex<Vec<Rect>>>) {
    run_with_shared_rects_and_handler(rects, None, None);
}

pub fn run_with_shared_rects_and_handler(
    rects: Arc<Mutex<Vec<Rect>>>,
    on_click: Option<ClickHandler>,
    rebuild: Option<RebuildFn>,
) {
    let event_loop = EventLoop::new().unwrap();
    let size = PhysicalSize::new(1024, 768);
    let mut app = AppState {
        window: None,
        renderer: None,
        rects,
        mouse_pos: (0.0, 0.0),
        on_click,
        rebuild,
        viewport: (size.width as f32, size.height as f32),
        text_focus_id: None,
        text_input_buffer: String::new(),
        scroll_state: std::collections::HashMap::new(),
        hovered_id: None,
        active_id: None,
        mouse_down: false,
    };
    event_loop.run_app(&mut app).unwrap();
}
