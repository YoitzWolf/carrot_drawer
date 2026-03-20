use std::fmt::Debug;
use glam::{Affine2, Mat3, Mat3A, Vec3};
use crate::core::vis_geometry::gentraits::AffineTransformable;
// pub enum CrossSectionSolver {
//     Sum,
//     Sub,
//     Xor,
// }



pub trait Contour where Self: Debug + Send + Sync + AffineTransformable
{
    fn to_vertex_list(&self) -> Vec<Vec<Vec3>>;
    fn box_clone(&self) -> Box<dyn Contour>;

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
pub struct NaiveContour {
    pub shape: Vec<Vec<Vec3>>,
    pub matrix: Mat3A,
}

impl NaiveContour {
    pub fn new(shape: Vec<Vec<Vec3>>, matrix: Mat3A) -> Self {
        Self { shape, matrix}
    }
}

impl AffineTransformable for NaiveContour {
    fn set_transform(&mut self, matrix: Mat3A) {
        todo!()
    }

    fn apply_transform(&mut self, matrix: &Mat3A) {
        self.matrix = matrix * &self.matrix;
    }

    fn rotate(&mut self, angle: f32) {
        self.apply_transform(&Mat3A::from_angle(angle));
    }
}

impl Contour for NaiveContour {
    fn to_vertex_list(&self) -> Vec<Vec<Vec3>> {
        self.shape.iter().map(|x| x.iter().map(|x| {self.matrix * x}).collect()).collect()
    }

    fn box_clone(&self) -> Box<dyn Contour> {
        Box::new(self.clone())
    }
}


#[derive(Debug, Clone)]
pub struct BasicContour {
    shape: BasicContourShape,
    matrix: Mat3A,
}

impl BasicContour {
    pub fn new(shape: BasicContourShape, matrix: Mat3A) -> Self {
        Self{shape, matrix}
    }
}

impl AffineTransformable for BasicContour {
    fn set_transform(&mut self, matrix: Mat3A) {
        self.matrix = matrix;
    }

    fn apply_transform(&mut self, matrix: &Mat3A) {
        self.matrix = matrix * &self.matrix;
    }

    fn rotate(&mut self, angle: f32) {
        self.apply_transform(&Mat3A::from_angle(angle));
    }
}

impl Contour for BasicContour {

    fn to_vertex_list(&self) -> Vec<Vec<Vec3>> {
        let m = &self.matrix;
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
}