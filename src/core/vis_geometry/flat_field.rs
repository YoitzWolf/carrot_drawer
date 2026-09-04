use std::fmt::Debug;
use glam::{Mat4, Vec3};
use crate::core::vis_geometry::gentraits::{AffineTransformable, Colorable};
use crate::core::vis_geometry::render_object::RenderObject;
use crate::core::vis_geometry::Vertex;

#[derive(Debug, Clone)]
pub struct FlatFieldPoint<P: Debug + Clone> {
    cords: Vec3,
    value: P
}

impl<P: Debug + Clone> FlatFieldPoint<P> {
    pub fn new(cords: Vec3, value: P) -> Self {
        Self { cords, value }
    }
}

impl FlatFieldPoint<Vec3> {
    pub fn color(&self) -> Vec3 {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct FlatField<P>
where P: Debug + Clone + Sync + Send {
    pub field: Vec<Vec< FlatFieldPoint<P> >>,
}

impl<P> FlatField<P>
where P: Debug + Clone + Sync + Send {
    pub fn new(field: Vec<Vec< FlatFieldPoint<P> >>) -> FlatField<P> {
        Self { field }
    }
}

impl<P> AffineTransformable for FlatField<P>
where P: Debug + Clone + Sync + Send {
    fn set_transform(&mut self, matrix: Mat4) {
        unimplemented!()
    }

    fn apply_transform(&mut self, matrix: &Mat4) {
        unimplemented!()
    }

    fn rotate(&mut self, angle: f32) {
        unimplemented!()
    }
}

impl // <P>
RenderObject<3> for FlatField<Vec3> //<P>
where //P: Debug + Clone + Sync + Send,
      Self: Send + Sync + Debug + AffineTransformable {
    fn render(&self) -> anyhow::Result<(Vec<Vertex<3>>, Vec<u32>)> {
        let mut idx = 0u32;
        let mut indeces = vec![];
        let mut vertices = vec![];
        let N = self.field.len() as u32;
        for i in 0..self.field.len() as u32 {
            for j in 0..self.field[i as usize].len() as u32 {
                if i < self.field.len() as u32 - 1 && j < self.field[i as usize].len() as u32 - 1 {
                    indeces.append(&mut vec![
                        idx+1, idx + N, idx,
                        idx + N, idx + 1, idx + N+1
                    ]);
                }
                let v = &self.field[i as usize][j as usize];
                vertices.push(
                    Vertex::<3> {
                        position: v.cords.to_array(),
                        color: v.color().to_array(),
                    }
                );
                idx += 1;
            }
        }
        Ok((vertices, indeces))
    }
}

impl // <P>
Colorable for FlatField<Vec3> //<P>
where //P: Debug + Clone + Sync + Send,
      Self: Send + Sync + Debug + AffineTransformable{
    fn set_colors(&mut self, colors: Vec<Vec3>) {
        let mut idx = 0;
        for i in 0..self.field.len() {
            for j in 0..self.field[i as usize].len() {
                self.field[i][j].value = colors[idx];
                idx += 1;
            }
        }
    }
}
