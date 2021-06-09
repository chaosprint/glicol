use dasp_graph::{Buffer, Input, Node};
// use dasp_signal::{self as signal, Signal};
// use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError};

pub struct Clock {}

impl Node<128> for Clock {
    fn process(&mut self, _inputs: &[Input<128>], _output: &mut [Buffer<128>]) {
        // we set the output buffer manually
    }
}

pub struct AudioIn {}

// impl AudioIn {
//     // pub fn new() ->
//     // Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
//     //     Ok((NodeData::new1( BoxedNodeSend::new( Self {
//     //     })), vec![]))
//     // }
// }

impl Node<128> for AudioIn {
    fn process(&mut self, _inputs: &[Input<128>], _output: &mut [Buffer<128>]) {
        // we set the output buffer manually
    }
}