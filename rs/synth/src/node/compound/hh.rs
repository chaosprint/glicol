// output: noiz 42 >> mul ~env >> hpf 15000 1.0 >> mul 0.8;
// ~env: ~trigger >> envperc 0.001 #decay;
// ~trigger: ~input;
use crate::{
    envelope::EnvPerc, filter::ResonantHighPassFilter, operator::Mul, signal::Noise, AudioContext,
    Pass,
};
use crate::{BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;

use petgraph::graph::NodeIndex;

use super::process_compound;

pub struct Hh<const N: usize> {
    input: NodeIndex,
    context: AudioContext<N>,
    input_order: Vec<usize>,
}

impl<const N: usize> Hh<N> {
    pub fn new(decay: f32) -> Self {
        let mut context = crate::AudioContextBuilder::<N>::new().channels(2).build();
        let input = context.add_mono_node(Pass {});

        let source = context.add_mono_node(Noise::new(42));
        let filter = context.add_mono_node(ResonantHighPassFilter::new().cutoff(15000.));
        let amp = context.add_stereo_node(Mul::new(0.));

        context.chain(vec![source, filter, amp, context.destination]);

        let env_amp = context.add_mono_node(EnvPerc::new().attack(0.003).decay(decay));
        context.tags.insert("d", env_amp);
        context.chain(vec![input, env_amp, amp]);

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

impl<const N: usize> Node<N> for Hh<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        process_compound(inputs, &self.input_order, self.input, &mut self.context, output);
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
