use std::sync::Arc;
use tokio::runtime::Handle;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceId, KeyEvent, MouseScrollDelta, TouchPhase, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
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

pub struct StateUpdate {}

impl ApplicationHandler<StateUpdate> for App {
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
        // let runtime = tokio::runtime::Builder::new_multi_thread()
        //     .enable_all()
        //     .build()
        //     .unwrap();
        let s = tokio::task::block_in_place(move || {
            Handle::current().block_on(async {
                State::new(
                    window,
                    &draw_vertices,
                    &draw_indexes,
                ).await
            })
        });
        self.state = Some(
            s.unwrap()
        );
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: StateUpdate) {
        // self.state = Some(event);
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
                // println!("REDRAW");
                render_state.render();
            },
            WindowEvent::MouseWheel {
                device_id: _,
                delta: MouseScrollDelta::LineDelta(dx, dy),
                phase: _
            } => {
                render_state.change_special_zoom(dy);
                println!("Zoom: dy {} and special {}", dy, render_state.get_special_zoom());
                render_state.render();
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    ..
                },
                ..
            } => match (code, state.is_pressed()) {
                (KeyCode::Escape, false) => event_loop.exit(),
                (KeyCode::ArrowLeft, true) => {
                    println!("ArrowLeft");
                    render_state.move_camera(-0.2, 0.0);
                    render_state.render();
                },
                (KeyCode::ArrowRight, true) => {
                    println!("ArrowRight");
                    render_state.move_camera(0.2, 0.0);
                    render_state.render();
                },
                (KeyCode::ArrowUp, true) => {
                    println!("ArrowLeft");
                    render_state.move_camera(0.0, 0.2);
                    render_state.render();
                },
                (KeyCode::ArrowDown, true) => {
                    println!("ArrowRight");
                    render_state.move_camera(0.0, -0.2);
                    render_state.render();
                },
                _ => {
                    // let draw_vertices = BasicContour::NPolygon(59).to_vertex_list().first().unwrap().clone();
                    // let draw_indexes = triangulate_2d(&draw_vertices).unwrap(); //vec![
                    // let draw_vertices = draw_vertices.iter().map(
                    //     |v| {
                    //         Vertex {
                    //             position: [v.x, v.y, v.z],
                    //             color: [1.0, 1.0, 1.0],
                    //         }
                    //     }
                    // ).collect();
                    // render_state.update_render_buffer(
                    //     &draw_vertices, &draw_indexes
                    // );
                    // render_state.render();
                }
            },
            _ => {}
        }
    }
}
