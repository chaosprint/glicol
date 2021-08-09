use dasp_graph::{Buffer, Input, Node};
use super::{NodeData, BoxedNodeSend, GlicolNodeData, mono_node};

pub struct Pass<const N:usize> {}

impl<const N:usize> Node<N> for Pass<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // println!("inputs of pass {:?}", inputs);
        output[0] = inputs[0].buffers()[0].clone();
        // println!("output of pass {:?}", output);
    }
}

impl<const N:usize> Pass<N> {
    pub fn new() -> GlicolNodeData<N> {
        mono_node!( N, Self {})
    }
}