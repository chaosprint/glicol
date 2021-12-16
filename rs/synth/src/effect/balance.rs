use dasp_graph::{Buffer, Input, Node};
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, stereo_node};

pub struct Balance<const N:usize> {
    balance: f32
}
impl<const N:usize> Balance<N> {
    pub fn new(balance: f32) -> GlicolNodeData<N> {
        stereo_node!( N, Self { balance } )
    }
}

impl<const N:usize> Node<N> for Balance<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // let _clock = inputs[2].clone();
        let left = inputs[1].buffers()[0].clone();
        let right = inputs[0].buffers()[0].clone();
        output[0] = left;
        output[1] = right;
        output[0].iter_mut().for_each(|s| *s = *s * (1.0 - self.balance));
        output[1].iter_mut().for_each(|s| *s = *s * self.balance);
    }
}