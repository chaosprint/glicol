use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use hashbrown::HashMap;
#[derive(Debug, Clone)]
pub struct Add { val: f32, input_order: Vec::<usize> }

impl Add {
    pub fn new(val: f32) -> Self {
        Self { val, input_order: Vec::<usize>::new() }
    }
    impl_to_boxed_nodedata!();
    // pub fn to_boxed_nodedata<const N: usize>(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
    //     NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    // }
}

impl<const N:usize> Node<N> for Add {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // println!("inputs of add {:?} {}", inputs, inputs.len());
        match inputs.len() {
            1 => {
                let main_input = inputs.values_mut().next().unwrap();
                match main_input.buffers().len() {
                    1 => {              
                        for i in 0..N {
                            output[0][i] = main_input.buffers()[0][i] + self.val;
                            if output.len() > 1 {
                                
                                output[1][i] = main_input.buffers()[0][i]  + self.val;
                            }
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
                let ref_input = &inputs[&self.input_order[1]];
                let main_input = &inputs[&self.input_order[0]];
                match main_input.buffers().len() {
                    1 => {
                        for i in 0..N {
                            output[0][i] = main_input.buffers()[0][i] + ref_input.buffers()[0][i];
                            if output.len() > 1 {
                                output[1][i] = main_input.buffers()[0][i] + ref_input.buffers()[0][i];
                            }
                        }
                    },
                    2 => {
                        if output.len() < 2 {return ()};
                        for i in 0..N {
                            output[0][i] = main_input.buffers()[0][i] + ref_input.buffers()[0][i];
                            if output.len() > 1 {
                                output[1][i] = main_input.buffers()[1][i] + match ref_input.buffers().len(){
                                    1 => { ref_input.buffers()[0][i]},
                                    _ => { ref_input.buffers()[1][i]},
                                }
                            }
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
            Message::ResetOrder => {
                self.input_order.clear();
            },
            _ => {}
        }
    }
}