use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};

#[derive(Debug, Copy, Clone)]
pub struct OnePole {
    pub a: f32,
    pub b: f32,
    y1: f32
}

impl OnePole {
    pub fn new(rate: f32) -> Self {
        let b = (-2.0 * std::f32::consts::PI * rate).exp();
        let a = 1.0 - b;
        Self{ a, b, y1: 0.0 }
    }
    impl_to_boxed_nodedata!();
}


impl<const N: usize> Node<N> for OnePole {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // println!("inputs[1] {:?}", inputs[1].buffers());
        match inputs.len() {
            1 => {
                for i in 0..N {
                    let y = inputs[0].buffers()[0][i] * self.a + self.b * self.y1;
                    output[0][i] = y;
                    self.y1 = y;
                }
            },
            2 => {
                for i in 0..N {
                   
                    self.b = (-2.0 * std::f32::consts::PI * inputs[0].buffers()[0][i]).exp();
                    self.a = 1. - self.b;
                    let y = inputs[1].buffers()[0][i] * self.a + self.b * self.y1;
                    output[0][i] = y;
                    self.y1 = y;
                }
            },
            _ => {return ()}
        }
    }

    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumber(v) => {
                match v.0 {
                    0 => { self.b = (-2.0 * std::f32::consts::PI * v.1).exp(); self.a = 1. - self.b },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}