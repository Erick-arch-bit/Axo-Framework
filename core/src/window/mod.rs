use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use crate::renderer::{Rect, Renderer};

pub type ClickHandler = Arc<dyn Fn(u64, &str, f64, f64) + Send + Sync>;

pub struct AppState {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    rects: Arc<Mutex<Vec<Rect>>>,
    mouse_pos: (f64, f64),
    on_click: Option<ClickHandler>,
}

impl AppState {
    fn hit_test(&self, x: f64, y: f64) -> Option<(usize, u64, String)> {
        let rects = self.rects.lock().unwrap();
        for (i, rect) in rects.iter().enumerate().rev() {
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

    fn handle_click(&self, button: MouseButton, state: ElementState) {
        if button == MouseButton::Left && state == ElementState::Pressed {
            if let Some((idx, id, cb_id)) = self.hit_test(self.mouse_pos.0, self.mouse_pos.1) {
                let rects = self.rects.lock().unwrap();
                if idx < rects.len() {
                    let rect = &rects[idx];
                    println!("[Input] Clicked '{}' (id={}) cb='{}' at ({:.0}, {:.0})",
                        rect.node_type, id, cb_id, rect.x, rect.y);
                    if !cb_id.is_empty() {
                        if let Some(ref handler) = self.on_click {
                            handler(id, &cb_id, self.mouse_pos.0, self.mouse_pos.1);
                        }
                    }
                }
            }
        }
    }
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes()
            .with_title("Lumina Framework")
            .with_inner_size(PhysicalSize::new(1024, 768));

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        let renderer = pollster::block_on(Renderer::new(Arc::clone(&window)));

        self.window = Some(window);
        self.renderer = Some(renderer);
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
            }
            WindowEvent::CursorMoved { position: PhysicalPosition { x, y }, .. } => {
                self.mouse_pos = (x, y);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.handle_click(button, state);
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    let rects = self.rects.lock().unwrap().clone();
                    match renderer.render(&rects) {
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
    run_with_shared_rects_and_handler(rects, None);
}

pub fn run_with_shared_rects_and_handler(rects: Arc<Mutex<Vec<Rect>>>, on_click: Option<ClickHandler>) {
    let event_loop = EventLoop::new().unwrap();
    let mut app = AppState {
        window: None,
        renderer: None,
        rects,
        mouse_pos: (0.0, 0.0),
        on_click,
    };
    event_loop.run_app(&mut app).unwrap();
}
