use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;
// use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Pan {
    pan_pos: f32, // Pan position (-1.0 to 1.0)
    input_order: Vec<usize>,
}

impl Pan {
    pub fn new(pan_pos: f32) -> Self {
        Self {
            pan_pos,
            input_order: vec![],
        }
    }

    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for Pan {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // no modulation
        if inputs.len() == 1 {
            let main_input = inputs.values_mut().next().unwrap();
            let input_buffers = main_input.buffers();

            let pan_norm = (self.pan_pos + 1.0) / 2.0; // Normalize pan position to 0.0 to 1.0
            let left_gain = (1.0 - pan_norm).sqrt(); // Left channel gain
            let right_gain = pan_norm.sqrt(); // Right channel gain

            let (left, right) = output.split_at_mut(1);
            for ((left, right), sample) in left[0].iter_mut().zip(right[1].iter_mut()).zip(input_buffers[0].iter()) {
                *left = left_gain * sample; // Left channel
                *right = right_gain * sample; // Right channel
            }
        } else {
            let ref_input = &inputs[&self.input_order[1]]; // can panic if there is no id
            let main_input = &inputs[&self.input_order[0]]; // can panic if there is no id

            let ref_main = ref_input.buffers()[0].iter().zip(main_input.buffers()[0].iter());
            let (left, right) = output.split_at_mut(1);
            let outs = left[0].iter_mut().zip(right[0].iter_mut());

            for ((left, right), (mod_pan, main)) in outs.zip(ref_main) {
                let pan_norm = (mod_pan + 1.0) / 2.0;
                let left_gain = (1.0 - pan_norm).sqrt(); // Left channel gain
                let right_gain = pan_norm.sqrt(); // Right channel gain
                *left = left_gain * main; // Left channel
                *right = right_gain * main; // Right channel
            }
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            // Clamp pan position within valid range
            Message::SetToNumber(0, value) => self.pan_pos = value.clamp(-1.0, 1.0),
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
