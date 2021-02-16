use dasp_graph::{Buffer, Input, Node};
// use dasp_slice::add_in_place;
use super::super::{Pairs, Rule, NodeData, 
    BoxedNodeSend, NodeResult, handle_params};

pub struct ConstSig {
    val: f32,
    sidechain_ids: Vec<u8>
}

impl ConstSig {
    handle_params!({
        val: 1.0
    });
}

impl Node<128> for ConstSig {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        if self.sidechain_ids.len() > 0 {
            self.val = inputs[0].buffers()[0][0];
        }
        for o in output {
            o.iter_mut().for_each(|s| *s = self.val);
        }
    }
}