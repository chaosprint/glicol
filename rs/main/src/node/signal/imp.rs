use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};
use super::super::super::{GlicolNodeData, mono_node};

pub struct Impulse {
    clock: usize,
    period: usize,
    sr: usize
    // sig: Box<dyn Signal<Frame=f32> + Send>,
    // sig: GenMut<(dyn Signal<Frame=f32> + 'static + Sized), f32>
}

impl Impulse {
    pub fn new() -> Self {
        Self {
            clock: 0,
            period: 44100,
            sr: 44100,
        }
    }
    pub fn freq(self, freq: f32) -> Self {
        let period = (self.sr as f32 / freq) as usize;
        Self {period, ..self}
    }
    pub fn sr(self, sr: usize) -> Self {
        Self {sr, ..self}
    }
    pub fn build(self) -> GlicolNodeData {
        mono_node!(self)
    }
}

impl Node<128> for Impulse {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {

        if inputs.len() > 0 {
            self.clock = inputs[0].buffers()[0][0] as usize;
        }
        // println!("processed");
        // for o in output {
        //     o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        // }
        for i in 0..128 {
            let out = (self.clock % self.period == 0) as u8;
            output[0][i] = out as f32;
            self.clock += 1;
        }
    }
}