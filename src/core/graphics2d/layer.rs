use std::collections::{BTreeMap, HashMap};
use crate::core::vis_geometry::contour::Contour;

/// Draw surface structure,
/// contains
#[derive(Debug, Default)]
pub struct Layer {
    title: Option<String>,
    queue: BTreeMap<i32, Box<dyn Contour>>, // TODO : draw query with object IDS
}


impl Clone for Layer {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            queue: {
                let mut m: BTreeMap<i32, Box<dyn Contour>> = Default::default();
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
            ..Default::default()
        }
    }

    pub fn title(&self) -> &Option<String> { &self.title }

    pub fn set_title(&mut self, title: Option<String>) { self.title = title; }

    pub fn get_queue(&self) -> &BTreeMap<i32, Box<dyn Contour>> { &self.queue }
    
    pub fn get_queue_mut(&mut self) -> &mut BTreeMap<i32, Box<dyn Contour>> { &mut self.queue }

    pub fn push(&mut self, key: i32, contour: Box<dyn Contour>) {
        self.queue.entry(key).or_insert(contour);
    }
    
    pub fn pop_first(&mut self) -> Option<(i32, Box<dyn Contour>)> {
        self.queue.pop_first()
    }
    
    pub fn pop(&mut self, key: i32) -> Option<Box<dyn Contour>> {
        self.queue.remove(&key)
    }
}


pub type Layers = HashMap<String, Layer>;