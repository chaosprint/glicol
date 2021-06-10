use dasp_graph::{Buffer, Input, Node};
use super::{NodeData, BoxedNodeSend, GlicolNodeData, mono_node};

pub struct Pass {}

impl Node<128> for Pass {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // println!("inputs of pass {:?}", inputs);
        output[0] = inputs[0].buffers()[0].clone();
        // println!("output of pass {:?}", output);
    }
}

impl Pass {
    pub fn new() -> GlicolNodeData {
        mono_node!(Self {})
    }
}