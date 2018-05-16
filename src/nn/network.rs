use rand::Rng;
use super::layer::Layer;
use {LAYER_DEPTH, LAYER_WIDTH, INPUT_NODE_COUNT};

#[derive(Default)]
pub struct Network {
    pub layers: [Layer; LAYER_DEPTH],
}

impl Network {
    pub fn initialize(&mut self, rng: &mut Rng) {

    }
    pub fn merge(rng: &mut Rng, first: &Network, second: &Network) -> Self {
        let mut layers: [Layer; LAYER_DEPTH] = unsafe { ::core::mem::uninitialized() };
        for layer in &mut layers {
            *layer = Layer::default();
        }
        /*
        layers[0] = Layer::create_amount(INPUT_NODE_COUNT);

        // TODO: cleanup
        for i in 0..LAYER_DEPTH {
            let mut layer = Layer::create_amount_with_previous(LAYER_WIDTH, &layers[i]);

            for node_index in 0..layer.nodes.len() {
                for link_index in 0..layer.nodes[node_index].links.len() {
                    layer.nodes[node_index].links[link_index].factor = if rng.gen_range(0, 100) ==
                                                                          50 {
                        rng.gen_range(-1f32, 1f32)
                    } else {
                        let first_factor = first.layers[i + 1].nodes[node_index].links[link_index]
                            .factor;
                        let second_factor =
                            second.layers[i + 1].nodes[node_index].links[link_index].factor;

                        if (first_factor - second_factor).abs() < ::core::f32::EPSILON {
                            first_factor
                        } else {
                            rng.gen_range(first_factor.min(second_factor),
                                          first_factor.max(second_factor))
                        }
                    };
                }
            }
            layers.push(layer);
        }
        */

        Network { layers: layers }
    }
}
