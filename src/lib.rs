use std::{collections::HashMap};

extern crate pest;
#[macro_use]
extern crate pest_derive;


use pest::Parser;
#[derive(Parser)]
#[grammar = "quaver.pest"]
pub struct QParser;

use dasp_graph::{NodeData, BoxedNodeSend};
use petgraph;
use petgraph::graph::{NodeIndex};

mod node_calc;
mod node_osc;
mod node_sampler;
mod node_env;
mod node_control;

use node_osc::{SinOsc, Impulse};
use node_calc::{Add, Mul};
use node_sampler::{Sampler};
use node_control::{Sequencer, Speed};
use node_env::EnvPerc;

pub struct Engine {
    // pub chains: HashMap<String, Vec<Box<dyn Node + 'static + Send >>>,
    pub elapsed_samples: usize,
    graph: petgraph::Graph<NodeData<BoxedNodeSend>, (), petgraph::Directed, u32>,
    // pub graph_: Box<petgraph::Graph<NodeData<BoxedNodeSend>, (), petgraph::Directed, u32>>,
    processor: dasp_graph::Processor<petgraph::graph::DiGraph<NodeData<BoxedNodeSend>, (), u32>>,
    // pub synth: Synth,
    audio_nodes: HashMap<String, NodeIndex>,
    control_nodes: HashMap<String, NodeIndex>,
    pub samples_dict: HashMap<String, &'static[f32]>,
    // pub nodes_: HashMap<String, NodeIndex>,
    pub sr: u32,
    pub bpm: f64,
    code: String,
    // buffer: [f32; 128],
    update: bool,
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
            code: "".to_string(),
            samples_dict: HashMap::new(),
            audio_nodes: HashMap::new(),
            control_nodes: HashMap::new(),
            elapsed_samples: 0,
            sr: 44100,
            bpm: 120.0,
            update: true,
            // buffer: [0.0; 128],
        }
    }

    pub fn update(&mut self) {
        self.update = true;
    }

    pub fn set_code(&mut self, code: String) {
        self.code = code;
    }

    pub fn parse(&mut self) {

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

                                    let this_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new(SinOsc::new(freq.clone(), 0.0, 0.0))));
    
                                    if ref_name.contains("~") {
                                        self.audio_nodes.insert(ref_name.to_string(), this_node);
                                    } else {
                                        self.control_nodes.insert(ref_name.to_string(), this_node);
                                    }                                
                                    node_vec.insert(0, this_node);

                                    if !freq.parse::<f64>().is_ok() {
                                        // assert!(self.control_nodes.contains_key(freq.as_str()));
                                        if self.control_nodes.contains_key(freq.as_str()) {
                                            let mod_node = self.control_nodes[freq.as_str()]; 
                                            self.graph.add_edge(mod_node, this_node, ());
                                        }                              
                                    }
                                },
                                "mul" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();
                                    let mul: String = paras.next().unwrap().as_str().to_string()
                                    .chars().filter(|c| !c.is_whitespace()).collect();

                                    let this_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new( Mul::new(mul.clone()))));

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], this_node, ());
                                    }
                                
                                    if ref_name.contains("~") {
                                        self.audio_nodes.insert(ref_name.to_string(), this_node);
                                    } else {
                                        self.control_nodes.insert(ref_name.to_string(), this_node);
                                    }
                                    
                                    node_vec.insert(0, this_node);

                                    // panic if this item not existed
                                    // TODO: move it to a lazy function
                                    // engine.nodes.insert(mul.as_str().to_string(), this_node);
                                    if !mul.parse::<f64>().is_ok() {
                                        if self.control_nodes.contains_key(mul.as_str()) {
                                            let mod_node = self.control_nodes[mul.as_str()]; 
                                            self.graph.add_edge(mod_node, this_node, ());
                                        }                              
                                    }
                                },
                                "add" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();
                                    let add = paras.next().unwrap().as_str().parse::<f64>().unwrap();
                                    let this_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new( Add::new(add))));

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], this_node, ());
                                    }
                                    if ref_name.contains("~") {
                                        self.audio_nodes.insert(ref_name.to_string(), this_node);
                                    } else {
                                        self.control_nodes.insert(ref_name.to_string(), this_node);
                                    }                                   
                                    node_vec.insert(0, this_node);
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

                                    let this_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new( Sequencer::new(events, 1.0)))
                                    );

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], this_node, ());
                                    }
                                    self.audio_nodes.insert(ref_name.to_string(), this_node);
                                
                                    node_vec.insert(0, this_node);
                                },
                                "sampler" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();
                                    let symbol = paras.next().unwrap().as_str();

                                    let this_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new(
                                            Sampler::new(self.samples_dict[symbol])))
                                    );

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], this_node, ());
                                    }
                                    self.audio_nodes.insert(ref_name.to_string(), this_node);
                                    
                                    node_vec.insert(0, this_node);
                                },
                                "imp" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();
                                    let imp = paras.next().unwrap().as_str().parse::<f64>().unwrap();
                                    let this_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new( Impulse::new(imp)))
                                    );

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], this_node, ());
                                    }
                                    self.audio_nodes.insert(ref_name.to_string(), this_node);
                                    node_vec.insert(0, this_node);
                                },
                                "speed" => {
                                    let mut paras = inner_rules.next().unwrap().into_inner();
                                    let speed = paras.next().unwrap().as_str().parse::<f32>().unwrap();
                                    let this_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new( Speed {speed: speed } ))
                                    );
                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], this_node, ());
                                    }
                                    
                                    self.audio_nodes.insert(ref_name.to_string(), this_node);
                                    
                                    node_vec.insert(0, this_node);
                                }
                                "env_perc" => {
                                    // let mut paras = inner_rules.next().unwrap().into_inner();
                                    let attack = inner_rules.next().unwrap().as_str().parse::<f64>().unwrap();
                                    let decay = inner_rules.next().unwrap().as_str().parse::<f64>().unwrap();
                                    // .unwrap().as_str().parse::<f64>().unwrap();

                                    let this_node = self.graph.add_node(
                                        NodeData::new1(BoxedNodeSend::new(
                                            EnvPerc::new(attack, decay, 0, 1.0)))
                                    );

                                    if node_vec.len() > 0 {
                                        self.graph.add_edge(node_vec[0], this_node, ());
                                    }
                                    self.audio_nodes.insert(ref_name.to_string(), this_node);
                                    node_vec.insert(0, this_node);

                                },
                                _ => {
                                    if name.contains("&") {
                                        let key: String = name.to_string()
                                        .chars().filter(|c| !c.is_whitespace()).collect();

                                        let this_node = self.audio_nodes[key.as_str()];
                                       
                                        if node_vec.len() > 0 {
                                            self.graph.add_edge(node_vec[0], this_node, ());
                                        }
                                        self.audio_nodes.insert(ref_name.to_string(), this_node);
                                        
                                        node_vec.insert(0, this_node);
                                    }
                                }
                            }
                        }
                    },
                    _ => unreachable!()
                }
            }
        }
    }

    pub fn gen_next_buf_64(&mut self) -> [f32; 64] {
        // you just cannot use self.buffer
        let mut output: [f32; 64] = [0.0; 64];
        for (_ref_name, node) in &self.audio_nodes {
            self.processor.process(&mut self.graph, *node);
            let b = &self.graph[*node].buffers[0];
            for i in 0..64 {
                output[i] += b[i];
            }
        }
        output
    }

    pub fn gen_next_buf_128(&mut self) -> [f32; 128] {
        // you just cannot use self.buffer
        let mut output: [f32; 128] = [0.0; 128];

        let is_near_bar_end = (self.elapsed_samples + 128) % 88200 < 128;
        // may be too time consuming?
        if self.update && is_near_bar_end {
            self.update = false;
            self.audio_nodes.clear();
            self.control_nodes.clear();
            self.graph.clear();
            self.parse();
        }
        for (_ref_name, node) in &self.audio_nodes {
            self.processor.process(&mut self.graph, *node);
            let b = &self.graph[*node].buffers[0];
            for i in 0..64 {
                output[i] += b[i];
            }
        }
        for (_ref_name, node) in &self.audio_nodes {
            self.processor.process(&mut self.graph, *node);
            let b = &self.graph[*node].buffers[0];
            for i in 64..128 {
                output[i] += b[i-64]; 
            }
        }
        self.elapsed_samples += 128;
        output
    }

    // pub fn process(&mut self, out_ptr: *mut f32, size: usize) {
    //     let wave_buf = self.gen_next_buf_128();
    //     let out_buf: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(out_ptr, size) };
    //     for i in 0..size {
    //         out_buf[i] = wave_buf[i] as f32
    //     }
    // }
}