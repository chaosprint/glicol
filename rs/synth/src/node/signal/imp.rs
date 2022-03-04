use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};

#[derive(Debug, Copy, Clone)]
pub struct Impulse {
    clock: usize,
    period: usize,
    sr: usize
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
        Self {
            sr, ..self
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for Impulse {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        if inputs.len() > 0 {
            self.clock = inputs[0].buffers()[0][0] as usize;
        }
        // println!("processed");
        // for o in output {
        //     o.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        // }
        for i in 0..N {
            let out = (self.clock % self.period == 0) as u8;
            output[0][i] = out as f32;
            self.clock += 1;
        }
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumber(v) => {
                match v.0 {
                    0 => {
                        let period = (self.sr as f32 / v.1) as usize;
                        self.period = period
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}