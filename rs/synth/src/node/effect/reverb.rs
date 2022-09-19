use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message};
use freeverb::*;
use hashbrown::HashMap;
use petgraph::graph::NodeIndex;

pub struct FreeverbNode<const N: usize> {
    fv: freeverb::Freeverb, 
    input_order: Vec<usize>,
}

impl<const N: usize> FreeverbNode<N> {
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

impl<const N:usize> Node<N> for FreeverbNode<N> {
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