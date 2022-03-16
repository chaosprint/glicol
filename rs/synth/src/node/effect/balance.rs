use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct Balance { input_order: Vec<usize> }

impl Balance {
    pub fn new() -> Self {
        Self {
            input_order: vec![]
        }
    }
    impl_to_boxed_nodedata!();

}

impl<const N:usize> Node<N> for Balance {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("inputs {:?} self.input_order {:?}", inputs, self.input_order);
        // panic!();
        match inputs.len() {
            2 => {
                let left = &inputs[&self.input_order[0]];
                let right = &inputs[&self.input_order[1]];
                output[0] = left.buffers()[0].clone();
                output[1] = right.buffers()[0].clone();
            },
            _ => {}
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}