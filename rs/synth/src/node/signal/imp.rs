use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Impulse {
    clock: usize,
    period: usize,
    sr: usize,
    input_order: Vec<usize>
}

impl Impulse {
    pub fn new() -> Self {
        Self {
            clock: 0,
            period: 44100,
            sr: 44100,
            input_order: Vec::new()
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
    fn process(&mut self, _inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // if inputs.len() > 0 {
        //     self.clock = inputs[0].buffers()[0][0] as usize;
        // }
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
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {self.period = (self.sr as f32 / value) as usize},
                    _ => {}
                }
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            _ => {}
        }
    }
}