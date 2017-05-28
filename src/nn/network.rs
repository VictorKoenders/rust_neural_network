use rand::{ThreadRng, Rng};
use super::layer::Layer;

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
            let width = if i == layer_count - 2 {
                output_count
            } else {
                layer_width
            };
            let mut layer = Layer::create_amount_with_previous(width, &layers[i]);

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

                        if first_factor == second_factor {
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

        Network { layers: layers }
    }
}
