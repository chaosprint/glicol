use dasp_graph::{Buffer, Input, Node};
use super::super::{ NodeData, GlicolNodeData, BoxedNodeSend, mono_node};

pub struct ConstSig<const N:usize> {
    val: f32
}

impl<const N:usize> ConstSig<N> {
    pub fn new(val: f32) -> GlicolNodeData<N> {
        mono_node! ( N, Self {val} )
    }
}

impl<const N:usize> Node<N> for ConstSig<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // if inputs.len() > 1 {
        //     self.val = inputs[0].buffers()[0][0];
        // }
        for o in output {
            o.iter_mut().for_each(|s| *s = self.val);
        }
    }
}