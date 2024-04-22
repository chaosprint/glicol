use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;

use super::apply_op;
#[derive(Debug, Clone)]
pub struct Mul {
    val: f32,
    input_order: Vec<usize>,
}

impl Mul {
    pub fn new(val: f32) -> Self {
        Self {
            val,
            input_order: vec![],
        }
    }
    impl_to_boxed_nodedata!();
    // pub fn to_boxed_nodedata<const N: usize>(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
    //     NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    // }
}

impl<const N: usize> Node<N> for Mul {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("inputs {:?} self.input_order {:?}", inputs, self.input_order);
        // panic!();
        apply_op(inputs, &self.input_order, output, self.val, std::ops::Mul::mul);
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(0, value) => self.val = value,
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
