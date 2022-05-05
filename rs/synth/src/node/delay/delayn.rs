use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
use dasp_ring_buffer as ring_buffer;
type Fixed = ring_buffer::Fixed<Vec<f32>>;

#[derive(Debug, Clone)]
pub struct DelayN {
    buf: Fixed,
    delay_n: usize,
    input_order: Vec<usize>
}

impl DelayN {
    pub fn new(n: usize) -> Self {    
        let delay_n = n;
        let init_n = match n {
            0 => 1,
            _ => n
        };
        let buf = ring_buffer::Fixed::from(vec![0.0; init_n]);
        Self {buf, delay_n, input_order: Vec::new()}
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for DelayN {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                let main_input = inputs.values_mut().next().unwrap();
                if self.delay_n != 0 {
                    for i in 0..N {
                        output[0][i] = self.buf.push(main_input.buffers()[0][i]);
                        if main_input.buffers().len() == 1 && output.len() == 2 {
                            output[1][i] = output[0][i];
                        }
                    }
                } else {
                    // same as Pass node
                    let input = match inputs.values().next() {
                        None => return,
                        Some(input) => input,
                    };
                    if input.buffers().len() == 1 && output.len() == 2 {
                        output[0].copy_from_slice(&input.buffers()[0]);
                        output[1].copy_from_slice(&input.buffers()[0]);
                    } else {
                        for (out_buf, in_buf) in output.iter_mut().zip(input.buffers()) {
                            out_buf.copy_from_slice(in_buf);
                        }
                    }
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
                        self.delay_n = value as usize;
                        self.buf = Fixed::from(vec![0.0; self.delay_n]);
                        // buf2 = Fixed::from(vec![0.0; delay_n]);
                        // self.buf.set_first(self.delay_n);
                        // self.buf2.set_first(delay_n);
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
            Message::ResetOrder => {
                self.input_order.clear();
            },
            _ => {}
        }
    }
}