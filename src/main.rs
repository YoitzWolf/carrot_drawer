use std::f32::consts::PI;
use std::time::Duration;
use glam::{Mat3A, Mat4, Vec2, Vec3};
use log::info;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopClosed, EventLoopProxy};
use tokio;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

mod app_setup;
use app_setup::*;

mod core;
use core::*;
use crate::core::vis_geometry::flat_field::{FlatField, FlatFieldPoint};
use crate::core::vis_geometry::gentraits::{AffineTransformable, Colorable};
use crate::core::vis_geometry::render_object::RenderObject;
use crate::core::graphics2d::layer::Layer;
use crate::core::vis_geometry::contour::{BasicContour, BasicContourShape};
use crate::core::vis_geometry::render_object::ContourRender;
use crate::core::vis_geometry::Vertex;

fn runner(mut proxy: EventLoopProxy<StateUpdate>) {
    tokio::task::spawn(async move {
        let mut r = 0;
        let mut layer = Layer::new(Some("base layer".to_string()));
        layer.push(0, Box::new(ContourRender{
            contour: Box::new(BasicContour::new(BasicContourShape::NPolygon(3), Mat4::IDENTITY)),
            colors: vec![Vec3::new(1.0, 0.0, 0.0); 3],
        }));
        layer.push(1, Box::new(ContourRender{
            contour: Box::new(BasicContour::new(BasicContourShape::NPolygon(3), Mat4::IDENTITY)),
            colors: vec![Vec3::new(1.0, 0.0, 0.0); 3],
        }));
        let h= 0.5f32;
        let shift = -2.5f32;
        let N = 20;
        layer.push(2, Box::new(FlatField{
            field: {
                (0..N).map(
                    |i| {
                        (0..N).map(
                            |j| {
                                FlatFieldPoint::new(
                                    Vec3::new(
                                        h*j as f32 + shift,
                                        h*i as f32 + shift,
                                        0.0
                                    ),
                                Vec3::new(
                                    ((i as f32).powi(2) + (j as f32).powi(2) - 16.),
                                    ((i as f32 - 5.5).powi(2) + (j as f32 - 5.5).powi(2) - 25.),
                                    1.0
                                ))
                            }
                        ).collect::<Vec::<_>>()
                    }
                ).collect::<Vec::<_>>()
            },
        }));
        let K = 1024;
        tokio::time::sleep(Duration::from_millis(10)).await;
        loop {
            tokio::time::sleep(Duration::from_millis(0)).await;
            {
                let v = layer.get_mut(2).unwrap();
                v.set_colors(
                    (0..N*N).map(
                        |x| {
                            Vec3::new(
                                ( -(x % N % 2) as f32 * r as f32 / K as f32 * 2.0 * PI).cos() / 2.0 + 0.5,
                                ( (x / N % 2) as f32  * r as f32 / K as f32 * 2.0 * PI).sin() / 2.0 + 0.5,
                                ( (x % 2) as f32 * r as f32 / K as f32 * 2.0 * PI).sin() / 2.0 + 0.5
                            )
                        }
                    ).collect()
                );
            }
            {
                let v = layer.get_mut(0).unwrap();
                v.set_transform(
                    Mat4::from_scale(
                        Vec3::new(
                            (r as f32 / K as f32 * 2.0 * PI).sin() / 2.0 + 0.5,
                            (r as f32 / K as f32 * 2.0 * PI).sin() / 2.0 + 0.5,
                            1.0
                        )
                    ) * Mat4::from_rotation_z(r as f32 / K as f32 * 2.0 * PI)
                );
                v.set_colors(
                    vec![
                        Vec3::new(f32::sin(r as f32 / K as f32 * PI * 2.0)/2.0 + 0.5, 0.0, 0.0),
                        Vec3::new(0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + 2.0*PI/3.0)/2.0 + 0.5, 0.0),
                        Vec3::new(0.0, 0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + 4.0*PI/3.0)/2.0 + 0.5),
                    ]
                );

                let v = layer.get_mut(1).unwrap();
                v.set_transform(
                    Mat4::from_scale(
                        Vec3::new(
                            (r as f32 / K as f32 * 2.0 * PI + PI).sin() / 2.0 + 0.5,
                            (r as f32 / K as f32 * 2.0 * PI + PI).sin() / 2.0 + 0.5,
                            1.0
                        )
                    ) * Mat4::from_rotation_z(r as f32 / K as f32 * 2.0 * PI + PI / 3.0)
                );
                v.set_colors(
                    vec![
                        Vec3::new(f32::sin(r as f32 / K as f32 * PI * 2.0)/2.0 + 0.5, 0.0, 0.0),
                        Vec3::new(0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + 2.0*PI/3.0)/2.0 + 0.5, 0.0),
                        Vec3::new(0.0, 0.0, f32::sin(r as f32 / K as f32 * PI * 2.0 + 4.0*PI/3.0)/2.0 + 0.5),
                    ]
                )
            }
            let (vertices, indexes) = layer.render().unwrap();
            match proxy.send_event(
                StateUpdate::ResetVertices {
                        vertices,
                        indexes,
                }
            ) {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to update render buffer");
                    break; }
            };
            r = (r + 1) % K;
        }
    });
}

pub async fn run() -> anyhow::Result<()> {
    env_logger::init();
    info!("Starting...");
    let mut event_loop = EventLoop::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut proxy = event_loop.create_proxy();
    let mut app = App::new();
    info!("Initialisation finished, starting app...");
    runner(proxy);
    info!("Loop execution starts...");
    event_loop.run_app(&mut app)?;
    info!("Exiting...");
    Ok(())
}


#[tokio::main]
async fn main() {
    run().await.unwrap();
}