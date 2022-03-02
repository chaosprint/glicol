use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};

#[derive(Debug, Copy, Clone)]
pub struct ConstSig { val: f32 }

impl ConstSig {
    pub fn new(val: f32) -> Self {
        Self {val}
    }
    impl_to_boxed_nodedata!();
}

impl<const N:usize> Node<N> for ConstSig {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = self.val;
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(v) => {
                match v.0 {
                    0 => {self.val = v.1},
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
