use super::neural_network::NeuralNetwork;
use rand::{Rng, ThreadRng};
use NETWORK_COUNT;

#[derive(Default)]
pub struct Simulation {
    pub generation: u32,
    pub networks: [NeuralNetwork; NETWORK_COUNT],
}

pub const NODE_SPEED: f32 = 1f32;

impl Simulation {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut networks: [NeuralNetwork; NETWORK_COUNT] = Default::default();
        for i in 0..NETWORK_COUNT {
            networks[i] = NeuralNetwork::new(rng);
        }
        Simulation {
            generation: 0,
            networks: networks,
        }
    }
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}
