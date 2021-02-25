use dasp_graph::{Buffer, Input, Node};
use dasp_signal::{self as signal, Signal};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError, NodeResult};

pub struct Noise {
    sig: Box<dyn Signal<Frame=f64> + Send>
}

impl Noise {
    pub fn new(paras: &mut Pairs<Rule>) ->NodeResult {
        let p = paras.next().unwrap();
        let pos = (p.as_span().start(), p.as_span().end());
        let seed = match p.as_str().parse::<f32>() {
            Ok(v) => v,
            Err(_) => return Err(EngineError::ParameterError(pos))
        };        
        let sig = signal::noise(seed as u64);
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