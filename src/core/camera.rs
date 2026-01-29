use winit::dpi::PhysicalSize;

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraState {
    // pos_x: f32,
    // pos_y: f32,
    window_scaling: [f32; 2],
    zoom: f32,
    __p: f32
}

impl CameraState {
    pub fn new() -> Self {
        Self {
            window_scaling: [1.0, 1.0],
            zoom: 100.0,
            __p: 0.0,
        }
    }

    pub fn set_scaling(&mut self, size: &PhysicalSize<u32>) {
        self.window_scaling = [(2.0 / size.width as f32), (2.0 / size.height as f32)];
        // self.window_scaling = {
        //     if (size.height > size.width) {
        //         [1.0, (size.width as f32 / size.height as f32)]
        //     } else {
        //         [(size.height as f32 / size.width as f32), 1.0]
        //     }
        // }
    }
}
