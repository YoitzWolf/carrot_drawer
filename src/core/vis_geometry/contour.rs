use glam::Vec3;
use crate::core::vis_geometry::Vertex;

// pub type PolygonList<const N: usize> = (Vec<Vertex<N>>, Vec<u32>);

pub enum CrossSectionSolver {
    Sum,
    Sub,
    Xor,

}

pub trait Contour {
    fn to_vertex_list(&self) -> Vec<Vec<Vec3>>;
}

pub enum BasicContour {
    /// square 1*1
    Square,
    /// rectangle with width=1 and height=param
    Rectangle(f32),
    NPolygon(usize),
    // /// triangle with base with width=1 and left corner angle param0 and left segment length param1
    // Triangle(f32, f32),
    // Sector(f32),
    // Complex(Vec<BasicContour>),
}

impl Contour for BasicContour {
    fn to_vertex_list(&self) -> Vec<Vec<Vec3>> {
        match self {
            BasicContour::Square => {
                vec![
                    vec![
                        Vec3::new(-1.0, -1.0, 0.0),
                        Vec3::new(1.0, -1.0, 0.0),
                        Vec3::new(1.0, 1.0, 0.0),
                        Vec3::new(-1.0, 1.0, 0.0),
                    ]
                ]
            },
            BasicContour::Rectangle(height) => {
                let height = *height/2.0;
                vec![
                    vec![
                        Vec3::new(-1.0, -height, 0.0),
                        Vec3::new(1.0, -height, 0.0),
                        Vec3::new(1.0, height, 0.0),
                        Vec3::new(-1.0, height, 0.0),
                    ]
                ]
            },
            &BasicContour::NPolygon(n) => {
                let p = 2.0 * std::f32::consts::PI / (n as f32);
                vec![
                    (0..n).map(
                        |i| {
                            // Vec3::new(0.5, 0.5, 0.0) +
                                Vec3::new((p*i as f32).cos(), (p*i as f32).sin(), 0.0)
                        }
                    ).collect()
                ]
            },
            // BasicContour::Triangle(_, _) => {
            //     todo!()
            // },
            // BasicContour::Sector(_) => {
            //     todo!()
            // },
            // BasicContour::Complex(_) => {
            //     todo!()
            // },
        }
    }
}