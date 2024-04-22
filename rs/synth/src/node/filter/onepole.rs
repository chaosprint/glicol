use crate::{impl_to_boxed_nodedata, BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct OnePole {
    pub a: f32,
    pub b: f32,
    y1: f32,
    input_order: Vec<usize>,
}

impl OnePole {
    pub fn new(rate: f32) -> Self {
        let b = (-2.0 * std::f32::consts::PI * rate).exp();
        let a = 1.0 - b;
        Self {
            a,
            b,
            y1: 0.0,
            input_order: vec![],
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for OnePole {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("inputs[1] {:?}", inputs[1].buffers());
        match inputs.len() {
            1 => {
                let main_input = inputs.values_mut().next().unwrap();
                for (out, main_in) in output[0].iter_mut().zip(main_input.buffers()[0].iter()) {
                    let y = main_in * self.a + self.b * self.y1;
                    *out = y;
                    self.y1 = y;
                }
            }
            2 => {
                let main_input = &inputs[&self.input_order[0]]; // can panic if there is no id
                let ref_input = &inputs[&self.input_order[1]]; // can panic if there is no id

                for ((out, main_in), ref_in) in output[0].iter_mut()
                    .zip(main_input.buffers()[0].iter())
                    .zip(ref_input.buffers()[0].iter())
                {
                    self.b = (-2.0 * std::f32::consts::PI * main_in).exp();
                    self.a = 1. - self.b;
                    let y = ref_in * self.a + self.b * self.y1;
                    *out = y;
                    self.y1 = y;
                }
            }
            _ => (),
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(0, value) => {
                self.b = (-2.0 * std::f32::consts::PI * value).exp();
                self.a = 1. - self.b
            },
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            Message::ResetOrder => {
                self.input_order.clear();
            }
            _ => {}
        }
    }
}
