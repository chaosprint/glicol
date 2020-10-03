use dasp_graph::{Buffer, Input, Node};
use dasp_signal::{self as signal, Signal};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend};

pub struct Noise {
    sig: Box<dyn Signal<Frame=f64> + Send>
}

impl Noise {
    pub fn new(_paras: &mut Pairs<Rule>) -> (NodeData<BoxedNodeSend>, Vec<String>) {
        // let mut paras = paras.next().unwrap().into_inner();
        let sig = signal::noise(0);
        (NodeData::new1(BoxedNodeSend::new( Self {
            sig: Box::new(sig)
        })), vec![])
    }
}

impl Node for Noise {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        output[0].iter_mut().for_each(|s| *s = self.sig.next() as f32);
    }
}