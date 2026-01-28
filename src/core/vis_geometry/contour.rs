use crate::core::vis_geometry::Vertex;

pub type PolygonList<const N: usize> = (Vec<Vertex<N>>, Vec<u32>);

pub enum ContourMode {
    Fill,
    Contour(f32),
}

pub trait Contour {
    fn to_polygon_list(&self, offset: u32, contour_mode: &ContourMode) -> PolygonList<3>;
}

