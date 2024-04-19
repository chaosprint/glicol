// output: sin ~snpitch >> add ~no >> mul ~snenv >> hpf 5000 1.0;
// ~no: noiz 42 >> mul 0.3;
// ~snenv: ~sntriggee >> envperc 0.001 #decay;
// ~snpitch: ~sntriggee >> envperc 0.001 0.1 >> mul 60 >> add 60;
// ~sntriggee: ~input;

use crate::{
    // filter::ResonantHighPassFilter,
    envelope::EnvPerc,
    operator::{Add, Mul},
    oscillator::SinOsc,
    signal::Noise,
    AudioContext,
    Pass,
};
use crate::{BoxedNodeSend, Buffer, Input, Message, Node, NodeData};
use hashbrown::HashMap;

use petgraph::graph::NodeIndex;

use super::process_compound;

pub struct Sn<const N: usize> {
    input: NodeIndex,
    context: AudioContext<N>,
    input_order: Vec<usize>,
}

impl<const N: usize> Sn<N> {
    pub fn new(decay: f32) -> Self {
        let mut context = crate::AudioContextBuilder::<N>::new().channels(2).build();
        let input = context.add_mono_node(Pass {});
        let env_amp = context.add_mono_node(EnvPerc::new().attack(0.001).decay(decay));
        context.tags.insert("d", env_amp);
        let env_pitch = context.add_mono_node(EnvPerc::new().attack(0.001).decay(0.1));
        let mul = context.add_stereo_node(Mul::new(55.));
        let add = context.add_stereo_node(Add::new(60.));

        let sin = context.add_mono_node(SinOsc::new());
        let mix = context.add_stereo_node(Add::new(0.));
        let filter = context.add_stereo_node(Add::new(0.));
        let amp = context.add_stereo_node(Mul::new(0.));

        let noise = context.add_mono_node(Noise::new(42));
        let noise_amp = context.add_stereo_node(Mul::new(0.3));

        context.chain(vec![sin, mix, filter, amp, context.destination]);
        context.chain(vec![noise, noise_amp, mix]);
        context.chain(vec![input, env_amp, amp]);
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

impl<const N: usize> Node<N> for Sn<N> {
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
