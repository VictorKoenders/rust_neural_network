// #![no_std]
#![feature(test)]

pub extern crate rand;
use rand::Rng;

#[cfg(test)]
extern crate test;

// use core as lib;
use std as lib;

pub const NODE_INPUT_COUNT: usize = 8;
pub const LAYER_WIDTH: usize = 10;
pub const LAYER_DEPTH: usize = 10;
pub const NODE_OUTPUT_COUNT: usize = 8;
pub const NETWORK_COUNT: usize = 10;

#[derive(Default)]
pub struct Networks {
    networks: [Network; NETWORK_COUNT],
    scores: [f32; NETWORK_COUNT],
}

impl Networks {
    pub fn initalize(&mut self, rng: &mut impl Rng) {
        for network in &mut self.networks {
            network.initialize_random_values(rng);
        }
    }
    pub fn apply_score(&mut self, rng: &mut impl Rng) {
        use lib::f32;
        let mut highest_index = 0;
        let mut lowest_index = 0;
        let mut highest_value = f32::NAN;
        let mut lowest_value = f32::INFINITY;
        for (index, value) in self.scores.iter().cloned().enumerate() {
            if value > highest_value {
                highest_index = index;
                highest_value = value;
            }
            if value < lowest_value {
                lowest_index = index;
                lowest_value = value;
            }
        }

        let other_mate = loop {
            let index = rng.gen_range(0, NETWORK_COUNT);
            if index != lowest_index && index != highest_index {
                break index;
            }
        };

        self.networks[lowest_index] =
            self.networks[highest_index].mate_with(&self.networks[other_mate], rng);
    }
}

pub struct Network {
    pub change_chance: f32,
    pub first_layer: FirstLayer,
    pub layers: [Layer; LAYER_DEPTH - 1],
    pub output: [Node; NODE_OUTPUT_COUNT],
}

impl Default for Network {
    fn default() -> Network { unsafe { lib::mem::zeroed() }}
}

impl Network {
    pub fn initialize_random_values(&mut self, rng: &mut impl Rng) {
        self.change_chance = rng.gen_range(0.1, 1.0);
        for node in self.first_layer.iter_mut() {
            for index in 0..NODE_INPUT_COUNT {
                node.input_node_values[index] = rng.gen_range(-1.0f32, 1.0f32);
            }
        }

        for i in 1..LAYER_DEPTH - 1 {
            for j in 0..LAYER_WIDTH {
                for k in 0..LAYER_WIDTH {
                    self.layers[i][j].input_node_values[k] = rng.gen_range(-1.0f32, 1.0f32);
                }
            }
        }

        for i in 0..NODE_OUTPUT_COUNT {
            for j in 0..LAYER_WIDTH {
                self.output[i].input_node_values[j] = rng.gen_range(-1.0f32, 1.0f32);
            }
        }
    }

    fn gen_between(v1: &f32, v2: &f32, rng: &mut impl Rng, change_chance: &f32) -> f32 {
        if rng.gen_range(0.0, 1.0) < *change_chance {
            rng.gen_range(-1.0, 1.0)
        } else {
            rng.gen_range(*v1, *v2)
        }
    }

    pub fn mate_with(&self, other: &Network, rng: &mut impl Rng) -> Network {
        let mut new_network: Network = Default::default();

        new_network.change_chance =
            Network::gen_between(&self.change_chance, &other.change_chance, rng, &0f32);

        if rng.gen_range(0.0, 1.0) < new_network.change_chance / 100f32 {
            new_network.change_chance = rng.gen_range(0.1, 1.0);
        }
        for (node_index, node) in new_network.first_layer.iter_mut().enumerate() {
            for index in 0..NODE_INPUT_COUNT {
                node.input_node_values[index] = Network::gen_between(
                    &self.first_layer[node_index].input_node_values[index],
                    &other.first_layer[node_index].input_node_values[index],
                    rng,
                    &new_network.change_chance,
                );
            }
        }

        for i in 1..LAYER_DEPTH - 1 {
            for j in 0..LAYER_WIDTH {
                for k in 0..LAYER_WIDTH {
                    new_network.layers[i][j].input_node_values[k] = Network::gen_between(
                        &self.layers[i][j].input_node_values[k],
                        &other.layers[i][j].input_node_values[k],
                        rng,
                        &new_network.change_chance
                    );
                }
            }
        }

        for i in 0..NODE_OUTPUT_COUNT {
            for j in 0..LAYER_WIDTH {
                new_network.output[i].input_node_values[j] = Network::gen_between(
                    &self.output[i].input_node_values[j],
                    &other.output[i].input_node_values[j],
                    rng,
                    &new_network.change_chance
                );
            }
        }

        new_network
    }

    pub fn execute(&mut self, input: &[f32; NODE_INPUT_COUNT]) -> [f32; NODE_OUTPUT_COUNT] {
        for node in self.first_layer.iter_mut() {
            node.value = node.calculate_value(input);
        }
        for node in self.layers[0].iter_mut() {
            node.value = node.calculate_value(&self.first_layer);
        }
        for i in 1.. self.layers.len() {
            let (previous, current) = self.layers.split_at_mut(i);
            for node in current[0].iter_mut() {
                let value = node.calculate_value(previous.last().unwrap());
                node.value = value;
            }
        }
        let mut output = [0f32; NODE_OUTPUT_COUNT];
        for (index, node) in self.output.iter_mut().enumerate() {
            node.value = node.calculate_value(self.layers.last().unwrap());
            output[index] = node.value;
        }
        output
    }
}

#[test]
fn test() {
    println!("Networks size: {}", lib::mem::size_of::<Networks>());
    println!("Network size: {}", lib::mem::size_of::<Network>());
    let mut network = Network::default();
    let input = Default::default();
    let output = network.execute(&input);
    assert_eq!(
        output,
        [0f32; NODE_OUTPUT_COUNT]
    );
}

#[bench]
fn bench(b: &mut test::Bencher) {
    let mut network = Network::default();
    let input = Default::default();
    b.iter(|| network.execute(&input));
}

pub type Layer = [Node; LAYER_WIDTH];
pub type FirstLayer = [FirstNode; LAYER_WIDTH];

pub trait NodeImpl {
    fn value(&self) -> f32;
    fn modifiers(&self) -> &[f32];

    fn calculate_value(&self, inputs: &[impl NodeImpl]) -> f32 {
        let mut value = 0f32;
        for (index, modifier) in self.modifiers().iter().enumerate() {
            value += modifier * inputs[index].value();
        }
        value
    }
}

pub struct Node {
    pub value: f32,
    pub input_node_values: [f32; LAYER_WIDTH],
}

impl NodeImpl for Node {
    fn value(&self) -> f32 { self.value }
    fn modifiers(&self) -> &[f32] {
        &self.input_node_values
    }
}

pub struct FirstNode {
    pub value: f32,
    pub input_node_values: [f32; NODE_INPUT_COUNT],
}

impl NodeImpl for FirstNode {
    fn value(&self) -> f32 { self.value }
    fn modifiers(&self) -> &[f32] {
        &self.input_node_values
    }
}

impl NodeImpl for f32 {
    fn value(&self) -> f32 { *self }
    fn modifiers(&self) -> &[f32] { &[] }
}