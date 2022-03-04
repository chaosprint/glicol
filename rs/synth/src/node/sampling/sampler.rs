use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};

#[derive(Debug, Clone)]
pub struct Sampler {
    playback: Vec<(usize, f64)>,
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
        output[0].silence();
        match inputs.len() {
            1 => {
                let input_buf = &mut inputs[0].buffers();
                for i in 0..N {
                    if input_buf[0][i] > 0.0 {
                        let dur = self.len as f64 / input_buf[0][i] as f64;
                        self.playback.push((self.clock, dur));
                    }
                    for (begin, dur) in &self.playback {
                        let pos = (self.clock - begin) as f64 / dur;
                        if pos <= 1.0 {
                            output[0][i] += match pos {
                                x if x == 0.0 => self.sample.0[0],
                                x if x == 1.0 => self.sample.0[self.endindex],
                                x if x > 0.0 && x < 1.0 => {
                                    let left = (x * (self.endindex as f64)).floor();
                                    let right = (x * (self.endindex as f64)).ceil();
                                    (self.sample.0[left as usize] as f64
                                    * ((x * (self.endindex as f64)) - left)
                                    + self.sample.0[right as usize] as f64
                                    * (right - (x * (self.endindex as f64)))) as f32
                                },
                                _ => 0.0
                            };
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