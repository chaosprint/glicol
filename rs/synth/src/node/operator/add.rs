use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, HashMap, ArrayVec, impl_to_boxed_nodedata};

#[derive(Debug, Clone)]
pub struct Add { val: f32, input_order: ArrayVec<usize, 64> }

impl Add {
    pub fn new(val: f32) -> Self {
        Self { val, input_order: ArrayVec::<usize, 64>::new() }
    }
    impl_to_boxed_nodedata!();
    // pub fn to_boxed_nodedata<const N: usize>(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
    //     NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    // }
}

impl<const N:usize> Node<N> for Add {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                let main_input = inputs.values_mut().next().unwrap();
                match main_input.buffers().len() {
                    1 => {
                        for i in 0..N {
                            output[0][i] = main_input.buffers()[0][i] + self.val;
                        }
                    },
                    2 => {
                        
                        if output.len() < 2 {return ()};
                        for i in 0..N {
                            output[0][i] = main_input.buffers()[0][i] + self.val;
                            output[1][i] = main_input.buffers()[1][i] + self.val;
                        }
                    },
                    _ => {}
                }
            },
            2 => {
                let main_input = &inputs[&self.input_order[1]]; // can panic if there is no id
                let ref_input = &inputs[&self.input_order[0]]; // can panic if there is no id
                match main_input.buffers().len() {
                    1 => {
                        for i in 0..N {
                            output[0][i] = main_input.buffers()[0][i] + ref_input.buffers()[0][i];
                        }
                    },
                    2 => {
                        if output.len() < 2 {return ()};
                        for i in 0..N {
                            output[0][i] = main_input.buffers()[0][i] + ref_input.buffers()[0][i];
                            output[1][i] = main_input.buffers()[1][i] + ref_input.buffers()[0][i];
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
            _ => {}
        }
    }
}