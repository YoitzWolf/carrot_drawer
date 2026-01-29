use std::sync::Arc;
// use wgpu::VertexStepMode::Vertex;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::Window;
use crate::core::*;
use crate::core::vis_geometry::contour::{BasicContour, Contour};
use crate::core::vis_geometry::triangulation::triangulate_2d;
use crate::core::vis_geometry::vertex::Vertex;

pub struct App {
    state: Option<State>,
    draw_vertices: Arc<Vec<Vertex<3>>>,
    draw_indexes:  Arc<Vec<u32>>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        let draw_vertices = BasicContour::NPolygon(59).to_vertex_list().first().unwrap().clone();
        // [4, 2, 3, 5, 2, 4, 1, 2, 5, 0, 1, 5, 6, 0, 1, 11, 0, 6, 7, 11, 6, 8, 11, 7, 10, 11, 8, 9, 10, 8]
        let draw_indexes = triangulate_2d(&draw_vertices).unwrap(); //vec![
            // 4, 2, 3,
            // 5, 2, 4,
            // 1, 2, 5,
            //0, 1, 5, // !
            //6, 0, 1, // !
            // 11, 0, 6,
            // 7, 11, 6,
            // 8, 11, 7,
            // 10, 11, 8,
            // 9, 10, 8
        //]; //[3..].to_vec();
        // println!("draw vertices: {:?}", draw_vertices);
        // println!("draw indexes: {:?}", draw_indexes);
        Self {
            state: None,
            draw_vertices: Arc::new(draw_vertices.iter().map(
                |v| {
                    Vertex {
                        position: [v.x, v.y, v.z],
                        color: [v.x, v.y, 1.0],
                    }
                }
            ).collect()),
            draw_indexes: Arc::new(draw_indexes),
        }
    }
}


impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // If we are not on web we can use pollster to
        // await the
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let s = runtime.block_on(
            State::new(
                window,
                self.draw_vertices.clone(),
                self.draw_indexes.clone(),
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
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(PhysicalSize::new(size.width, size.height)),
            WindowEvent::RedrawRequested => {
                state.render();
            }
            WindowEvent::KeyboardInput {
                event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    ..
                },
                ..
            } => match (code, state.is_pressed()) {
                // (KeyCode::Escape, true) => event_loop.exit(),
                _ => {}
            },
            _ => {}
        }
    }
}
