use super::node::Node;
use core::ptr;

pub struct Link {
    pub factor: f32,
    pub node: *const Node,
}

impl Default for Link {
    fn default() -> Link {
        Link {
            factor: 0f32,
            node: ptr::null_mut(),
        }
    }
}
