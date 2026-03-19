use std::collections::{BTreeMap, HashMap};
use crate::core::vis_geometry::gentraits::{Colorable};
use crate::core::vis_geometry::render_object::RenderObject;
use crate::core::vis_geometry::Vertex;

pub trait InLayerRenderObject: crate::core::vis_geometry::render_object::RenderObject<3> + Colorable {
    fn box_clone(&self) -> Box<dyn InLayerRenderObject>;
}

impl<'a, T: RenderObject<3> + Colorable + Clone + 'static> InLayerRenderObject for T {
    fn box_clone(&self) -> Box<dyn InLayerRenderObject> {
        Box::new(self.clone())
    }
}

/// Draw surface structure,
#[derive(Debug, Default)]
pub struct Layer {
    title: Option<String>,
    modified: bool,
    queue: BTreeMap<i64, Box<dyn InLayerRenderObject>>, // TODO : draw query with object IDS
}


impl Clone for Layer {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            modified: self.modified,
            queue: {
                let mut m: BTreeMap<i64, Box<dyn InLayerRenderObject>> = Default::default();
                self.queue.iter().for_each(
                    |(key, bx)| {
                        m.insert(*key, {
                            bx.box_clone()
                        });
                    }
                );
                m
            },
        }
    }
}

impl Layer {
    pub fn new(title: Option<String>) -> Self {
        Self {
            title,
            modified: true,
            ..Default::default()
        }
    }

    pub fn title(&self) -> &Option<String> { &self.title }

    // pub fn set_title(&mut self, title: Option<String>) { self.title = title; }

    pub fn get_queue(&self) -> &BTreeMap<i64, Box<dyn InLayerRenderObject >> { &self.queue }
    
    // pub fn get_queue_mut(&mut self) -> &mut BTreeMap<i32, Box<dyn Contour>> { &mut self.queue }

    pub fn push(&mut self, key: i64, contour: Box<dyn InLayerRenderObject>) {
        self.modified = true;
        self.queue.entry(key).or_insert(contour);
    }
    
    pub fn pop_first(&mut self) -> Option<(i64, Box<dyn InLayerRenderObject>)> {
        self.modified = true;
        self.queue.pop_first()
    }
    
    pub fn pop(&mut self, key: i64) -> Option<Box<dyn InLayerRenderObject>> {
        self.modified = true;
        self.queue.remove(&key)
    }

    pub fn get_mut(&mut self, key: i64) -> Option<&mut Box<dyn InLayerRenderObject>> {
        self.modified = true;
        self.queue.get_mut(&key)
    }
}

impl crate::core::vis_geometry::gentraits::Rotated for Layer {
    fn set_rotation(&mut self, angle: f32) {
        unimplemented!()
    }

    fn rotate(&mut self, angle: f32) {
        unimplemented!()
    }
}

impl crate::core::vis_geometry::render_object::RenderObject<3> for Layer {
    fn render(&self) -> anyhow::Result<(Vec<Vertex<3>>, Vec<u32>)> {
        self.queue.iter().fold(
            Ok((vec![], vec![])),
            |r, x| {
                if let Ok((mut vxs, mut idxs)) = r {
                    let (mut vx, mut ids) = x.1.render()?;
                    // println!("In-layer render: {:?} {:?}", vx, ids);
                    let N = vxs.len() as u32;
                    ids.iter_mut().for_each(|i| *i += N);
                    vxs.append(&mut vx);
                    idxs.append(&mut ids);
                    Ok((vxs, idxs))
                } else {
                    r
                }
            }
        )
    }
}

pub type Layers = HashMap<String, Layer>;