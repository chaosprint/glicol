use crate::{impl_to_boxed_nodedata, oscillator::process_oscillation, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct TriOsc {
    pub freq: f32,
    pub phase: f32,
    pub sr: usize,
    inc: f32,
    input_order: Vec<usize>,
}

impl std::default::Default for TriOsc {
    fn default() -> Self {
        Self {
            freq: 1.0,
            phase: 0.0,
            sr: 44100,
            inc: 0.,
            input_order: vec![],
        }
    }
}

impl TriOsc {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn freq(self, freq: f32) -> Self {
        Self { freq, ..self }
    }
    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self }
    }
    pub fn phase(self, phase: f32) -> Self {
        Self { phase, ..self }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for TriOsc {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        process_oscillation(inputs, &mut self.input_order, output, self.freq, &mut self.inc, |out, freq| {
            let v = -1.0 + (self.phase * 2.);

            *out = 2.0 * (v.abs() - 0.5);
            self.phase += freq / self.sr as f32;

            if self.phase > 1. {
                self.phase -= 1.
            }
        });
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(0, value) => self.freq = value,
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
