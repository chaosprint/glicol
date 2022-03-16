// output: saw ~pitch >> mul ~env;
// ~trigger: ~input;
// ~pitch: ~trigger >> mul 261.626;
// ~env: ~trigger >> envperc #attack #decay;
use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message};
use hashbrown::HashMap;
use crate::{
    Pass,
    AudioContext,
    operator::{Mul},
    oscillator::SquOsc,
    envelope::EnvPerc,
};

use petgraph::graph::NodeIndex;

pub struct SquSynth<const N: usize> {
    input: NodeIndex,
    context: AudioContext<N>,
    input_order: Vec<usize>,
}

impl<const N: usize> SquSynth<N> {

    pub fn new(attack: f32, decay: f32) -> Self {
        let mut context = crate::AudioContextBuilder::<N>::new().channels(2).build();
        let input = context.add_mono_node( Pass{} );

        let source = context.add_mono_node( SquOsc::new() );
        let amp = context.add_stereo_node( Mul::new(0.));

        context.chain(vec![source, amp, context.destination]);
        
        let env_amp = context.add_mono_node( EnvPerc::new().attack(attack).decay(decay));
        context.tags.insert("d", env_amp);
        context.chain(vec![input, env_amp, amp]);

        let pitch = context.add_stereo_node( Mul::new(261.63) );
        context.chain(vec![input, pitch, source]);
        
        Self {
            context,
            input,
            input_order: vec![]
        }
    }

    pub fn to_boxed_nodedata(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    }

}

impl<const N: usize> Node<N> for SquSynth<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                let main_input = inputs[&self.input_order[0]].buffers();
                self.context.graph[self.input].buffers[0] = main_input[0].clone();
                // self.context.graph[self.input].buffers[1] = main_input[1].clone();
                let cout = self.context.next_block();
                for i in 0..N {
                    output[0][i] = cout[0][i];
                    output[1][i] = cout[1][i];
                }
            }
            _ => return ()
        }
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumber(pos, value) => {
                match pos {
                    0 => {
                        self.context.graph[self.context.tags["d"]].node.send_msg(Message::SetToNumber(0, value))
                    },
                    1 => {
                        self.context.graph[self.context.tags["d"]].node.send_msg(Message::SetToNumber(1, value))
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