use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;

use dasp_signal::{self as signal, Signal};

pub struct Noise { 
    sig: Box<dyn Signal<Frame=f64> + Send>,
    input_order: Vec<usize> 
}

impl Noise {
    pub fn new( seed: usize ) -> Self {
        Self {
            sig: Box::new(signal::noise(seed as u64)),
            input_order: Vec::new()
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N:usize> Node<N> for Noise {
    fn process(&mut self, _inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        for out in output {
            out.iter_mut().for_each(|s| *s = self.sig.next() as f32);
        }
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {
                        self.sig = Box::new(signal::noise(value as u64));
                    },
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