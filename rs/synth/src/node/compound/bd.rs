// ~env: imp 1 >> envperc 0.003 0.3

// ~p: ~ep >> mul 50 >> add 60

// o: sin ~p >> mul ~env

// ~ep: imp 1 >> envperc 0.01 0.1;

use crate::{
    envelope::EnvPerc,
    operator::{Add, Mul},
    oscillator::SinOsc,
    AudioContext, Pass,
};
use crate::{BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;

use petgraph::graph::NodeIndex;

use super::process_compound;

pub struct Bd<const N: usize> {
    input: NodeIndex,
    context: AudioContext<N>,
    input_order: Vec<usize>,
}

impl<const N: usize> Bd<N> {
    pub fn new(decay: f32) -> Self {
        let mut context = crate::AudioContextBuilder::<N>::new().channels(2).build();
        let input = context.add_mono_node(Pass {});
        let env_amp = context.add_mono_node(EnvPerc::new().attack(0.003).decay(decay));
        context.tags.insert("d", env_amp);
        let env_pitch = context.add_mono_node(EnvPerc::new().attack(0.01).decay(0.1));
        let mul = context.add_stereo_node(Mul::new(50.));
        let add = context.add_stereo_node(Add::new(60.));
        let sin = context.add_mono_node(SinOsc::new());
        let sin_amp = context.add_mono_node(Mul::new(0.));
        context.chain(vec![sin, sin_amp, context.destination]);
        context.chain(vec![input, env_amp, sin_amp]);
        context.chain(vec![input, env_pitch, mul, add, sin]);

        Self {
            context,
            input,
            input_order: vec![],
        }
    }

    pub fn to_boxed_nodedata(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new(self))
    }
}

impl<const N: usize> Node<N> for Bd<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        process_compound(inputs, &self.input_order, self.input, &mut self.context, output)
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(0, value) =>
                self.context.graph[self.context.tags["d"]]
                    .node
                    .send_msg(Message::SetToNumber(1, value)),
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            _ => {}
        }
    }
}
