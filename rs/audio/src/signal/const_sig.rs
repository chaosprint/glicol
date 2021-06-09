use dasp_graph::{Buffer, Input, Node};
use super::super::{ NodeData, GlicolNodeData, BoxedNodeSend, mono_node};

pub struct ConstSig {
    val: f32
}

impl ConstSig {
    pub fn new(val: f32) -> GlicolNodeData {
        mono_node! ( Self {val} )
    }
}

impl Node<128> for ConstSig {
    fn process(&mut self, _inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // if inputs.len() > 1 {
        //     self.val = inputs[0].buffers()[0][0];
        // }
        for o in output {
            o.iter_mut().for_each(|s| *s = self.val);
        }
    }
}