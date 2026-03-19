use glam::Vec3;


pub trait BoxClone {
    fn box_clone<T>(&self) -> Box<T>;
}

pub trait Rotated {
    fn set_rotation(&mut self, angle: f32);
    fn rotate(&mut self, angle: f32);
}

pub trait Colorable {
    fn set_colors(&mut self, colors: Vec<Vec3>);
}