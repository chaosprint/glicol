use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};

#[derive(Debug, Clone)]
pub struct Sampler {
    playback: Vec<(usize, f32)>,
    pub sample: (&'static[f32], usize),
    len: usize,
    endindex: usize,
    clock: usize,
}

impl Sampler {
    pub fn new(sample: (&'static[f32], usize)) -> Self {
        Self {
            playback: vec![],
            sample,
            len: sample.0.len()/sample.1,
            endindex:  sample.0.len()-1,
            clock: 0
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for Sampler {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // output[0].silence();
        // output[1].silence();
        match inputs.len() {
            1 => {
                let input_buf = &mut inputs[0].buffers();
                for i in 0..N {
                    if input_buf[0][i] > 0.0 {
                        let dur = self.len as f32 / input_buf[0][i] as f32;
                        self.playback.push((self.clock, dur));
                    }
                    for (begin, dur) in &self.playback {
                        let pos = (self.clock - begin) as f32 / dur;
                        if pos <= 1.0 {
                            match self.sample.1 {
                                1 => {
                                    output[0][i] += match pos {
                                        x if x == 0.0 => self.sample.0[0],
                                        x if x == 1.0 => self.sample.0[self.endindex],
                                        x if x > 0.0 && x < 1.0 => {
                                            let pos_index_float = x * (self.endindex as f32);
                                            let left = pos_index_float.floor();
                                            let right = pos_index_float.ceil();
                                            let left_portion = pos_index_float - left;
                                            let right_portion = 1. - left_portion;

                                            self.sample.0[left as usize] * left_portion +
                                            self.sample.0[right as usize] * right_portion
                                        },
                                        _ => 0.0
                                    };
                                    output[1][i] = output[0][i];
                                },
                                2 => {
                                    match pos {
                                        x if x == 0.0 => {
                                            output[0][i] += self.sample.0[0];
                                            output[1][i] += self.sample.0[self.len];
                                        },
                                        x if x == 1.0 => {
                                            output[0][i] += self.sample.0[self.len-1];
                                            output[1][i] += self.sample.0[self.endindex];
                                        },
                                        x if x > 0.0 && x < 1.0 => {
                                            let pos_index_float = x * ((self.len-2) as f32);
                                            let left = pos_index_float.floor();
                                            let right = pos_index_float.ceil();
                                            let left_portion = pos_index_float - left;
                                            let right_portion = 1. - left_portion;

                                            output[0][i] += self.sample.0[left as usize] * left_portion +
                                            self.sample.0[right as usize] * right_portion;
                                            
                                            output[1][i] += self.sample.0[left as usize + self.len + 1] * left_portion
                                            + self.sample.0[right as usize + self.len + 1] * right_portion

                                        },
                                        _ => {}
                                    };
                                },
                                _ => {return ()}
                            }
                            
                        }
                    }
                    self.clock += 1;
                }
            }
            _ => return ()
        }
    }
    fn send_msg(&mut self, _info: Message) {

        // match info {
        //     Message::SetToNumber(v) => {
        //         match v.0 {
        //             0 => {self.freq = v.1},
        //             _ => {}
        //         }
        //     }
        //     _ => {}
        // }
    }
}