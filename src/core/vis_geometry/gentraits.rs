use glam::{Affine2, Mat3A, Mat4, Vec3};


pub trait BoxClone {
    fn box_clone<T>(&self) -> Box<T>;
}

pub trait AffineTransformable {
    fn set_transform(&mut self, matrix: Mat4);

    fn apply_transform(&mut self, matrix: &Mat4);

    fn rotate(&mut self, angle: f32);
}

pub trait Colorable {
    fn set_colors(&mut self, colors: Vec<Vec3>);
}