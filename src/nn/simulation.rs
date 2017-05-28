use super::neural_network::NeuralNetwork;
use rand::{ThreadRng, Rng};
use itertools::Itertools;

pub struct Simulation {
    pub generation: u32,
    pub next_id: u32,
    pub networks: Vec<NeuralNetwork>,
    pub energy_nodes: Vec<(f32, f32, u32)>,
}

const INPUT_NODE_COUNT: usize = 10;
const LAYER_WIDTH: usize = 20;
const LAYER_COUNT: usize = 3;
const OUTPUT_NODES: usize = 2;
const NODE_SPEED: f32 = 1f32;

impl Simulation {
    pub fn new(rng: &mut ThreadRng) -> Simulation {
        let mut networks = Vec::with_capacity(15);
        for i in 0..15 {
            networks.push(NeuralNetwork::new(i,
                                             rng,
                                             INPUT_NODE_COUNT,
                                             LAYER_WIDTH,
                                             LAYER_COUNT,
                                             OUTPUT_NODES));
        }
        Simulation {
            next_id: 15,
            generation: 0,
            networks: networks,
            energy_nodes: (0..15)
                .map(|_| (rng.gen_range(0f32, 800f32), rng.gen_range(0f32, 600f32), 100))
                .collect(),
        }
    }

    pub fn update(&mut self, rng: &mut ThreadRng) {
        for network in &mut self.networks {
            let (x, y) = (network.x, network.y);
            let x = ((x + 25f32) / 450f32) - 1f32;
            let y = ((y + 25f32) / 350f32) - 1f32;

            let charging = if network.is_charging { 1f32 } else { -1f32 };
            let mut values = [0f32; INPUT_NODE_COUNT];
            values[0] = x;
            values[1] = y;
            values[2] = charging;

            {
                let source_slice: &_ = &self.energy_nodes[..];
                let sources = source_slice.iter();
                let sources = sources.sorted_by(|&&(ax, ay, _), &&(bx, by, _)| {
                    network.distance_to_point(&(ax, ay))
                        .partial_cmp(&network.distance_to_point(&(bx, by)))
                        .unwrap_or(::std::cmp::Ordering::Equal)
                });
                let mut sources = sources.iter();
                for mut i in 3..(INPUT_NODE_COUNT - 1) {
                    if let Some(source) = sources.next() {
                        let angle = (source.0 - x).atan2(source.1 - y);
                        let value = network.distance_to_point(&(source.0, source.1)) / 1000f32 -
                                    1f32;
                        values[i] = angle;
                        i += 1;
                        values[i] = value;
                    } else {
                        break;
                    }
                }
            }

            network.run(&values);
            let ref last_layer = network.network.layers[network.network.layers.len() - 1].nodes;
            network.facing += clamp(last_layer[0].value, -1f32, 1f32) / 10f32;
            let distance = clamp(last_layer[1].value, -1f32, 1f32);
            network.x += network.facing.cos() * distance * NODE_SPEED;
            network.y += network.facing.sin() * distance * NODE_SPEED;


            if let Some(power) = self.energy_nodes
                .iter_mut()
                .find(|n| network.in_range_of(&(n.0, n.1)) && n.2 > 0) {
                network.energy += 1;
                power.2 -= 1;
                network.is_charging = true;
            } else {
                network.is_charging = false;
                network.energy -= 1;
            }
        }

        self.networks.retain(|n| n.is_alive());

        if self.networks.len() < 15 {
            let child = {
                if self.networks.len() <= 2 {
                    NeuralNetwork::new(self.next_id,
                                       rng,
                                       INPUT_NODE_COUNT,
                                       LAYER_WIDTH,
                                       LAYER_COUNT,
                                       OUTPUT_NODES)
                } else {
                    let highest_node = self.networks.iter().max_by_key(|n| n.energy).unwrap();
                    let nearest_node = self.networks
                        .iter()
                        .min_by_key(|n| if n.id == highest_node.id {
                            10000
                        } else {
                            highest_node.distance_to(n) as u32
                        })
                        .unwrap();

                    NeuralNetwork::from_parents(self.next_id, rng, highest_node, nearest_node)
                }
            };
            self.networks.push(child);
            self.next_id += 1;
        }

        self.energy_nodes.retain(|n| n.2 > 0);

        if self.energy_nodes.len() < 15 {
            self.energy_nodes.push((rng.gen_range(0f32, 800f32), rng.gen_range(0f32, 600f32), 100));
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
