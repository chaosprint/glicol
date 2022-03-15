use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, HashMap, impl_to_boxed_nodedata};
use dasp_ring_buffer as ring_buffer;
type Fixed = ring_buffer::Fixed<Vec<f32>>;

#[derive(Debug, Clone)]
pub struct DelayN2 {
    buf1: Fixed,
    buf2: Fixed,
    input_order: Vec<usize>
}

impl DelayN2 {
    pub fn new(n: usize) -> Self {
        Self {
            buf1: ring_buffer::Fixed::from(vec![0.0; n]),
            buf2: ring_buffer::Fixed::from(vec![0.0; n]), 
            input_order: Vec::new()
        }
    }
    impl_to_boxed_nodedata!();
}


impl<const N: usize> Node<N> for DelayN2 {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                let main_input = inputs.values_mut().next().unwrap();
                for i in 0..N {
                    output[0][i] = self.buf1.push(main_input.buffers()[0][i]);
                    output[1][i] = self.buf2.push(main_input.buffers()[1][i]);
                }
            },
            _ => {return ()}
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {
                        let delay_n = value as usize;
                        // buf = Fixed::from(vec![0.0; delay_n]);
                        // buf2 = Fixed::from(vec![0.0; delay_n]);
                        self.buf1.set_first(delay_n);
                        self.buf2.set_first(delay_n);
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