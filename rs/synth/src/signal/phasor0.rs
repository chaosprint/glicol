use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData,
    NodeResult, BoxedNodeSend, EngineError};

pub struct Phasor<const N:usize> {
    freq: f32,
    phase_n: usize,
    clock: usize,
    buffer: Buffer<N>,
    sr: usize,
}

impl<const N: usize> Phasor<N> {
    pub fn new() -> Self {
        Self {
            freq: 0.01,
            phase_n: 0,
            clock: 0,
            buffer: Buffer::<N>::default(),
            sr: 44100,
        }
    }
    pub fn freq(self, freq: f32) -> Self {
        Self {freq, ..self}
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }

    pub fn build(self) -> GlicolNodeData<N> {
        mono_node! { N, self }
    }
}

    // pub fn new(paras: &mut Pairs<Rule>) -> NodeResult {
    //     let p = paras.as_str();
    //     let freq = match p.parse::<f32>() {
    //         Ok(v) => v,
    //         Err(_w) => 0.0
    //     };

    //     let period = (44100.0 / freq ) as usize;
        
    //     Ok((NodeData::new1(BoxedNodeSend::new( Self {
    //         step: 0,
    //         period
    //     })), vec![]))
    // }

impl Node<128> for Phasor {
    fn process(&mut self, _inputs: &[Input<128>], output: &mut [Buffer<128>]) {

        for i in 0..128 {
            let out = self.step % self.period;
            output[0][i] = out as f32 / self.period as f32;
            self.step += 1;
        }
    }
}