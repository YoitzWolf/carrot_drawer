use std::fmt::Debug;
use glam::{Mat3A, Vec3};

// pub enum CrossSectionSolver {
//     Sum,
//     Sub,
//     Xor,
// }



pub trait Contour where Self: Debug + Send + Sync {
    fn to_vertex_list(&self) -> Vec<Vec<Vec3>>;
    fn box_clone(&self) -> Box<dyn Contour>;
    fn rotate(&mut self, angle: f32);

}

#[derive(Debug, Clone)]
pub enum BasicContourShape {
    /// square 1*1
    Square,
    /// rectangle with width=1 and height=param
    Rectangle(f32),
    NPolygon(usize),
    // /// triangle with base with width=1 and left corner angle param0 and left segment length param1
    Triangle(f32, f32),
    // Sector(f32),
    // Complex(Vec<BasicContour>),
}

#[derive(Debug, Clone)]
pub struct BasicContour {
    shape: BasicContourShape,
    rotation: f32
}

impl BasicContour {
    pub fn new(shape: BasicContourShape, rotation: f32) -> Self {
        Self{shape, rotation}
    }
}

impl Contour for BasicContour {

    fn to_vertex_list(&self) -> Vec<Vec<Vec3>> {
        let m = Mat3A::from_angle(self.rotation);
        match self.shape {
            BasicContourShape::Square => {
                vec![
                    vec![
                        m*Vec3::new(-1.0, -1.0, 0.0),
                        m*Vec3::new(1.0, -1.0, 0.0),
                        m*Vec3::new(1.0, 1.0, 0.0),
                        m*Vec3::new(-1.0, 1.0, 0.0),
                    ]
                ]
            },
            BasicContourShape::Rectangle(height) => {
                let height = height/2.0;
                vec![
                    vec![
                        m*Vec3::new(-1.0, -height, 0.0),
                        m*Vec3::new(1.0, -height, 0.0),
                        m*Vec3::new(1.0, height, 0.0),
                        m*Vec3::new(-1.0, height, 0.0),
                    ]
                ]
            },
            BasicContourShape::NPolygon(n) => {
                let p = 2.0 * std::f32::consts::PI / (n as f32);
                vec![
                    (0..n).map(
                        |i| {
                            // Vec3::new(0.5, 0.5, 0.0) +
                                m*Vec3::new((p*i as f32).cos(), (p*i as f32).sin(), 0.0)
                        }
                    ).collect()
                ]
            },
            BasicContourShape::Triangle(a, b) => {
                vec![vec![
                    m*Vec3::new(1.0, 0.0, 0.0),
                    m*Vec3::new(a.cos(), a.sin(), 0.0),
                    m*Vec3::new(b.sin(), b.cos(), 0.0),
                ]]
            },
            // BasicContour::Sector(_) => {
            //     todo!()
            // },
            // BasicContour::Complex(_) => {
            //     todo!()
            // },
        }
    }
    fn box_clone(&self) -> Box<(dyn Contour + 'static)> {
        Box::new(self.clone())
    }

    fn rotate(&mut self, angle: f32) {
        self.rotation = angle;
    }
}