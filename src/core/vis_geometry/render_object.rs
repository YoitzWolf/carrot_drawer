use std::fmt::Debug;
use glam::{Affine2, Affine3A, Mat3A, Vec3};
use crate::core::vis_geometry::{contour, Vertex};
use crate::core::vis_geometry::contour::Contour;
use crate::core::vis_geometry::gentraits::{Colorable, AffineTransformable};
use crate::core::vis_geometry::triangulation::triangulate_2d;

pub trait RenderObject<const N: usize> where Self: Debug + Send + Sync + AffineTransformable
{
    fn render(&self) -> anyhow::Result<(Vec<Vertex<N>>, Vec<u32>)>;
    // fn box_clone(&self) -> Box<dyn RenderObject<N>>;
}

#[derive(Debug)]
pub struct ContourRender {
    pub contour: Box<dyn Contour>,
    pub colors: Vec<Vec3>
}

impl Colorable for ContourRender {
    fn set_colors(&mut self, colors: Vec<Vec3>) {
        self.colors = colors;
    }
}

impl Clone for ContourRender {
    fn clone(&self) -> Self {
        Self {
            contour: self.contour.box_clone(),
            colors: self.colors.clone(),
        }
    }
}

impl AffineTransformable for ContourRender {
    fn rotate(&mut self, angle: f32) {
        self.contour.rotate(angle);
    }

    fn set_transform(&mut self, matrix: Mat3A) {
        self.contour.set_transform(matrix);
    }

    fn apply_transform(&mut self, matrix: &Mat3A) {
        self.contour.apply_transform(matrix);
    }
}

impl RenderObject<3> for ContourRender {
    fn render(&self) -> anyhow::Result<(Vec<Vertex<3>>, Vec<u32>)> {
        self.contour.to_vertex_list().iter_mut().fold(
            Ok((vec![], vec![])),
            |mut r, contour| {
                if let Ok((mut vxs, mut idxs)) = r {
                    let mut idx = triangulate_2d(contour)?;
                    let N = vxs.len() as u32;
                    idx.iter_mut().for_each(|i| *i += N);
                    vxs.append(
                        &mut contour.iter().enumerate().map(|(i, x)| {
                            Vertex::<3> {
                                position: x.to_array(),
                                color: self.colors[i+N as usize].to_array(),
                            }
                        }).collect::<Vec<_>>()
                    );
                    idxs.append(&mut idx);
                    Ok((vxs, idxs))
                } else {
                    r
                }
            }
        )
    }

    //fn box_clone(&self) -> Box<dyn RenderObject<3>> {
    //    Box::new(self.clone())
    //}
}