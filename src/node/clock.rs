use dasp_graph::{Buffer, Input, Node};
// use dasp_signal::{self as signal, Signal};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend};

pub struct Clock {
    phase: usize
}

// impl Clock {
//     pub fn new() -> (NodeData<BoxedNodeSend>, Vec<String>) {

//     }
// }

impl Node for Clock {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        for i in 0..64 {
            output[0][i] = self.phase;
            self.phase += 1;
        }
        // output[0].iter_mut().for_each(|s| *s = self.sig.next() as f32);
    }
}