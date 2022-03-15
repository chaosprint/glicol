use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, HashMap, AudioContext,
    oscillator::{SinOsc}, filter::{ OnePole, AllPassFilterGain}, effect::Balance,
    operator::{Mul, Add}, delay::{DelayN, DelayMs}, node::Pass
};

use petgraph::graph::NodeIndex;

pub struct Reverb<const N: usize> {
    context: AudioContext<N>,
    input_order: Vec<usize>
}

impl<const N: usize> Reverb<N> {
    pub fn new() -> Self {

        let mut context = crate::AudioContextBuilder::<N>::new().channels(2).build();

        let predelay = context.add_mono_node(DelayN::new(1700));
        let a = context.add_stereo_node(AllPassFilterGain::new().delay(32.).gain(0.7));
        
        Self {
            context,
            input_order: Vec::new()
        }
    }
    pub fn to_boxed_nodedata(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    }
}

impl<const N:usize> Node<N> for Reverb<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        let main_input = inputs[&self.input_order[0]].buffers();
        self.context.graph[self.context.input].buffers[0] = main_input[0].clone();
        // self.context.graph[self.input].buffers[1] = main_input[1].clone();
        let cout = self.context.next_block();
        for i in 0..N {
            output[0][i] = cout[0][i];
            output[1][i] = cout[1][i];
        }
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {
                        // self.context.graph[self.context.tags["s"]].node.send_msg(Message::SetToNumber(0, value));
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
            _ => {}
        }
    }
}