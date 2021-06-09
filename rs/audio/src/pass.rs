use dasp_graph::{Buffer, Input, Node};
use super::{NodeData, BoxedNodeSend, GlicolNodeData, mono_node};

pub struct Pass {}

impl Node<128> for Pass {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        output[0] = inputs[0].buffers()[0].clone();
    }
}

impl Pass {
    pub fn new() -> GlicolNodeData {
        mono_node!(Self {})
    }
}