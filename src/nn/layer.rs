use rand::{ThreadRng, Rng};
use super::node::Node;
use super::link::Link;

pub struct Layer {
    pub nodes: Vec<Node>,
}

impl Layer {
    pub fn create_amount(amount: usize) -> Layer {
        let mut layer = Layer { nodes: Vec::with_capacity(amount) };
        for _ in 0..amount {
            layer.nodes.push(Node {
                value: 0f32,
                links: Vec::new(),
            });
        }
        layer
    }

    pub fn create_amount_with_previous(amount: usize, previous: &Layer) -> Layer {
        let mut layer = Layer { nodes: Vec::with_capacity(amount) };
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

    pub fn create_amount_with_previous_and_values(rng: &mut ThreadRng,
                                                  amount: usize,
                                                  previous: &Layer)
                                                  -> Layer {
        let mut layer = Layer { nodes: Vec::with_capacity(amount) };
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
