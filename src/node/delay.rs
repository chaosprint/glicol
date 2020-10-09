use dasp_graph::{Buffer, Input, Node};
use dasp_signal::{self as signal, Signal};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend};

pub struct Delay {
    sig: Box<dyn Signal<Frame=f64> + Send>
}

impl Delay {
    pub fn new(paras: &mut Pairs<Rule>) -> (NodeData<BoxedNodeSend>, Vec<String>) {
        let mut delay = paras.as_str();
        
        // let sig = signal::noise(0);
        (NodeData::new1(BoxedNodeSend::new( Self {
            sig: Box::new(sig)
        })), vec![])
    }
}

impl Node for Delay {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        output[0].iter_mut().for_each(|s| *s = self.sig.next() as f32);
    }
}