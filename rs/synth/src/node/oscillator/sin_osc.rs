use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
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
        Self {
            freq, ..self
        }
    }
    pub fn sr(self, sr: usize) -> Self {
        Self {
            sr, ..self
        }
    }
    pub fn phase(self, phase: f32) -> Self {
        Self {
            phase, ..self
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for SinOsc {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            0 => {
                for i in 0..N {
                    for j in 0..output.len() {
                        output[j][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                    }
                    self.phase += self.freq / self.sr as f32;
                    if self.phase > 1.0 {
                        self.phase -= 1.0
                    }
                }
            },
            1 => {
                let mod_input = match self.input_order.len() {
                    0 => {
                        &mut *inputs.values_mut().next().unwrap()
                    },
                    _ => {
                        &inputs[&self.input_order[0]]
                    }
                };
                let mod_buf = mod_input.buffers();
                    for i in 0..N {
                        for j in 0..output.len() {
                            output[j][i] = (self.phase * 2.0 * std::f32::consts::PI).sin();
                        }
                        self.phase += mod_buf[0][i] / self.sr as f32;
                        if self.phase > 1.0 {
                            self.phase -= 1.0
                        }
                    }
            }
            _ => return ()
        }
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {self.freq = value},
                    _ => {}
                }
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            Message::ResetOrder => {
                self.input_order.clear();
            },
            _ => {}
        }
    }
}