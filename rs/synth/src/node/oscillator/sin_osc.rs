use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct SinOsc {
    pub freq: f32,
    pub phase: f32,
    pub sr: usize,
    input_order: Vec<usize>,
}

impl std::default::Default for SinOsc {
    fn default() -> Self {
        Self {
            freq: 1.0,
            phase: 0.0,
            sr: 44100,
            input_order: vec![],
        }
    }
}

impl SinOsc {
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

impl<const N: usize> Node<N> for SinOsc {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            0 => {
                for i in 0..N {
                    for buf in output.iter_mut() {
                        buf[i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                    }
                    self.phase += self.freq / self.sr as f32;
                    if self.phase > 1.0 {
                        self.phase -= 1.0
                    }
                }
            }
            1 => {
                let mod_input = match *self.input_order {
                    [] => &mut *inputs.values_mut().next().unwrap(),
                    [ref first_input, ..] => &inputs[first_input],
                };

                for (i, mod_buf) in mod_input.buffers()[0].iter().enumerate() {
                    for buf in output.iter_mut() {
                        buf[i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                    }

                    self.phase += mod_buf / self.sr as f32;
                    if self.phase > 1.0 {
                        self.phase -= 1.0
                    }
                }
            }
            _ => (),
        }
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
