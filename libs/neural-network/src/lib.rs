pub use self::layer_topology::*;

use self::layer::*;
use self::neuron::*;

use rand::{Rng};

mod layer;
mod neuron;
mod layer_topology;

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
        .windows(2)
        .map(|layers| {
            Layer::random(layers[0].neurons, layers[1].neurons)
        })
        .collect();

        Self { layers }

    }
    
    pub fn  propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        for layer in &self.layers {
            inputs = layer.propagate(inputs);
        }

        inputs
    }
}