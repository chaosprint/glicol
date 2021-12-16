use dasp_graph::{Buffer, Input, Node};
use dasp_signal::{self as signal, Signal};
use super::super::{ GlicolNodeData, mono_node, NodeData, BoxedNodeSend};

pub struct Noise<const N:usize> {
    sig: Box<dyn Signal<Frame=f64> + Send>
}

impl<const N:usize> Noise<N> {
    pub fn new(seed: u64) -> GlicolNodeData<N> {
        mono_node! ( N, Self {
            sig: Box::new(signal::noise(seed as u64))
        })
    }
}


impl<const N:usize> Node<N> for Noise<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        output[0].iter_mut().for_each(|s| *s = self.sig.next() as f32);
    }
}
