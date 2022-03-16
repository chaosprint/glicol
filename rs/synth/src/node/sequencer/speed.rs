use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Speed { val: f32, input_order: Vec<usize> }

impl Speed {
    pub fn new(val: f32) -> Self {
        Self {val, input_order: Vec::new()}
    }
    impl_to_boxed_nodedata!();
}

impl< const N: usize> Node<N> for Speed {
    fn process(&mut self, _inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        output[0].silence();
        output[0][0] = self.val;
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {self.val = value},
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