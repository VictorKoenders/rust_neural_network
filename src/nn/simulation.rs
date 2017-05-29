use super::neural_network::NeuralNetwork;
use rand::{ThreadRng, Rng};

pub struct Simulation {
    pub generation: u32,
    pub next_id: u32,
    pub networks: Vec<NeuralNetwork>,
    pub energy_nodes: Vec<(f32, f32, u32)>,
}

pub const INPUT_NODE_COUNT: usize = 10;
pub const LAYER_WIDTH: usize = 20;
pub const LAYER_COUNT: usize = 3;
pub const OUTPUT_NODES: usize = 2;
pub const NODE_SPEED: f32 = 1f32;
pub const INITIAL_POWER_AMOUNT: u32 = 1000;

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
                .map(|_| (rng.gen_range(0f32, 800f32), rng.gen_range(0f32, 600f32), INITIAL_POWER_AMOUNT))
                .collect(),
        }
    }

    pub fn update(&mut self, rng: &mut ThreadRng) {
        for network in &mut self.networks {
            let values = network.generate_first_layer_values(&self.energy_nodes[..]);

            network.run(&values);
            let last_layer = &network.network.layers[network.network.layers.len() - 1].nodes;
            network.facing += clamp(last_layer[0].value, -1f32, 1f32) / 10f32;
            let distance = clamp(last_layer[1].value, -1f32, 1f32);
            network.x += network.facing.cos() * distance * NODE_SPEED;
            network.y += network.facing.sin() * distance * NODE_SPEED;


            if let Some(power) = self.energy_nodes
                .iter_mut()
                .find(|n| network.in_range_of(&(n.0, n.1)) && n.2 > 0) {
                if power.2 >= 5 {
                    network.energy += 5;
                    power.2 -= 5;
                } else {
                    network.energy += power.2;
                    power.2 = 0;
                }
                network.is_charging = true;
            } else {
                network.is_charging = false;
                if network.energy > 2 {
                    network.energy -= 2;
                } else {
                    network.energy = 0;
                }
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

        for node in &mut self.energy_nodes {
            if node.2 > 0 {
                node.2 -= 1;
            }
        }

        self.energy_nodes.retain(|n| n.2 > 0);

        if self.energy_nodes.len() < 15 {
            self.energy_nodes.push((rng.gen_range(0f32, 800f32), rng.gen_range(0f32, 600f32), INITIAL_POWER_AMOUNT));
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
