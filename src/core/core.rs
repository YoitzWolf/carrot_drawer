use std::sync::Arc;
use wgpu::BindingType::Texture;
use wgpu::{TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureViewDimension};
use winit::{
    window::Window,
    dpi::PhysicalSize
};

use wgpu::util::DeviceExt;

use crate::core::camera::*;
use crate::core::vis_geometry::vertex::*;

const VERTICES: &[Vertex<3>] = &[
    Vertex { position: [0.0, 0.5,   0.1], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.5], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5,  0.5], color: [0.0, 0.0, 1.0] },


    Vertex { position: [0.0, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [0.5, 0.5,  0.5], color:  [0.0, 1.0, 1.0] },
    Vertex { position: [-0.5, 0.5, 0.5], color: [1.0, 0.0, 1.0] },
];

// This will store the state of our game
pub struct State {
    size: PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_format: wgpu::TextureFormat,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    camera_state: CameraState,
    camera_buffer: wgpu::Buffer, // uniform
    camera_bind_group: wgpu::BindGroup,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY, // autoselection
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;

        let surface = instance.create_surface(window.clone())?;
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            }).await?;
        println!("Device Features: {:?}", device.features());
        let shader = device.create_shader_module(
            wgpu::include_wgsl!("shaders/test_shader.wgsl")
        );
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let mut camera_state = CameraState::new();
        camera_state.set_scaling(&size);
        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_state]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                ],
                immediate_size: 0,
            });
        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor{
                label: Some("Test Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[
                        Vertex::<3>::desc(),
                    ],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less, // 1.
                    stencil: wgpu::StencilState::default(), // 2.
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment:  Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                multiview_mask: None,
                cache: None,
            }
        );
        let depth_texture = device.create_texture(
            &TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width: size.width.max(1),
                    height: size.height.max(1),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }
        );
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        // -------------------------------------------------------------
        let state = Self {
            size,
            surface,
            device,
            queue,
            surface_format,
            render_pipeline,
            vertex_buffer,
            camera_state,
            camera_buffer,
            camera_bind_group,
            depth_texture,
            depth_view,
            window,
        };
        state.configure_surface();
        Ok(state)
    }
    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn update_state(&mut self) {
        self.camera_state.set_scaling(&self.size);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_state]));
    }

    pub fn resize(&mut self, mut new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width <= 0 {
            new_size.width = 1; // to not crush
        }
        if new_size.height <= 0 {
            new_size.height = 1;  // to not crush
        }
        self.size = new_size;
        self.configure_surface();
        self.update_state();
        self.depth_texture = self.device.create_texture(
            &TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width: self.size.width.max(1),
                    height: self.size.height.max(1),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }
        );
        self.depth_view = self.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    }
    
    pub fn render(&mut self) {
        // self.window.request_redraw();
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });
        // Renders a screen
        let mut encoder = self.device.create_command_encoder(&Default::default());
        // Create the renderpass which will clear the screen.
        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        // drawing commands
        renderpass.set_pipeline(&self.render_pipeline);
        renderpass.set_bind_group(0, &self.camera_bind_group, &[]);
        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        renderpass.draw(0..VERTICES.len() as u32, 0..1);
        // End the renderpass.
        drop(renderpass);
        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}
