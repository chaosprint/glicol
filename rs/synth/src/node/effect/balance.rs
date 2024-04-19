use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct Balance {
    input_order: Vec<usize>,
}

impl Default for Balance {
    fn default() -> Self {
        Self::new()
    }
}

impl Balance {
    pub fn new() -> Self {
        Self {
            input_order: vec![],
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for Balance {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("inputs {:?} self.input_order {:?}", inputs, self.input_order);
        // panic!();
        if inputs.len() == 2 {
            let left = &inputs[&self.input_order[0]];
            let right = &inputs[&self.input_order[1]];
            output[0] = left.buffers()[0].clone();
            output[1] = right.buffers()[0].clone();
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
