extern crate rust_nn;
extern crate rand;

use rust_nn::neural_network::NeuralNetwork;
use rust_nn::simulation::{
    INPUT_NODE_COUNT,
    LAYER_WIDTH,
    LAYER_COUNT,
    OUTPUT_NODES
};
use std::f32::consts::PI;

#[test]
pub fn test_positional_arguments(){
    let mut rng = rand::thread_rng();
    let mut nn = NeuralNetwork::new(0, &mut rng, INPUT_NODE_COUNT, LAYER_WIDTH, LAYER_COUNT, OUTPUT_NODES);

    nn.x = -25f32;
    nn.y = -25f32;

    let args = nn.generate_first_layer_values(&[]);

    debug_assert_eq!(args[0], -1f32);
    debug_assert_eq!(args[1], -1f32);
    
    nn.x = 825f32;
    nn.y = -25f32;

    let args = nn.generate_first_layer_values(&[]);

    debug_assert_eq!(args[0], 1f32);
    debug_assert_eq!(args[1], -1f32);
    
    nn.x = -25f32;
    nn.y = 625f32;

    let args = nn.generate_first_layer_values(&[]);

    debug_assert_eq!(args[0], -1f32);
    debug_assert_eq!(args[1], 1f32);
    
    nn.x = 825f32;
    nn.y = 625f32;

    let args = nn.generate_first_layer_values(&[]);

    debug_assert_eq!(args[0], 1f32);
    debug_assert_eq!(args[1], 1f32);
}

#[test]
pub fn test_energy_source_arguments(){
    let mut rng = rand::thread_rng();
    let mut nn = NeuralNetwork::new(
        0,
        &mut rng,
        INPUT_NODE_COUNT,
        LAYER_WIDTH,
        LAYER_COUNT,
        OUTPUT_NODES
    );

    nn.x = 300f32;
    nn.y = 300f32;

    let sources = vec![
        (0f32, 0f32, 0),
        (800f32, 300f32, 0),
        (0f32, 600f32, 0)
    ];

    let args = nn.generate_first_layer_values(&sources[..]);

    debug_assert_eq!(args[4], -135f32 * PI / 180f32);
    debug_assert_eq!(args[5], -0.4375);
    debug_assert_eq!(args[6], 135f32 * PI / 180f32);
    debug_assert_eq!(args[7], -0.4375);
    debug_assert_eq!(args[8], 0f32 * PI / 180f32);
    debug_assert_eq!(args[9], -0.21875f32);
}