use std::{collections::HashMap, num::ParseFloatError};

extern crate pest;
extern crate pest_derive;
use pest::Parser;
use pest::iterators::Pairs;
mod parser;
use parser::*;

use dasp_graph::{NodeData, BoxedNodeSend, Processor};
use petgraph::graph::{NodeIndex, DiGraph};
use petgraph::{Graph, Directed};

mod node;
use node::phasor::{Phasor};
use node::adc::{Adc, AdcSource};
use node::oscillator::{SinOsc, Impulse, Saw, Square};
use node::operator::{Add, Mul};
use node::sampler::{Sampler};
use node::sequencer::{Sequencer, Speed};
use node::envelope::EnvPerc;
use node::noise::Noise;
use node::pass::Pass;
use node::filter::{LPF, HPF};
use node::map::{LinRange};
use node::rand::{Choose};
use node::buf::{Buf};
use node::state::{State};
use node::freeverb::{FreeVerbNode};

mod utili;
use utili::midi_or_float;

pub struct Engine {
    pub elapsed_samples: usize,
    pub graph: Graph<NodeData<BoxedNodeSend>, (), Directed, u32>,
    processor: Processor<DiGraph<NodeData<BoxedNodeSend>, (), u32>>,
    sidechains_list: Vec<(NodeIndex, String)>,
    pub adc_source_nodes: Vec<NodeIndex>,
    pub adc_nodes: Vec<NodeIndex>,
    pub audio_nodes: HashMap<String, NodeIndex>,
    pub control_nodes: HashMap<String, NodeIndex>,
    pub samples_dict: HashMap<String, &'static[f32]>,
    pub sr: u32,
    pub bpm: f64,
    code: &'static str,
    code_backup: &'static str,
    update: bool,
}

impl Engine {
    pub fn new() -> Engine {
        // Chose a type of graph for audio processing.
        type MyGraph = Graph<NodeData<BoxedNodeSend>, (), Directed, u32>;
        // Create a short-hand for our processor type.
        type MyProcessor = Processor<MyGraph>;

        // Create a graph and a processor with some
        // suitable capacity to avoid dynamic allocation
        // if 1024, error in wasm, 512 is fine
        let max_nodes = 512; 
        let max_edges = 512;
        let g = MyGraph::with_capacity(max_nodes, max_edges);
        let p = MyProcessor::with_capacity(max_nodes);

        Engine {
            graph: g,
            processor: p,
            code: "",
            code_backup: "",
            samples_dict: HashMap::new(),
            adc_source_nodes: Vec::new(),
            adc_nodes: Vec::new(),
            sidechains_list: Vec::new(),
            audio_nodes: HashMap::new(),
            control_nodes: HashMap::new(),
            elapsed_samples: 0,
            sr: 44100,
            bpm: 120.0,
            update: false,
        }
    }

    pub fn reset(&mut self) {
        self.elapsed_samples = 0;
        self.update = false;
        self.code = "";
        self.sidechains_list.clear();
        self.control_nodes.clear();
        self.audio_nodes.clear();
        self.graph.clear();
    }

    pub fn update(&mut self) {
        self.update = true;
    }

    pub fn set_code(&mut self, code: &'static str) {
        self.code = code;
    }

    // error only comes from this method
    pub fn make_graph(&mut self) -> Result<(), EngineError>{
        self.audio_nodes.clear();
        self.control_nodes.clear();
        self.graph.clear();
        self.sidechains_list.clear();

        let lines = GlicolParser::parse(Rule::block, self.code)
        .expect("unsuccessful parse")
        .next().unwrap();

        let mut previous_nodes = Vec::<NodeIndex>::new();
        let mut current_ref_name: &'static str = "";

        // add function to Engine HashMap Function Chain Vec accordingly
        for line in lines.into_inner() {

            // self.ref_name;
            let inner_rules = line.into_inner();
            for element in inner_rules {
                match element.as_rule() {
                    Rule::reference => {
                        current_ref_name = element.as_str();
                    },
                    Rule::chain => {
                        previous_nodes.clear();
                        // change name to previous_nodes

                        for func in element.into_inner() {
                            let mut paras = func.into_inner();
                            let p = paras.next().unwrap();
                            // println!("{} {}", );
                            // let pos = (p.as_span().start(), p.as_span().end());
                            let name: &str = p.as_str();

                            // println!("{:?}", p.as_rule());
                            let mut dest = "".to_string();

                            if p.as_rule() == Rule::paras {
                                dest = format!("@rev{}", p.as_str());
                            }

                            let (node_data, sidechains) = match name {
                                "sin" => SinOsc::new(&mut paras)?,
                                "mul" => Mul::new(&mut paras)?,
                                "add" => Add::new(&mut paras)?,
                                "linrange" => LinRange::new(&mut paras)?,
                                "imp" => Impulse::new(&mut paras)?,
                                "sampler" => Sampler::new(&mut paras, &self.samples_dict)?,
                                "seq" => Sequencer::new(&mut paras)?,
                                "saw" => Saw::new(&mut paras)?,
                                "squ" => Square::new(&mut paras)?,
                                "lpf" => LPF::new(&mut paras)?,
                                "hpf" => HPF::new(&mut paras)?,
                                "speed" => Speed::new(&mut paras)?,
                                "noiz" => Noise::new(&mut paras)?,
                                "choose" => Choose::new(&mut paras)?,
                                "envperc" => EnvPerc::new(&mut paras)?,
                                "pha" => Phasor::new(&mut paras)?,
                                "buf" => Buf::new(&mut paras, &self.samples_dict)?,
                                "state" => State::new(&mut paras)?,
                                "freeverb" => FreeVerbNode::new(&mut paras)?,
                                _ => Pass::new(name)?
                            };
                    
                            let node_index = self.graph.add_node(node_data);
                    
                            // connect to previous node, or redirect the previous node to a control node
                            if previous_nodes.len() > 0 {
                                if dest != "" {
                                    // println!("{}", dest);
                                    self.sidechains_list.push((previous_nodes[0], dest));
                                } else {
                                    self.graph.add_edge(previous_nodes[0], node_index, ());
                                }
                            }
                    
                            // only process the last nodes of chains in the audio nodes vec
                            if !current_ref_name.contains("~") {
                                self.audio_nodes.insert(current_ref_name.to_string(), node_index);
                                self.control_nodes.insert(current_ref_name.to_string(), node_index);
                            } else {
                                self.control_nodes.insert(current_ref_name.to_string(), node_index);
                            }
                    
                            // prepare to be connected by the next node of the chain
                            previous_nodes.insert(0, node_index);
                    
                            // lazy sidechain connection
                            for sidechain in sidechains.into_iter() {
                                self.sidechains_list.push((node_index, sidechain));
                            };
                        }
                    },
                    _ => ()
                }
            }
        }

        // here all nodes are processed, we create lazy edge connection
        for pair in &self.sidechains_list {
            // assert!(self.control_nodes.contains_key(&pair.1), 
            // "no such a control node");

            if pair.1.contains("@rev") {
                // let name: Vec<&str> = pair.1.split("@rev").collect();
                let name = &pair.1[4..];
                if !self.control_nodes.contains_key(name) {
                    return Err(EngineError::NonExistControlNodeError);
                }
                let control_node = self.control_nodes[name];
                self.graph.add_edge(pair.0, control_node, ());
            } else {
                if !self.control_nodes.contains_key(&pair.1) {
                    return Err(EngineError::NonExistControlNodeError);
                }
                let control_node = self.control_nodes[&pair.1];
                self.graph.add_edge(control_node, pair.0, ()); // the order matters
            }
        };

        Ok(())
    }

    // for bela
    pub fn make_adc_node(&mut self, chan:usize) {
        for _ in 0..chan {
            let index = self.graph.add_node(
                NodeData::new1( BoxedNodeSend::new( Adc {} ) )
            );

            self.adc_nodes.push(index);
            let source = self.graph.add_node( 
                NodeData::new1( BoxedNodeSend::new( AdcSource {} ) )
            );

            self.adc_source_nodes.push(source);
            self.graph.add_edge(source, index, ());
        }
    }

    pub fn set_adc_node_buffer(&mut self, buf: &[f32], chan: usize,
        frame: usize, _interleave: bool) {
        // , _chan: u8, _frame: u16, _interleave: bool
        for c in 0..chan {
            for f in 0..frame {
                self.graph[
                    self.adc_source_nodes[c]
                ].buffers[0][f] = buf[c*frame+f];
            }
        }
    }

    pub fn gen_next_buf_64(&mut self) -> [f32; 64] {
        
        // using self.buffer will cause errors on bela
        let mut output: [f32; 64] = [0.0; 64];
        for (_ref_name, node) in &self.audio_nodes {

            // find the edge order issue
            // print!("this should before process {:?}", 
            // self.graph.raw_edges());
            // if self.graph.raw_edges().len() > 0 {f
            self.processor.process(&mut self.graph, *node);
            let b = &self.graph[*node].buffers[0];
            for i in 0..64 {
                output[i] += b[i];
                }
            // }
        }
        self.elapsed_samples += 64;
        output
    }

    pub fn gen_next_buf_128(&mut self) -> Result<([f32; 128], [u8;256]), EngineError> {
        // you just cannot use self.buffer
        let mut output: [f32; 128] = [0.0; 128];
        let mut console: [u8;256] = [0; 256];

        let is_near_bar_end = (self.elapsed_samples + 128) % 88200 < 128;
        
        // for wasm live coding
        if self.update && is_near_bar_end {
            self.update = false;

            match self.make_graph() {
                Ok(_) => {
                    self.code_backup = self.code;
                },
                Err(e) => {
                    // println!("{:?}", e);
                    console = match e {
                        EngineError::SampleNotExistError((s, e)) => {
                            let mut info: [u8; 256] = [0; 256];
                            let l = self.code.clone()[..s].matches("\n").count() as u8;
                            info[0] = 1;
                            info[1] = l;
                            // println!("{}", self.code);
                            let word = self.code[s..e].as_bytes();
                            for i in 2..word.len()+2 {
                                info[i] = word[i-2]
                            }
                            info   
                        }
                        _ => unimplemented!()
                    };
                    self.code = self.code_backup;
                    // state = e as usize + 1;
                    // get where the error is
                    // also which kind of error it is
                    self.make_graph()?; // this should be fine
                }
            }
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
            for i in 0..64 {
                output[i+64] += b[i]; 
            }
        }
        self.elapsed_samples += 128;
        Ok((output, console))
    }
}

#[derive(Debug)]
pub enum EngineError {
    NonExistControlNodeError,
    HandleNodeError,
    ParameterError,
    SampleNotExistError((usize, usize))
}

impl std::convert::From<ParseFloatError> for EngineError {
    fn from(_error: ParseFloatError) -> Self {
        EngineError::ParameterError
    }
}