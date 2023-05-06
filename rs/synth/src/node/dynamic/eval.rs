use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message};
use hashbrown::HashMap;
// use evalexpr::*;
use std::collections::BTreeMap;
use fasteval::Evaler;  // use this trait so we can call eval().
use fasteval::Compiler;  // use this trait so we can call compile().
use fasteval::eval_compiled;

pub struct Eval<const N: usize> {
    pub sr: usize,
    pub bpm: f32,
    phase: usize,
    // code: String,
    // precompiled: evalexpr::Node,
    var: Vec<String>,
    map: BTreeMap<String, f64>,
    compiled: Vec<fasteval::Instruction>,
    parser: fasteval::Parser,
    slab: Vec<fasteval::Slab>,
    input_order: Vec<usize>
}

impl<const N: usize> Eval<N> {

    pub fn new() -> Self {

        let parser = fasteval::Parser::new();
        // let mut slab = fasteval::Slab::new();

        let mut map : BTreeMap<String, f64> = BTreeMap::new();
        map.insert("x".to_string(), 0.0);
        map.insert("y".to_string(), 0.0);
        map.insert("z".to_string(), 0.0);
        map.insert("sr".to_string(), 44100.);

        // let compiled = parser.parse("0.0", &mut slab.ps).unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
    
        Self {
            sr: 44100,
            bpm: 120.,
            phase: 0,
            map,
            parser,
            var: vec![],
            slab: vec![],
            compiled: vec![],
            input_order: Vec::new()
        }
    }

    pub fn sr(mut self, sr: usize) -> Self {
        self.map.insert("sr".to_string(), sr as f64);
        // self.context.set_value("sr".to_owned(), Value::Int(sr as i64)).unwrap();
        Self {sr, ..self}
    }

    pub fn bpm(self, bpm: f32) -> Self {
        Self {bpm, ..self}
    }

    pub fn code(mut self, code: String) -> Self {
        let lines = code.split(";");
        for line in lines {
            let mut slab = fasteval::Slab::new();
            let mut assign = line.split(":=");;
            if line.contains(":=") {
                self.var.push(assign.next().unwrap().to_string().replace(" ", "").replace("\t", "").replace("\n",""));
            }
            let compiled = self.parser.parse(assign.next().unwrap(), &mut slab.ps)
            .unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
            self.slab.push(slab);
            self.compiled.push(compiled);
            
        };
        Self { ..self }
    }

    pub fn to_boxed_nodedata(mut self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
        // self.scope.push("sr", self.sr as f32);
        NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
    }
}

impl<const N:usize> Node<N> for Eval<N> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {

        for i in 0..N {
            if inputs.len() > 0 {    
                self.map.insert(
                    "in".to_owned(),
                    inputs[&self.input_order[0]].buffers()[0][i] as f64
                );
            }
            self.map.insert("phase".to_owned(), self.phase as f64);
            // output[0][i] = self.compiled.eval(&self.slab, &mut self.map).unwrap() as f32;
            // output[0][i] = fasteval::ez_eval(&self.code, &mut self.map).unwrap() as f32;

            for (j, ins) in self.compiled.iter().enumerate() {
                // println!("i is {}", j);
                if j < self.compiled.len()-1 {
                    let v = ins.eval(&self.slab[j], &mut self.map).unwrap();
                    self.map.insert(self.var[j].clone(), v);
                } else {
                    let v = ins.eval(&self.slab[j], &mut self.map).unwrap();
                    output[0][i] = v as f32;
                }
            }
            self.phase += 1;
        }
    }
    
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToSymbol(pos, s) => {
                match pos {
                    0 => {
                        let code = s;
                        let lines = code.split(";");
                        self.slab.clear();
                        self.var.clear();
                        self.compiled.clear();
                        for line in lines {
                            let mut slab = fasteval::Slab::new();
                            let mut assign = line.split(":=");;
                            if line.contains(":=") {
                                self.var.push(assign.next().unwrap().to_string().replace(" ", "").replace("\t", "").replace("\n",""));
                            }
                            let compiled = self.parser.parse(assign.next().unwrap(), &mut slab.ps)
                            .unwrap().from(&slab.ps).compile(&slab.ps, &mut slab.cs);
                            self.slab.push(slab);
                            self.compiled.push(compiled);
                        };
                        // self.code(s);
                        // self.compiled = self.parser.parse(&s, &mut self.slab.ps).unwrap().from(&self.slab.ps)
                        // .compile(&self.slab.ps, &mut self.slab.cs);
                        // self.code = s
                        // self.precompiled = build_operator_tree(&s).unwrap()
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