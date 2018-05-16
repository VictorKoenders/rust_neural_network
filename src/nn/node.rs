use super::link::Link;
use LAYER_WIDTH;

#[derive(Default)]
pub struct Node {
    pub value: f32,
    pub links: [Link; LAYER_WIDTH],
}
