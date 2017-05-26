use rand::{ThreadRng, Rng};

pub struct Node {
    pub value: f32,
    pub links: Vec<Link>,
}

pub struct Link {
    pub factor: f32,
    pub node: *const Node,
}

pub struct Network {
    pub layers: Vec<Layer>,
}

impl Network {
    pub fn merge(rng: &mut ThreadRng, first: &Network, second: &Network) -> Network {
        let layer_count = first.layers.len();
        let input_count = first.layers[0].nodes.len();
        let layer_width = first.layers[1].nodes.len();
        let output_count = first.layers[layer_count - 1].nodes.len();

        let mut layers = Vec::with_capacity(layer_count);
        layers.push(Layer::create_amount(input_count));
        for i in 0..layer_count - 1 {
            let width = if i == layer_count - 2 { output_count } else { layer_width };
            let mut layer = Layer::create_amount_with_previous(width, &layers[i]);

            for node_index in 0..layer.nodes.len() {
                for link_index in 0..layer.nodes[node_index].links.len() {
                    layer.nodes[node_index].links[link_index].factor = if rng.gen_range(0, 100) == 50 {
                        rng.gen_range(-1f32, 1f32)
                    } else {
                        let first_factor = first.layers[i + 1].nodes[node_index].links[link_index].factor;
                        let second_factor = second.layers[i + 1].nodes[node_index].links[link_index].factor;

                        if first_factor == second_factor {
                            first_factor
                        } else {                        
                            rng.gen_range(first_factor.min(second_factor), first_factor.max(second_factor))
                        }
                    };
                }
            }
            layers.push(layer);
        }

        Network {
            layers: layers
        }
    }
}

pub struct Layer {
    pub nodes: Vec<Node>,
}

impl Layer {
    pub fn create_amount(amount: usize) -> Layer {
        let mut layer = Layer {
            nodes: Vec::with_capacity(amount)
        };
        for _ in 0..amount {
            layer.nodes.push(Node {
                value: 0f32,
                links: Vec::new()
            });
        }
        layer
    }

    pub fn create_amount_with_previous(amount: usize, previous: &Layer) -> Layer {
        let mut layer = Layer {
            nodes: Vec::with_capacity(amount)
        };
        for _ in 0..amount {
            let mut node = Node {
                value: 0f32,
                links: Vec::with_capacity(previous.nodes.len()),
            };

            for previous_node in &previous.nodes {
                node.links.push(Link {
                    node: previous_node,
                    factor: 0f32,
                })
            }
            
            layer.nodes.push(node);
        }
        layer
    }

    pub fn create_amount_with_previous_and_values(rng: &mut ThreadRng, amount: usize, previous: &Layer) -> Layer {
        let mut layer = Layer {
            nodes: Vec::with_capacity(amount)
        };
        for _ in 0..amount {
            let mut node = Node {
                value: 0f32,
                links: Vec::with_capacity(previous.nodes.len()),
            };

            for previous_node in &previous.nodes {
                node.links.push(Link {
                    node: previous_node,
                    factor: rng.gen_range(-1f32, 1f32),
                })
            }
            
            layer.nodes.push(node);
        }
        layer
    }
}

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
    pub fn new(id: u32, rng: &mut ThreadRng, input_nodes: usize, layer_width: usize, layer_count: usize, output_nodes: usize) -> NeuralNetwork {
        let mut network = Network {
            layers: Vec::with_capacity(layer_count + 2)
        };
        network.layers.push(Layer::create_amount(input_nodes));
        for i in 0..layer_count {
            let layer = Layer::create_amount_with_previous_and_values(rng, layer_width, &network.layers[i]);
            network.layers.push(layer);
        }
        let layer = Layer::create_amount_with_previous_and_values(rng, output_nodes, &network.layers[network.layers.len() - 1]);
        network.layers.push(layer);
        
        NeuralNetwork {
            id: id,
            network: network,
            x: rng.gen_range(0f32, 800f32),
            y: rng.gen_range(0f32, 600f32),
            facing: rng.gen_range(0f32, 2f32 * ::std::f32::consts::PI),
            energy: 1000,
            is_charging: false
        }
    }

    pub fn from_parents(id: u32, rng: &mut ThreadRng, first: &NeuralNetwork, second: &NeuralNetwork) -> NeuralNetwork {
        NeuralNetwork {
            id: id,
            network: Network::merge(rng, &first.network, &second.network),
            x: (first.x - second.x) / 2f32 + first.x,
            y: (first.y - second.y) / 2f32 + first.y,
            facing: rng.gen_range(0f32, 2f32 * ::std::f32::consts::PI),
            energy: 1000,
            is_charging: false
        }
    }

    pub fn distance_to(&self, other: &NeuralNetwork) -> f32 {
        ((self.x - other.x).powf(2f32) + (self.y - other.y).powf(2f32))
    }

    pub fn in_range_of(&self, point: &(f32, f32)) -> bool {
        ((self.x - point.0).powf(2f32) + (self.y - point.1).powf(2f32)) < 2500f32
    }

    pub fn is_alive(&self) -> bool {
        if self.energy == 0 { false }
        else if self.x <= -25f32 { false }
        else if self.x >= 825f32 { false }
        else if self.y <= -25f32 { false }
        else if self.y >= 625f32 { false }
        else { true }
    }

    pub fn run(&mut self, values: &[f32]){
        for (index, layer) in self.network.layers.iter_mut().enumerate(){
            if index == 0 {
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

pub struct Simulation {
    pub generation: u32,
    pub next_id: u32,
    pub networks: Vec<NeuralNetwork>,
    pub energy_nodes: Vec<(f32, f32, u32)>,
}

const INPUT_NODE_COUNT: usize = 10;
const LAYER_WIDTH: usize = 20;
const LAYER_COUNT: usize = 3;
const OUTPUT_NODES: usize = 3;

impl Simulation {
    pub fn new(rng: &mut ThreadRng) -> Simulation {
        let mut networks = Vec::with_capacity(15);
        for i in 0..15 {
            networks.push(
                NeuralNetwork::new(i, rng, INPUT_NODE_COUNT, LAYER_WIDTH, LAYER_COUNT, OUTPUT_NODES)
            );
        }
        Simulation {
            next_id: 15,
            generation: 0,
            networks: networks,
            energy_nodes: (0..15).map(|_| {
                (rng.gen_range(0f32, 800f32), rng.gen_range(0f32, 600f32), 100)
            }).collect()
        }
    }

    pub fn update(&mut self, rng: &mut ThreadRng) {
        for network in &mut self.networks {
            let (x, y) = (network.x, network.y);
            let x = ((x + 25f32) / 450f32) - 1f32;
            let y = ((y + 25f32) / 350f32) - 1f32;
            
            let charging = if network.is_charging { 1f32 } else { -1f32 };
            network.run(&[
                x,
                y,
                charging,
                1f32,
                1f32,
                1f32,
                1f32,
                1f32,
                1f32,
                1f32
            ]);
            let ref last_layer = network.network.layers[network.network.layers.len() - 1].nodes;
            network.x += clamp(last_layer[0].value, -1f32, 1f32);
            network.y += clamp(last_layer[1].value, -1f32, 1f32);
            network.facing += clamp(last_layer[2].value, -0.1f32, 0.1f32);

            if let Some(power) = self.energy_nodes.iter_mut().find(|n| network.in_range_of(&(n.0, n.1)) && n.2 > 0) {
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
                    NeuralNetwork::new(self.next_id, rng, INPUT_NODE_COUNT, LAYER_WIDTH, LAYER_COUNT, OUTPUT_NODES)
                } else {
                    let highest_node = self.networks.iter().max_by_key(|n| n.energy).unwrap();
                    let nearest_node = self.networks.iter().min_by_key(|n| if n.id == highest_node.id { 10000 } else { highest_node.distance_to(n) as u32 }).unwrap();
                    
                    NeuralNetwork::from_parents(self.next_id, rng, highest_node, nearest_node)
                }
            };
            self.networks.push(child);
            self.next_id += 1;
        }

        self.energy_nodes.retain(|n| n.2 > 0);

        if self.energy_nodes.len() < 15 {
            self.energy_nodes.push(
                (rng.gen_range(0f32, 800f32), rng.gen_range(0f32, 600f32), 100)
            );
        }
    }
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value > max { max }
    else if value < min { min }
    else { value }
}