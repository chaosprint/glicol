use dasp_graph::{Buffer, Input, Node};
// use dasp_slice::add_in_place;
use super::super::{ NodeData, GlicolNodeData, BoxedNodeSend, mono_node};
use super::{Para};

pub struct ConstSig {
    val: f32
}

impl ConstSig {
    pub fn new(val: f32) -> GlicolNodeData {
        mono_node! ( Self {val} )
    }
}

#[macro_export]
macro_rules! const_sig {
    ($data: expr) => {
        ConstSig::new($data)
    };
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