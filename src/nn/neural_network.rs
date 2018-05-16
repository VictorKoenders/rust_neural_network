use super::layer::Layer;
use super::network::Network;
use rand::Rng;
use {LAYER_DEPTH, LAYER_WIDTH, INPUT_NODE_COUNT, OUTPUT_NODE_COUNT};

#[derive(Default)]
pub struct NeuralNetwork {
    pub network: Network,
}

impl NeuralNetwork {
    pub fn new(rng: &mut Rng) -> Self {
        let mut network = Network {
            layers: Default::default(),
        };

        network.initialize(rng);

        NeuralNetwork {
            network: network,
        }
    }

    pub fn from_parents(
        id: u32,
        rng: &mut Rng,
        first: &NeuralNetwork,
        second: &NeuralNetwork,
    ) -> Self {
        NeuralNetwork {
            network: Network::merge(rng, &first.network, &second.network),
        }
    }

    pub fn run(&mut self, values: &[f32]) {
        for (index, layer) in self.network.layers.iter_mut().enumerate() {
            if index == 0 {
                assert_eq!(
                    layer.nodes.len(),
                    values.len(),
                    "Input layer length should equal values length"
                );

                for (index, node) in layer.nodes.iter_mut().enumerate() {
                    node.value = values[index];
                }
                continue;
            }
            for node in &mut layer.nodes {
                let mut value = 0f32;
                for link in &mut node.links {
                    value += clamp(link.factor * unsafe { (*link.node).value }, -1f32, 1f32);
                }
                node.value = value;
            }
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
