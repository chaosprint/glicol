use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use dasp_ring_buffer as ring_buffer;
type Fixed = ring_buffer::Fixed<Vec<f32>>;

#[derive(Debug, Clone)]
pub struct DelayN {
    buf: Fixed
}

impl DelayN {
    pub fn new(n: usize) -> Self {
        Self {buf: ring_buffer::Fixed::from(vec![0.0; n])}
    }
    impl_to_boxed_nodedata!();
}


impl<const N: usize> Node<N> for DelayN {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                for i in 0..N {
                    output[0][i] = self.buf.push(inputs[0].buffers()[0][i]);
                }
            },
            _ => {return ()}
        }
    }

    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumber(v) => {
                match v.0 {
                    0 => { self.buf.set_first(v.1 as usize) },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}