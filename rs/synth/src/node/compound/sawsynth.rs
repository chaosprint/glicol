// output: saw ~pitch >> mul ~env;
// ~trigger: ~input;
// ~pitch: ~trigger >> mul 261.626;
// ~env: ~trigger >> envperc #attack #decay;
use crate::{
    envelope::EnvPerc,
    operator::Mul,
    // filter::ResonantHighPassFilter,
    oscillator::SawOsc,
    AudioContext,
    Pass,
};
use crate::{Buffer, Input, Message, Node};
use hashbrown::HashMap;

use petgraph::graph::NodeIndex;

use super::process_compound;

pub struct SawSynth<const N: usize> {
    input: NodeIndex,
    context: AudioContext<N>,
    input_order: Vec<usize>,
}

impl<const N: usize> SawSynth<N> {
    pub fn new(attack: f32, decay: f32) -> Self {
        let mut context = crate::AudioContextBuilder::<N>::new().channels(2).build();
        let input = context.add_mono_node(Pass {});

        let source = context.add_mono_node(SawOsc::new());
        let amp = context.add_stereo_node(Mul::new(0.));

        context.chain(vec![source, amp, context.destination]);

        let env_amp = context.add_mono_node(EnvPerc::new().attack(attack).decay(decay));
        context.tags.insert("env_amp", env_amp);
        context.chain(vec![input, env_amp, amp]);

        let pitch = context.add_stereo_node(Mul::new(261.63));
        context.chain(vec![input, pitch, source]);

        Self {
            context,
            input,
            input_order: vec![],
        }
    }
}

impl<const N: usize> Node<N> for SawSynth<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        process_compound(
            inputs,
            &self.input_order,
            self.input,
            &mut self.context,
            output,
        );
    }

    fn send_msg(&mut self, info: Message) {
        match info {
            Message::SetToNumber(pos, value) => match pos {
                0 => self.context.graph[self.context.tags["env_amp"]]
                    .node
                    .send_msg(Message::SetToNumber(0, value)),
                1 => self.context.graph[self.context.tags["env_amp"]]
                    .node
                    .send_msg(Message::SetToNumber(1, value)),
                _ => {}
            },
            Message::Index(i) => self.input_order.push(i),
            Message::IndexOrder(pos, index) => self.input_order.insert(pos, index),
            _ => {}
        }
    }
}
