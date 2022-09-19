use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message};
use freeverb::*;
use hashbrown::HashMap;
use petgraph::graph::NodeIndex;

pub struct Reverb<const N: usize> {
    fv: freeverb::Freeverb, 
    input_order: Vec<usize>,
}

impl<const N: usize> Reverb<N> {
    pub fn new() -> Self {
        let fv = freeverb::Freeverb::new(44100);
        Self {
            fv: fv,
            input_order: Vec::new(),
        }
    }
    pub fn sr(self, sr: usize) -> Self {
        let fv = freeverb::Freeverb::new(sr);
        Self {
            fv, ..self
        }
    }
    pub fn to_boxed_nodedata(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    }
}

impl<const N:usize> Node<N> for Reverb<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // output
        for i in 0..N {
            // pass test
            // output[0][i] = inputs[&self.input_order[0]].buffers()[0][i];
            let out = self.fv.tick((inputs[&self.input_order[0]].buffers()[0][i] as f64, inputs[&self.input_order[0]].buffers()[1][i] as f64));
            output[0][i] = out.0 as f32;
            output[1][i] = out.1 as f32;
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {
                        self.fv.set_dampening(value as f64)
                    },
                    1 => { // set_room_size
                        self.fv.set_room_size(value as f64)
                    },
                    // 2 => {
                    //     self.fv.set_freeze(value as f64)
                    // },
                    2 => {
                        self.fv.set_width(value as f64)
                    },
                    3 => {
                        self.fv.set_wet(value as f64)
                    },
                    4 => {
                        self.fv.set_dry(value as f64)
                    },
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