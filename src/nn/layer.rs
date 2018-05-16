use rand::Rng;
use super::node::Node;
use LAYER_WIDTH;

#[derive(Default)]
pub struct Layer {
    pub nodes: [Node; LAYER_WIDTH],
}

impl Layer {
    pub fn create_amount(amount: usize) -> Self {
        unimplemented!();
        // let mut layer = Layer { nodes: [Node::default(); amount] };
        // layer
    }

    pub fn create_amount_with_previous(amount: usize, previous: &Layer) -> Layer {
        unimplemented!();
        /*let mut layer = Layer { nodes: [Node::default(); amount] };
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
        layer*/
    }

    pub fn create_amount_with_previous_and_values(rng: &mut Rng,
                                                  amount: usize,
                                                  previous: &Layer)
                                                  -> Layer {
        unimplemented!();
        /*
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
        layer*/
    }
}
