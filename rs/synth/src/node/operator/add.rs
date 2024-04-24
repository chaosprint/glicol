use crate::{Buffer, Input, Message, Node};
use hashbrown::HashMap;

use super::apply_op;
#[derive(Debug, Clone)]
pub struct Add {
    val: f32,
    input_order: Vec<usize>,
}

impl Add {
    pub fn new(val: f32) -> Self {
        Self {
            val,
            input_order: Vec::<usize>::new(),
        }
    }
}

impl<const N: usize> Node<N> for Add {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("inputs of add {:?} {}", inputs, inputs.len());
        apply_op(inputs, &self.input_order, output, self.val, std::ops::Add::add)
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
