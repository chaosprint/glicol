use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, HashMap, AudioContext,
    oscillator::{SinOsc}, filter::{ OnePole, AllPassFilterGain}, effect::Balance,
    operator::{Mul, Add}, delay::{DelayN, DelayMs}, node::Pass
};

use petgraph::graph::NodeIndex;

pub struct Plate<const N: usize> { 
    input: NodeIndex,
    context: AudioContext<N>,
    input_order: Vec<usize>
}

impl<const N: usize> Plate<N> {
    pub fn new(mix: f32) -> Self {

        let mut context = crate::AudioContextBuilder::<N>::new().channels(2).build();
        
        let input = context.add_mono_node( Pass{} );
        let wet1 = context.add_mono_node(OnePole::new(0.7));
        let wet2 = context.add_mono_node(DelayMs::new().delay(50.));
        let wet3 = context.add_mono_node(AllPassFilterGain::new().delay(4.771).gain(0.75));
        let wet4 = context.add_mono_node(AllPassFilterGain::new().delay(3.595).gain(0.75));
        let wet5 = context.add_mono_node(AllPassFilterGain::new().delay(12.72).gain(0.625));
        let wet6 = context.add_mono_node(AllPassFilterGain::new().delay(9.307).gain(0.625));
        let wet7 = context.add_mono_node(Add::new(0.0)); // fb here
        let wet8 = context.add_mono_node(AllPassFilterGain::new().delay(100.0).gain(0.7)); // mod here

        context.chain(vec![input, wet1, wet2, wet3, wet4, wet5, wet6, wet7, wet8]);

        let b1 = context.add_mono_node(SinOsc::new().freq(0.1));
        let b2 = context.add_mono_node(Mul::new(5.5));
        let b3 = context.add_mono_node(Add::new(29.5));
        let _ = context.chain(vec![b1, b2, b3, wet8]);

        let aa = context.add_mono_node(DelayN::new(394));
        context.connect(wet8, aa);
        let ab = context.add_mono_node(DelayN::new(2800));
        context.connect(aa, ab);
        let ac = context.add_mono_node(DelayN::new(1204));
        context.connect(ab, ac);

        let (ba, _) = context.chain_boxed(vec![
            DelayN::new(2000).to_boxed_nodedata(1),
            OnePole::new(0.1).to_boxed_nodedata(1),
            AllPassFilterGain::new().delay(7.596).gain(0.5).to_boxed_nodedata(1),
        ]);
        context.connect(ac, ba[0]);
        let bb = context.add_mono_node(AllPassFilterGain::new().delay(35.78).gain(0.5));
        context.connect(ba[2], bb);
        let bc = context.add_mono_node(AllPassFilterGain::new().delay(100.).gain(0.5));
        context.connect(bb, bc);
        let _ = context.chain(vec![b1, b2, b3, bc]);

        let ca = context.add_mono_node(DelayN::new(179));
        context.connect(bc, ca);
        let cb = context.add_mono_node(DelayN::new(2679));
        context.connect(ca, cb);
        let cc1 = context.add_mono_node(DelayN::new(3500));
        let cc2 = context.add_mono_node(Mul::new(0.3));
        context.chain(vec![cb, cc1, cc2]);
        
        let da1 = context.add_mono_node(AllPassFilterGain::new().delay(30.).gain(0.7));
        let da2 = context.add_mono_node(DelayN::new(522));
        context.chain(vec![cc2, da1, da2]);
        
        let db = context.add_mono_node(DelayN::new(2400));
        context.connect(da2, db);
        let dc = context.add_mono_node(DelayN::new(2400));
        context.connect(db, dc);

        let ea1 = context.add_mono_node(OnePole::new(0.1));
        let ea2 = context.add_mono_node(AllPassFilterGain::new().delay(6.2).gain(0.7));
        context.chain(vec![dc, ea1, ea2]);

        let eb = context.add_mono_node(AllPassFilterGain::new().delay(34.92).gain(0.7));
        context.connect(ea2, eb);

        let fa1 = context.add_mono_node(AllPassFilterGain::new().delay(20.4).gain(0.7));
        let fa2 = context.add_mono_node(DelayN::new(1578));
        context.chain(vec![eb, fa1, fa2]);
        let fb = context.add_mono_node(DelayN::new(2378));
        context.connect(fa2, fb);

        let fb1 = context.add_mono_node(DelayN::new(2500));
        let fb2 = context.add_mono_node(Mul::new(0.3));
        context.chain(vec![fb, fb1, fb2, wet7]); // back to feedback
        
        let left_subtract = context.add_mono_node(crate::node::Sum{});
        context.connect(bb,left_subtract);
        context.connect(db,left_subtract);
        context.connect(ea2,left_subtract);
        context.connect(fa2,left_subtract);
        let left_subtract2 = context.add_mono_node(Mul::new(-1.0));
        context.connect(left_subtract,left_subtract2);
        
        let left = context.add_mono_node(crate::node::Sum{});
        context.connect(aa,left);
        context.connect(ab,left);
        context.connect(cb,left);
        context.connect(left_subtract2,left);
        let left1 = context.add_mono_node(Mul::new(mix));
        context.tags.insert("mix1", left1);
        let left2 = context.add_mono_node(Add::new(0.0)); // input dry * (1.-mix)
        let mixdiffleft = context.add_mono_node(Mul::new(1.-mix));
        context.tags.insert("mixdiff1", mixdiffleft);
        context.chain(vec![input, mixdiffleft, left2]);
        context.chain(vec![left,left1,left2]);
        
        let right_subtract = context.add_mono_node(crate::node::Sum{});
        context.connect(eb,right_subtract);
        context.connect(ab,right_subtract);
        context.connect(ba[2],right_subtract);
        context.connect(ca,right_subtract);
        let right_subtract2 = context.add_mono_node(Mul::new(-1.0));
        context.connect(right_subtract,right_subtract2);

        let right = context.add_mono_node(crate::node::Sum{});
        context.connect(da2,right);
        context.connect(db,right);
        context.connect(fb,right);
        context.connect(right_subtract2,right);
        let right1 = context.add_mono_node(Mul::new(mix));
        context.tags.insert("mix2", right1);
        let right2 = context.add_mono_node(Add::new(0.0)); // input dry * (1.-mix)

        let mixdiff = context.add_mono_node(Mul::new(1.-mix));
        context.tags.insert("mixdiff2", mixdiff);
        context.chain(vec![input, mixdiff, right2]);

        context.chain(vec![right,right1,right2]);
        
        let balance = context.add_stereo_node(Balance::new());
        context.connect(left,balance);
        context.connect(right,balance);
        context.connect(balance, context.destination); 
        Self {
            input,
            context,
            input_order: Vec::new()
        }
    }
    pub fn to_boxed_nodedata(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    }
}

impl<const N:usize> Node<N> for Plate<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        let main_input = inputs[&self.input_order[0]].buffers();
        self.context.graph[self.input].buffers[0] = main_input[0].clone();
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
                        // self.mix = value;
                        self.context.graph[self.context.tags["mix1"]].node.send_msg(Message::SetToNumber(0, value));
                        self.context.graph[self.context.tags["mix2"]].node.send_msg(Message::SetToNumber(0, value));
                        self.context.graph[self.context.tags["mixdiff1"]].node.send_msg(Message::SetToNumber(0, 1.-value));
                        self.context.graph[self.context.tags["mixdiff2"]].node.send_msg(Message::SetToNumber(0, 1.-value));
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