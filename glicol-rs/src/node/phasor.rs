use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError};

pub struct Phasor {
    step: usize,
    period: usize
}

impl Phasor {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<
    (NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
        let p = paras.as_str();
        let freq = match p.parse::<f32>() {
            Ok(v) => v,
            Err(_w) => 0.0
        };

        let period = (44100.0 / freq ) as usize;
        
        Ok((NodeData::new1(BoxedNodeSend::new( Self {
            step: 0,
            period
        })), vec![]))
    }
}

impl Node for Phasor {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {

        for i in 0..64 {
            let out = self.step % self.period;
            output[0][i] = out as f32 / self.period as f32;
            self.step += 1;
        }
    }
}