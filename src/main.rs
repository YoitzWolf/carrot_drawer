use std::f32::consts::PI;
use std::time::Duration;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopClosed, EventLoopProxy};
use tokio;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

mod app_setup;
use app_setup::*;

mod core;
use core::*;
use crate::core::vis_geometry::Vertex;

fn runner(mut proxy: EventLoopProxy<StateUpdate>) {
    tokio::task::spawn(async move {
        let mut r = 0;
        loop {
            match proxy.send_event(StateUpdate::RawRedraw) {
                Ok(_) => {
                    println!("Start poll");
                    break;
                }
                Err(_) => {}
            }
        }
        let K = 512;
        loop {
            // println!("r: {}", r);
            tokio::time::sleep(Duration::from_millis(1)).await;
            match proxy.send_event(
                StateUpdate::ResetVertices {
                    vertices: vec![
                        Vertex{
                            position: [
                                f32::sin(r as f32 / K as f32 * PI * 2.0 ),
                                f32::cos(r as f32 / K as f32 * PI * 2.0 ),
                                0.0
                            ],
                            color: [f32::sin(r as f32 / K as f32 * PI * 2.0)/2.0 + 0.5, 0.0, 0.0],
                        },
                        Vertex{
                            position: [
                                f32::sin(r as f32 / K as f32 * PI * 2.0 + 2.0*PI/3.0),
                                f32::cos(r as f32 / K as f32 * PI * 2.0 + 2.0*PI/3.0),
                                0.0
                            ],
                            color: [0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + PI/2.0)/2.0 + 0.5, 0.0],
                        },
                        Vertex{
                            position: [
                                f32::sin(r as f32 / K as f32 * PI * 2.0 + 4.0*PI/3.0),
                                f32::cos(r as f32 / K as f32 * PI * 2.0 + 4.0*PI/3.0),
                                0.0
                            ],
                            color: [0.0, 0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + PI)/2.0 + 0.5],
                        },
                        Vertex{
                            position: [0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + PI/2.0), 0.0],
                            color: [0.0, 0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + PI)/2.0 + 0.5],
                        },
                        Vertex{
                            position: [0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + PI/2.0), 0.0],
                            color: [0.0, 0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + PI)/2.0 + 0.5],
                        },
                    ],
                    indexes: vec![0, 2, 1,],
                }
            ) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to update render buffer");
                    break; }
            };

            // match proxy.send_event(StateUpdate::RawRedraw) {
            //     Ok(_) => {}
            //     Err(_) => { break; }
            // }
            r = (r + 1) % K;
        }
    });
}

pub async fn run() -> anyhow::Result<()> {
    env_logger::init();
    let mut event_loop = EventLoop::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut proxy = event_loop.create_proxy();
    let mut app = App::new();
    runner(proxy);
    event_loop.run_app(&mut app)?;
    Ok(())
}


#[tokio::main]
async fn main() {
    run().await.unwrap();
}