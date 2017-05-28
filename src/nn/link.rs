use super::node::Node;

pub struct Link {
    pub factor: f32,
    pub node: *const Node,
}
