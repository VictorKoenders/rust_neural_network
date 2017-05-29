use super::simulation::INPUT_NODE_COUNT;
use super::network::Network;
use rand::{ThreadRng, Rng};
use itertools::Itertools;
use super::layer::Layer;

pub struct NeuralNetwork {
    pub id: u32,
    pub network: Network,
    pub x: f32,
    pub y: f32,
    pub facing: f32,
    pub energy: u32,
    pub is_charging: bool,
}

impl NeuralNetwork {
    pub fn new(id: u32,
               rng: &mut ThreadRng,
               input_nodes: usize,
               layer_width: usize,
               layer_count: usize,
               output_nodes: usize)
               -> NeuralNetwork {
        let mut network = Network { layers: Vec::with_capacity(layer_count + 2) };
        network.layers.push(Layer::create_amount(input_nodes));
        for i in 0..layer_count {
            let layer =
                Layer::create_amount_with_previous_and_values(rng, layer_width, &network.layers[i]);
            network.layers.push(layer);
        }
        let layer = Layer::create_amount_with_previous_and_values(rng,
                                                                  output_nodes,
                                                                  &network.layers[network.layers.len() -
                                                                   1]);
        network.layers.push(layer);

        NeuralNetwork {
            id: id,
            network: network,
            x: rng.gen_range(0f32, 800f32),
            y: rng.gen_range(0f32, 600f32),
            facing: rng.gen_range(0f32, 2f32 * ::std::f32::consts::PI),
            energy: 1000,
            is_charging: false,
        }
    }

    pub fn from_parents(id: u32,
                        rng: &mut ThreadRng,
                        first: &NeuralNetwork,
                        second: &NeuralNetwork)
                        -> NeuralNetwork {
        NeuralNetwork {
            id: id,
            network: Network::merge(rng, &first.network, &second.network),
            x: (first.x - second.x) / 2f32 + first.x,
            y: (first.y - second.y) / 2f32 + first.y,
            facing: rng.gen_range(0f32, 2f32 * ::std::f32::consts::PI),
            energy: 1000,
            is_charging: false,
        }
    }

    pub fn generate_first_layer_values(&self, energy_nodes: &[(f32, f32, u32)]) -> [f32;INPUT_NODE_COUNT] {
        let (x, y) = (self.x, self.y);
        let x = ((x + 25f32) / 425f32) - 1f32;
        let y = ((y + 25f32) / 325f32) - 1f32;

        let charging = if self.is_charging { 1f32 } else { -1f32 };
        let mut values = [0f32; INPUT_NODE_COUNT];
        values[0] = x;
        values[1] = y;
        values[2] = charging;
        let mut facing = self.facing;
        while facing < -::std::f32::consts::PI {
            facing += 2f32 * ::std::f32::consts::PI;
        }
        while facing > ::std::f32::consts::PI {
            facing -= 2f32 * ::std::f32::consts::PI;
        }
        values[3] = facing / ::std::f32::consts::PI / 2f32;

        {
            let sources = energy_nodes.iter();
            let sources = sources.sorted_by(|&&(ax, ay, _), &&(bx, by, _)| {
                self.distance_to_point(&(ax, ay))
                    .partial_cmp(&self.distance_to_point(&(bx, by)))
                    .unwrap_or(::std::cmp::Ordering::Equal)
            });
            let mut sources = sources.iter();
            for i in (3..(INPUT_NODE_COUNT - 1)).step_by(2) {
                if let Some(source) = sources.next() {
                    let mut angle = facing - (source.1 - self.y).atan2(source.0 - self.x);
                    while angle < -::std::f32::consts::PI {
                        angle += 2f32 * ::std::f32::consts::PI;
                    }
                    while angle > ::std::f32::consts::PI {
                        angle -= 2f32 * ::std::f32::consts::PI;
                    }
                    let distance = self.distance_to_point(&(source.0, source.1)) / (800f32 * 600f32 / 1.5f32) - 1f32;
                    values[i] = angle / ::std::f32::consts::PI;
                    values[i + 1] = distance;
                } else {
                    break;
                }
            }
        }

        values
    }

    pub fn distance_to(&self, other: &NeuralNetwork) -> f32 {
        ((self.x - other.x).powf(2f32) + (self.y - other.y).powf(2f32))
    }
    pub fn distance_to_point(&self, point: &(f32, f32)) -> f32 {
        ((self.x - point.0).powf(2f32) + (self.y - point.1).powf(2f32))
    }

    pub fn in_range_of(&self, point: &(f32, f32)) -> bool {
        ((self.x - point.0).powf(2f32) + (self.y - point.1).powf(2f32)) < 2500f32
    }

    pub fn is_alive(&self) -> bool {
        if self.energy == 0 {
            false
        } else if self.x <= -25f32 {
            false
        } else if self.x >= 825f32 {
            false
        } else if self.y <= -25f32 {
            false
        } else if self.y >= 625f32 {
            false
        } else {
            true
        }
    }

    pub fn run(&mut self, values: &[f32]) {
        for (index, layer) in self.network.layers.iter_mut().enumerate() {
            if index == 0 {
                assert_eq!(layer.nodes.len(),
                           values.len(),
                           "Input layer length should equal values length");

                for (index, node) in layer.nodes.iter_mut().enumerate() {
                    node.value = values[index];
                }
                continue;
            }
            for node in &mut layer.nodes {
                let mut value = 0f32;
                for link in &mut node.links {
                    value += link.factor * unsafe { (*link.node).value };
                }
                node.value = value;
            }
        }
    }
}
