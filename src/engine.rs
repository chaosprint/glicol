use std::{collections::HashMap};
// , iter::Cloned, slice::Iter

use pest::Parser;
#[derive(Parser)]
#[grammar = "quaver.pest"]
pub struct QParser;


// add_in_place, 
// use dasp::slice::;
// , Sine, ConstHz, GenMut
// use dasp_signal::{self as signal, Signal};
// use dasp_slice::{ToFrameSlice};
// use dasp_interpolate::linear::Linear;
// {Buffer, Input, Node, {
use dasp_graph::{NodeData, BoxedNodeSend};
// use dasp_signal::{self as signal, Signal, FromIterator,
    // interpolate::linear::Linear, interpolate::Converter};
// use dasp_interpolate::linear::Linear;
// BoxedNode
// use std::f64::consts::PI;

use petgraph;
use petgraph::graph::{NodeIndex};

mod calc_node;
mod osc_node;
mod sampler_node;
mod env_node;


use osc_node::{SinOsc, Impulse};
use calc_node::{Add, Mul};
use sampler_node::{Sampler, Looper};
use env_node::EnvPerc;


pub struct Engine {
    // pub chains: HashMap<String, Vec<Box<dyn Node + 'static + Send >>>,
    pub elapsed_samples: usize,
    pub graph: petgraph::Graph<NodeData<BoxedNodeSend>, (), petgraph::Directed, u32>,
    // pub graph_: Box<petgraph::Graph<NodeData<BoxedNodeSend>, (), petgraph::Directed, u32>>,
    pub processor: dasp_graph::Processor<petgraph::graph::DiGraph<NodeData<BoxedNodeSend>, (), u32>>,
    // pub synth: Synth,
    pub nodes: HashMap<String, NodeIndex>,
    pub samples_dict: HashMap<String, &'static[f32]>,
    // pub nodes_: HashMap<String, NodeIndex>,
    pub sr: u32,
    pub bpm: f64,
    pub code: String,
    pub update: bool,
}

impl Engine {
    pub fn new() -> Engine {
        // Chose a type of graph for audio processing.
        type Graph = petgraph::Graph<NodeData<BoxedNodeSend>, (), petgraph::Directed, u32>;
        // Create a short-hand for our processor type.
        type Processor = dasp_graph::Processor<Graph>;
        // Create a graph and a processor with some suitable capacity to avoid dynamic allocation.
        let max_nodes = 512; // if 1024, error, 512 is fine
        let max_edges = 512;
        let g = Graph::with_capacity(max_nodes, max_edges);
        // let g_ = Graph::with_capacity(max_nodes, max_edges);
        let p = Processor::with_capacity(max_nodes);
        // let box_g = Box::new(g);

        Engine {
            // chains: HashMap::<String, Vec<Box<dyn Node + 'static + Send >>>::new(), 
            // a hashmap of Box<AsFunc>
            graph: g,
            processor: p,
            code: String::from(""),
            samples_dict: HashMap::new(),
            nodes: HashMap::new(),
            elapsed_samples: 0,
            sr: 44100,
            bpm: 120.0,
            update: false,
        }
    }

    fn parse(&mut self) {

        // parse the code
        let lines = QParser::parse(Rule::block, self.code.as_str())
        .expect("unsuccessful parse")
        .next().unwrap();

        // add function to Engine HashMap Function Chain Vec accordingly
        for line in lines.into_inner() {

            let mut ref_name = "~";
            // let mut func_chain = Vec::<Box<dyn Signal<Frame=f64> + 'static + Send>>::new();
            // init Chain
            // match line.as_rule() {
            //     Rule::line => {
            let inner_rules = line.into_inner();
            // let mut func_vec = Vec::<Box<dyn QuaverFunction + 'static + Send>>::new();

            for element in inner_rules {
                match element.as_rule() {
                    Rule::reference => {
                        ref_name = element.as_str();
                    },
                    Rule::chain => {
                        let mut node_vec = Vec::<NodeIndex>::new();
                        for func in element.into_inner() {
                            let mut inner_rules = func.into_inner();
                            let name: &str = inner_rules.next().unwrap().as_str();
                            match name {
                                "sin" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();

                                    // paras -> reference -> string
                                    // paras -> float -> string
                                    let freq: String = paras.next().unwrap().as_str().to_string()
                                    .chars().filter(|c| !c.is_whitespace()).collect();
                                    // let freq = match paras.next().unwrap().as_str().parse::<f64>() {
                                    //     Ok(val) => {val.to_string()},
                                    //     Err(_) => {
                                    //         paras.next().unwrap().into_inner().next().unwrap().as_str().to_string()
                                    //     }
                                    // };
                                    // if !freq.as_str().to_string().parse::<f64>().is_ok() {
                                    //     freq = freq.into_inner().next().unwrap().as_str().to_string();
                                    // }

                                    // parsing 200 will cause error, 200.0 is fine.
                                    // let freq = paras.next().unwrap().as_str().parse::<f64>().unwrap();
                                    // let freq = paras.next().unwrap().as_str().to_string();

                                    let sin_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new(SinOsc::new(freq.clone(), 0.0, 0.0))));
                                    // engine.graph.add_node(
                                        // NodeData::new1(BoxedNodeSend::new( Mul::new(0.5))));
                                    
                                    self.nodes.insert(ref_name.to_string(), sin_node);
                                    node_vec.insert(0, sin_node);

                                    if !freq.parse::<f64>().is_ok() {
                                        assert!(self.nodes.contains_key(freq.as_str()));
                                        if self.nodes.contains_key(freq.as_str()) {
                                            let mod_node = self.nodes[freq.as_str()]; 
                                            self.graph.add_edge(mod_node, sin_node, ());
                                        }                              
                                    }

                                },
                                "mul" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();
                                    let mul: String = paras.next().unwrap().as_str().to_string()
                                    .chars().filter(|c| !c.is_whitespace()).collect();

                                    let mul_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new( Mul::new(mul.clone()))));

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], mul_node, ());
                                    }
                                    
                                    self.nodes.insert(ref_name.to_string(), mul_node);
                                    node_vec.insert(0, mul_node);

                                    // panic if this item not existed
                                    // TODO: move it to a lazy function
                                    // engine.nodes.insert(mul.as_str().to_string(), mul_node);
                                    if !mul.parse::<f64>().is_ok() {
                                        if self.nodes.contains_key(mul.as_str()) {
                                            let mod_node = self.nodes[mul.as_str()]; 
                                            self.graph.add_edge(mod_node, mul_node, ());
                                        }                              
                                    }

                                },
                                "add" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();
                                    let add = paras.next().unwrap().as_str().parse::<f64>().unwrap();
                                    let add_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new( Add::new(add))));

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], add_node, ());
                                    }
                                    
                                    self.nodes.insert(ref_name.to_string(), add_node);
                                    node_vec.insert(0, add_node);
                                    // engine.node.push(mul_node);
                                    // node_vec.push(mul_node);
                                },
                                "loop" => {
                                    let mut events = Vec::<(f64, f64)>::new();

                                    let mut paras = inner_rules
                                    .next().unwrap().into_inner();

                                    let seq = paras.next().unwrap();
                                    let mut compound_index = 0;
                                    let seq_by_space: Vec<pest::iterators::Pair<Rule>> = 
                                    seq.clone().into_inner().collect();

                                    for compound in seq.into_inner() {
                                        let mut shift = 0;
                                        // calculate the length of seq
                                        let compound_vec: Vec<pest::iterators::Pair<Rule>> = 
                                        compound.clone().into_inner().collect();
                
                                        for note in compound.into_inner() {
                                            if note.as_str().parse::<i32>().is_ok() {
                                                let seq_shift = 1.0 / seq_by_space.len() as f64 * 
                                                compound_index as f64;
                                                
                                                let note_shift = 1.0 / compound_vec.len() as f64 *
                                                shift as f64 / seq_by_space.len() as f64;
                
                                                let d = note.as_str().parse::<i32>().unwrap() as f64;
                                                let relative_pitch = 2.0f64.powf((d - 60.0)/12.0);
                                                let relative_time = seq_shift + note_shift;
                                                events.push((relative_time, relative_pitch));
                                            }
                                            shift += 1;
                                        }
                                        compound_index += 1;
                                    }

                                    let looper_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new( Looper::new(events)))
                                    );

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], looper_node, ());
                                    }
                                    
                                    self.nodes.insert(ref_name.to_string(), looper_node);
                                    node_vec.insert(0, looper_node);

                                    // func_chain.functions.push(Box::new(q_loop));
                                },
                                "sampler" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();
                                    let symbol = paras.next().unwrap().as_str();

                                    let sampler_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new(
                                            Sampler::new(self.samples_dict[symbol])))
                                    );

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], sampler_node, ());
                                    }
                                    
                                    self.nodes.insert(ref_name.to_string(), sampler_node);
                                    node_vec.insert(0, sampler_node);
                                },
                                "imp" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();
                                    let imp = paras.next().unwrap().as_str().parse::<f64>().unwrap();
                                    let imp_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new( Impulse::new(imp)))
                                    );

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], imp_node, ());
                                    }
                                    
                                    self.nodes.insert(ref_name.to_string(), imp_node);
                                    node_vec.insert(0, imp_node);
                                },
                                "env_perc" => {
                                    // let mut paras = inner_rules.next().unwrap().into_inner();
                                    let attack = inner_rules.next().unwrap().as_str().parse::<f64>().unwrap();
                                    let decay = inner_rules.next().unwrap().as_str().parse::<f64>().unwrap();
                                    // .unwrap().as_str().parse::<f64>().unwrap();

                                    let env_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new(
                                            EnvPerc::new(attack, decay, 0, 1.0)))
                                    );

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], env_node, ());
                                    }
                                    
                                    self.nodes.insert(ref_name.to_string(), env_node);
                                    node_vec.insert(0, env_node);

                                },
                                _ => unreachable!()
                            }
                            // create the edge here
                            // if node_vec.len() == 2 {
                            //     engine.graph.add_edge(node_vec[0], node_vec[1], ());
                            //     node_vec.clear();
                            // }
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }

    fn generate_wave_buf(&mut self, _size:usize) -> [f32; 128] {
        let mut output: [f32; 128] = [0.0; 128];

        let is_near_bar_end = (self.elapsed_samples + 128) % 88200 < 128  ;

        // may be too time consuming?
        if self.update && is_near_bar_end {
            self.update = false;
            self.nodes.clear();
            self.graph.clear();
            self.parse();
        }

        // (60.0 / self.bpm * 4.0 * 44100.0) as usize
        // we should see if we can update it
        for (ref_name, node) in &self.nodes {
        
            if ref_name.contains("~") {
                self.processor.process(&mut self.graph, *node);
                let b = &self.graph[*node].buffers[0];
                for i in 0..64 {
                    output[i] += b[i];
                    // no clock += 1 here as num of nodes is not fixed
                }
            }
        }

        for (ref_name, node) in &self.nodes {
            
            if ref_name.contains("~") {

                 // this line should be here, otherwise double process fx
                self.processor.process(&mut self.graph, *node);

                let b = &self.graph[*node].buffers[0];
                for i in 64..128 {
                    output[i] += b[i-64]; 
                }
            }
        }

        self.elapsed_samples += 128;

        output
    }

    pub fn process(&mut self, out_ptr: *mut f32, size: usize) {
        let wave_buf = self.generate_wave_buf(size);
        let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
        for i in 0..size {
            out_buf[i] = wave_buf[i] as f32
        }
    }
}