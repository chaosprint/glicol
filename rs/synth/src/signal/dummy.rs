use dasp_graph::{Buffer, Input, Node};
// use dasp_signal::{self as signal, Signal};
// use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError};

pub struct Clock<const N: usize> {}

impl<const N: usize> Node<N> for Clock<N> {
    fn process(&mut self, _inputs: &[Input<N>], _output: &mut [Buffer<N>]) {
        // we set the output buffer manually
    }
}

pub struct AudioIn<const N: usize> {}

// impl AudioIn {
//     // pub fn new() ->
//     // Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
//     //     Ok((NodeData::new1( BoxedNodeSend::new( Self {
//     //     })), vec![]))
//     // }
// }

impl<const N: usize> Node<N> for AudioIn<N> {
    fn process(&mut self, _inputs: &[Input<N>], _output: &mut [Buffer<N>]) {
        // we set the output buffer manually
    }
}