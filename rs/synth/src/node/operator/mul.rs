use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};

#[derive(Debug, Copy, Clone)]
pub struct Mul { val: f32 }

impl Mul {
    pub fn new(val: f32) -> Self {
        Self { val }
    }
    impl_to_boxed_nodedata!();
    // pub fn to_boxed_nodedata<const N: usize>(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
    //     NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    // }
}

impl<const N:usize> Node<N> for Mul {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // println!("inputs[0] {:?}", inputs[0].buffers());
        // panic!();
        match inputs.len() {
            1 => {
                
                match inputs[0].buffers().len() {
                    1 => {
                        for i in 0..N {
                            output[0][i] = inputs[0].buffers()[0][i] * self.val;
                        }
                    },
                    2 => {
                        
                        if output.len() < 2 {return ()};
                        for i in 0..N {
                            output[0][i] = inputs[0].buffers()[0][i] * self.val;
                            output[1][i] = inputs[0].buffers()[1][i] * self.val;
                        }
                    },
                    _ => {}
                }
            },
            2 => {
                match inputs[0].buffers().len() {
                    1 => {
                        for i in 0..N {
                            output[0][i] = inputs[0].buffers()[0][i] * inputs[1].buffers()[0][i];
                        }
                    },
                    2 => {
                        if output.len() < 2 {return ()};
                        for i in 0..N {
                            output[0][i] = inputs[0].buffers()[0][i] * inputs[1].buffers()[0][i];
                            output[1][i] = inputs[0].buffers()[1][i] * inputs[1].buffers()[0][i];
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
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