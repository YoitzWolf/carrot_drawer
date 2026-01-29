use std::sync::{Arc, Mutex};
// use wgpu::VertexStepMode::Vertex;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;
use crate::core::*;
use crate::core::vis_geometry::contour::{BasicContour, Contour};
use crate::core::vis_geometry::triangulation::triangulate_2d;
use crate::core::vis_geometry::vertex::Vertex;

pub struct App {
    state: Option<State>,
    // draw_vertices: Vec<Vertex<3>>,
    // draw_indexes:  Vec<u32>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        
        Self {
            state: None,
        }
    }
}


impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let draw_vertices = BasicContour::NPolygon(59).to_vertex_list().first().unwrap().clone();
        let draw_indexes = triangulate_2d(&draw_vertices).unwrap(); //vec![
        let draw_vertices = draw_vertices.iter().map(
            |v| {
                Vertex {
                    position: [v.x, v.y, v.z],
                    color: [v.x, v.y, 1.0],
                }
            }
        ).collect();
        // If we are not on web we can use pollster to
        // await the
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let s = runtime.block_on(
            State::new(
                window,
                &draw_vertices,
                &draw_indexes,
            )
        );
        self.state = Some(
            s.unwrap() // State::new(window)
        );
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let render_state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(
                size) => render_state.resize(PhysicalSize::new(size.width, size.height)
            ),
            WindowEvent::RedrawRequested => {
                render_state.render();
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    ..
                },
                ..
            } => match (code, state.is_pressed()) {
                (KeyCode::Escape, true) => event_loop.exit(),
                _ => {
                    let draw_vertices = BasicContour::NPolygon(59).to_vertex_list().first().unwrap().clone();
                    let draw_indexes = triangulate_2d(&draw_vertices).unwrap(); //vec![
                    let draw_vertices = draw_vertices.iter().map(
                        |v| {
                            Vertex {
                                position: [v.x, v.y, v.z],
                                color: [1.0, 1.0, 1.0],
                            }
                        }
                    ).collect();
                    render_state.update_render_buffer(
                        &draw_vertices, &draw_indexes
                    );
                    render_state.render();
                }
            },
            _ => {}
        }
    }
}
