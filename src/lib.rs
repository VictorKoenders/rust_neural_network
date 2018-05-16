/*
#![feature(step_by)]

extern crate rand;
extern crate itertools;
*/
#![no_std]

#![feature(associated_consts)]


extern crate rand;

mod nn;

pub use nn::*;

const NETWORK_COUNT: usize = 10;
const INPUT_NODE_COUNT: usize = 10;
const OUTPUT_NODE_COUNT: usize = 10;
const LAYER_WIDTH: usize = 10;
const LAYER_DEPTH: usize = 10;
