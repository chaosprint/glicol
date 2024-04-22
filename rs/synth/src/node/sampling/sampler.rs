use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Sampler {
    playback: Vec<(usize, f32)>,
    pub sample: (&'static [f32], usize, usize),
    len: usize,
    endindex: usize,
    clock: usize,
    sr: usize,
    input_order: Vec<usize>,
}

impl Sampler {
    pub fn new(sample: (&'static [f32], usize, usize), sr: usize) -> Self {
        Self {
            playback: vec![],
            sample,
            len: sample.0.len() / sample.1, // total_length/channels
            endindex: sample.0.len() - 1,
            clock: 0,
            sr,
            input_order: vec![],
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for Sampler {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        output[0].silence();
        output[1].silence();
        if inputs.len() == 1 {
            let main_input = inputs.values_mut().next().unwrap();
            let input_buf = &mut main_input.buffers();
            for i in 0..N {
                if input_buf[0][i] > 0.0 {
                    let dur = self.len as f32
                        / input_buf[0][i]
                        / (self.sample.2 as f32 / self.sr as f32);
                    self.playback.push((self.clock, dur));
                }

                let mut to_remove = vec![];
                for (count, (begin, dur)) in self.playback.iter().enumerate() {
                    let pos = (self.clock - begin) as f32 / dur;
                    if pos <= 1.0 {
                        match self.sample.1 {
                            1 => {
                                output[0][i] += match pos {
                                    0.0 => self.sample.0[0],
                                    1.0 => self.sample.0[self.endindex],
                                    // this match arm guard is technically not correct but since
                                    // we've already guaranteed that it can't be 0.0 or 1.0, it's
                                    // fine
                                    0.0..=1.0 => {
                                        let pos_index_float = pos * (self.endindex as f32);
                                        let left = pos_index_float.floor();
                                        let right = pos_index_float.ceil();
                                        let left_portion = pos_index_float - left;
                                        let right_portion = 1. - left_portion;

                                        self.sample.0[left as usize] * left_portion
                                            + self.sample.0[right as usize] * right_portion
                                    }
                                    _ => 0.0,
                                };
                                output[1][i] = output[0][i];
                            }
                            2 => match pos {
                                0.0 => {
                                    output[0][i] += self.sample.0[0];
                                    output[1][i] += self.sample.0[self.len];
                                }
                                1.0 => {
                                    output[0][i] += self.sample.0[self.len - 1];
                                    output[1][i] += self.sample.0[self.endindex];
                                }
                                // same comment here about match arms as above
                                0.0..=1.0 => {
                                    let pos_index_float = pos * ((self.len - 2) as f32);
                                    let left = pos_index_float.floor();
                                    let right = pos_index_float.ceil();
                                    let left_portion = pos_index_float - left;
                                    let right_portion = 1. - left_portion;

                                    output[0][i] += self.sample.0[left as usize]
                                        * left_portion
                                        + self.sample.0[right as usize] * right_portion;

                                    output[1][i] += self.sample.0
                                        [left as usize + self.len + 1]
                                        * left_portion
                                        + self.sample.0[right as usize + self.len + 1]
                                            * right_portion;
                                }
                                _ => {}
                            }
                            _ => (),
                        }
                    } else {
                        // panic!();
                        to_remove.push(count)
                    }
                }
                for c in to_remove.iter().rev() {
                    self.playback.remove(*c);
                }
                // if self.playback.len() > 10 {
                //     panic!("too much playback")
                // }

                self.clock += 1;
            }
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToSamples(0, sample) => {
                self.sample = sample;
                self.len = sample.0.len() / sample.1;
                self.endindex = sample.0.len() - 1;
                // self.playback.clear();
            }
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
