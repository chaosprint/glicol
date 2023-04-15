use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
use boingboingboing::blep::BLEP;

pub struct BandLimitedSawOsc {
    pub freq: f32,
    pub sr: usize,
    input_order: Vec<usize>,
    pub blep: BLEP,
}

impl std::default::Default for BandLimitedSawOsc {
    fn default() -> Self {
        Self {
            freq: 1.0,
            sr: 44100,
            input_order: vec![],
            blep: boingboingboing::blep(44100)
        }
    }
}

impl BandLimitedSawOsc {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn freq(self, freq: f32) -> Self {
        Self {
            freq: freq, ..self
        }
    }
    pub fn sr(self, sr: usize) -> Self {
        Self {
            blep: boingboingboing::blep(sr), ..self
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for BandLimitedSawOsc {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            0 => {
                self.blep.set_freq(self.freq);
                for i in 0..N {
                    output[0][i] = self.blep.saw();
                }
            },
            1 => {
                    let mod_input =  match self.input_order.len() {
                        0 => {
                            &mut *inputs.values_mut().next().unwrap()
                        },
                        _ => {
                            &inputs[&self.input_order[0]]
                        }
                    };
                    let mod_buf = mod_input.buffers();
                    for i in 0..N {
                        output[0][i] = self.blep.saw();
                        self.blep.set_freq(mod_buf[0][i]);
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
