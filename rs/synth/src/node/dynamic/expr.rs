use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message};
use hashbrown::HashMap;
use evalexpr::*;

pub struct Expr<const N: usize> {
    sr: usize,
    bpm: f32,
    phase_n: usize,
    precompiled: evalexpr::Node,
    context: HashMapContext,
    input_order: Vec<usize>
}

impl<const N: usize> Expr<N> {

    pub fn new() -> Self {

        let mut context = math_consts_context!(
            PI,
            E
        ).unwrap();

        context.set_value("sr".to_owned(), Value::Int(44100)).unwrap();
        context.set_value("x".to_owned(), Value::Float(0.0)).unwrap();
        context.set_value("y".to_owned(), Value::Float(0.0)).unwrap();
        // context.set_value("z".to_owned(), Value::Float(0.0)).unwrap();

        // let context = HashMapContext::new();

        let precompiled = build_operator_tree("").unwrap();

        Self {
            sr: 44100,
            bpm: 120.,
            phase_n: 0,
            context,
            precompiled,
            input_order: Vec::new()
        }
    }

    pub fn sr(mut self, sr:usize) -> Self {
        self.context.set_value("sr".to_owned(), Value::Int(sr as i64)).unwrap();
        Self {sr, ..self}
    }

    pub fn bpm(self, bpm: f32) -> Self {
        Self {bpm, ..self}
    }

    pub fn code(self, code: String) -> Self {
        let precompiled = build_operator_tree(&code).unwrap();
        Self {precompiled, ..self}
    }

    pub fn to_boxed_nodedata(mut self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
        // self.scope.push("sr", self.sr as f32);
        NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    }
}

impl<const N:usize> Node<N> for Expr<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {

        // self.context.set_value("phase".to_owned(), Value::Float(self.phase_n as f64)).unwrap();
        
        for i in 0..N {
            if inputs.len() > 0 {    
                self.context.set_value("in".to_owned(),
                evalexpr::Value::Float(
                    inputs[&self.input_order[0]].buffers()[0][i] as f64 )).unwrap();
            }
            output[0][i] = self.precompiled.eval_float_with_context_mut(&mut self.context).unwrap() as f32;
            self.phase_n += 1;
        }
    }
    
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToSymbol(pos, s) => {
                match pos {
                    0 => {self.precompiled = build_operator_tree(&s).unwrap()},
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