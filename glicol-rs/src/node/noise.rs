use dasp_graph::{Buffer, Input, Node};
use dasp_signal::{self as signal, Signal};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, NodeResult};

pub struct Noise {
    sig: Box<dyn Signal<Frame=f64> + Send>
}

impl Noise {
    pub fn new(_paras: &mut Pairs<Rule>) ->NodeResult {
        // let mut paras = paras.next().unwrap().into_inner();
        let sig = signal::noise(0);
        Ok((NodeData::new1(BoxedNodeSend::new( Self {
            sig: Box::new(sig)
        })), vec![]))
    }
}

impl Node<128> for Noise {
    fn process(&mut self, _inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        output[0].iter_mut().for_each(|s| *s = self.sig.next() as f32);
    }
}