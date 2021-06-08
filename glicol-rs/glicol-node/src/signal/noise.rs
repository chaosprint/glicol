use dasp_graph::{Buffer, Input, Node};
use dasp_signal::{self as signal, Signal};
use super::super::super::{ GlicolNodeData, mono_node, NodeData, BoxedNodeSend};

pub struct Noise {
    sig: Box<dyn Signal<Frame=f64> + Send>
}

impl Noise {
    pub fn new(seed: u64) -> GlicolNodeData {
        mono_node! ( Self {
            sig: Box::new(signal::noise(seed as u64))
        })
    }
}


impl Node<128> for Noise {
    fn process(&mut self, _inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        output[0].iter_mut().for_each(|s| *s = self.sig.next() as f32);
    }
}