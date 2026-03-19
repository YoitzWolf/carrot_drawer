use std::collections::HashMap;
use crate::core::graphics2d::layer::{Layer, Layers};

struct Engine {
    layers: Layers,
}

impl Engine {
    pub fn new() -> Self { Self { layers: Layers::new() } }

    pub fn set_layers(&mut self, layers: Layers) { self.layers = layers }

    pub fn add_layer(&mut self, label: String) {
        self.layers.insert(label.clone(), Layer::new(Some(label)));
    }

    pub fn remove_layer(&mut self, label: String) { self.layers.remove(&label); }

    pub fn get_layer(&self, label: &str) -> Option<&Layer> { self.layers.get(label) }

    pub fn get_layer_mut(&mut self, label: &str)  -> Option<&mut Layer> { self.layers.get_mut(label) }

    // pub fn get_rendered(&self) -> HashMap<String, >
}