//! # Glicol: A Computer Music Language Written in Rust
//! This is the engine of Glicol
//! The engine has two part one is the language and the other is the audio
//! The audio engine can be useful for some real world projects
//! If you are targeting WebAssembly, this can be a useful resource.
// extern dependancies
use std::{collections::HashMap};
use dasp_graph::{NodeData, BoxedNodeSend, Processor, Buffer, Input, Node};
use petgraph::graph::{NodeIndex};
use petgraph::stable_graph::{StableDiGraph};
use pest::Parser;
use pest::iterators::Pairs;

use glicol_node::{oscillator, signal, filter, operation, sampling, effect, pass::*};
use glicol_node::Para;
use glicol_node::make_node;
use glicol_node::signal::dummy::{Clock, AudioIn};
use oscillator::sin_osc::SinOsc;
use oscillator::saw_osc::SawOsc;
use oscillator::squ_osc::SquOsc;
use oscillator::tri_osc::TriOsc;
use signal::imp::*;
use signal::const_sig::ConstSig;
use signal::noise::Noise;
use operation::mul::Mul;
use operation::add::Add;
use filter::lpf::*; use filter::hpf::*; 
use filter::apfgain::*;
use filter::apfdecay::*; 
use filter::onepole::*;
use filter::comb::*;
use sampling::seq::*;
use sampling::sampler::*;
use sampling::speed::*;
use sampling::choose::*;
use effect::delayn::*;
use effect::delay::*;

mod parser;
use parser::*;

mod utili;
use utili::{preprocess_sin, preprocess_mul, lcs, process_error_info};

pub type GlicolNodeData = NodeData<BoxedNodeSend<128>, 128>;
pub type GlicolGraph = StableDiGraph<GlicolNodeData, (), u32>;
pub type GlicolProcessor = Processor<GlicolGraph, 128>;
pub type NodeResult = Result<(GlicolNodeData, Vec<String>), EngineError>;

/// The engine of Glicol
/// This engine can takes Glicol code, convert it to audio graph and process it
/// The engine can also be used independantly as a wrapper to dasp_graph crate (see the examples folder)
pub struct Engine {
    pub elapsed_samples: usize,
    pub graph: GlicolGraph,
    pub processor: GlicolProcessor,
    sidechains_list: Vec<(NodeIndex, String)>,
    pub adc_source_nodes: Vec<NodeIndex>,
    pub adc_nodes: Vec<NodeIndex>,
    pub samples_dict: HashMap<String, &'static[f32]>,
    pub sr: usize,
    pub bpm: f32,
    pub chain_string: HashMap<String, String>,
    pub node_by_chain: HashMap<String, Vec<(NodeIndex, String)>>,
    pub chain_info: HashMap<String, Vec<String>>,
    pub clock: NodeIndex,
    audio_in: NodeIndex,
    pub code: String,
    code_backup: String,
    pub update: bool,
    track_amp: f32,
    pub modified: Vec<String>,
    pub all_refs: Vec<String>, // for always using current code
}

impl Engine {
    pub fn new(sr: usize) -> Self {
        let max_nodes = 1024;
        let max_edges = 1024;
        let g = GlicolGraph::with_capacity(max_nodes, max_edges);
        let p = GlicolProcessor::with_capacity(max_nodes);

        Self {
            graph: g,
            processor: p,
            code: "".to_string(),
            code_backup: "".to_string(),
            samples_dict: HashMap::new(),
            adc_source_nodes: Vec::new(),
            adc_nodes: Vec::new(),
            sidechains_list: Vec::new(),
            chain_string: HashMap::new(),
            node_by_chain: HashMap::new(),
            chain_info: HashMap::new(),
            clock: NodeIndex::new(0),
            audio_in: NodeIndex::new(1),
            sr,
            bpm: 120.0,
            elapsed_samples: 0,
            update: false,
            track_amp: 1.0,
            modified: Vec::new(),
            all_refs: Vec::new(),
        }
    }

    /// The main function to convert the code input string into graph structure inside the engine
    pub fn make_graph(&mut self) -> Result<(), EngineError>{
        // self.node_by_chain.clear();
        self.samples_dict.insert("imp".to_string(), &[1.0]);
        self.graph.clear_edges();
        self.all_refs.clear();
        // self.modified.clear();
        // self.sidechains_list.clear();
        
        // The reason to have a dummy clock is to make sure when using the reference
        // node such as sin is not calculated twice or more
        if self.graph.node_count() < 2 {
            self.clock = self.graph.add_node(
                NodeData::new1(BoxedNodeSend::new(Clock{})));
            self.audio_in = self.graph.add_node(
                NodeData::new1(BoxedNodeSend::new(AudioIn{})));
        }

        // dummy input reference
        self.all_refs.push("_input".to_string());
        self.node_by_chain.insert(
            "_input".to_string(),
            vec![(self.audio_in, "_input".to_string())]
        );

        println!("code before preprocess: {}",&self.code);
        self.code = preprocess_sin(&mut self.code)?;
        self.code = preprocess_mul(&mut self.code)?;
        println!("code after preprocess: {}",&self.code);

        let lines = match GlicolParser::parse(Rule::block, &mut self.code) {
            Ok(mut v) => v.next().unwrap(),
            Err(e) => { println!("{:?}", e); return Err(EngineError::ParsingError(e))}
        };

        // let mut previous_nodes = Vec::<NodeIndex>::new();
        let mut current_ref_name: &str = "";

        // add nodes to nodes chain vectors in the HashMap with ref as key
        for line in lines.into_inner() {
            let inner_rules = line.into_inner();
            for element in inner_rules {
                match element.as_rule() {
                    Rule::reference => {
                        current_ref_name = element.as_str();
                    },
                    Rule::chain => {
                        self.all_refs.push(current_ref_name.to_string());
                        let refname = current_ref_name.to_string();

                        // TODO: this should be solved by parser
                        let new: Vec<String> = element.clone().into_inner()
                        .map(|v|v.as_str().to_string().chars()
                        .filter(|c| !c.is_whitespace()).collect()).collect();
                        // new.reverse();

                        let (add, _rem, del) = match self.chain_info
                        .contains_key(&refname) {

                            true => {
                                let old = self.chain_info[&refname].clone();
                                self.chain_info.insert(refname.clone(), 
                                new.clone());
                                lcs(&old, &new)
                            },
                            _ => {
                                self.chain_info.insert(refname.clone(), 
                                new.clone());
                                let t = Vec::<String>::new();
                                lcs(&t, &new)
                            }
                        };

                        // if (add.len() + del.len()) > 0 {
                        //     self.modified.push(refname.clone());
                        // }

                        for info in &del {
                            let mut i = 0;
                            for nodeinfo in &self.node_by_chain[&refname] {
                                let nodeindex = nodeinfo.0;
                                let nodeid = &nodeinfo.1;
                                if nodeid == info {
                                    self.graph.remove_node(nodeindex);
                                    self.sidechains_list.retain(
                                        |v| v.0 != nodeindex);
                                    break;
                                }
                                i += 1;
                            }
                            let mut list = self.node_by_chain[&refname].clone();
                            list.remove(i);
                            self.node_by_chain.insert(refname.clone(), list);
                        };
                        
                        for func in element.into_inner() {
                            let mut paras = func.into_inner();
                            let id: String = paras.as_str().to_string()
                            .chars().filter(|c| !c.is_whitespace()).collect();
                            let first = paras.next().unwrap();
                            // let pos = (p.as_span().start(), p.as_span().end());
                            let name = first.as_str();
                            let dest = match first.as_rule() {
                                Rule::paras => format!("@rev{}", first.as_str()),
                                _ => "".to_string()
                            };

                            // println!("{:?}", &add);
                            for info in &add {
                                // println!("info {:?} != {:?} ?", &id, &info.0);
                                if info.0 == id {

                                    let (node_data, sidechains) = make_node(
                                        name, &mut paras,
                                        &self.samples_dict,
                                        self.sr,
                                        self.bpm
                                    )?;

                                    let node_index = self.graph.add_node(node_data);
                                    
                                    if !self.node_by_chain.contains_key(&refname) {
                                        // head of chain
                                        self.node_by_chain.insert(refname.clone(),
                                        vec![(node_index, id.clone())]);
                                    } else {

                                        let mut list = self.node_by_chain[&refname]
                                        .clone();

                                        if &dest != "" {
                                            self.sidechains_list.push(
                                                (list.last().unwrap().0, 
                                                dest.clone()));
                                        };                     
                                        list.insert(
                                            info.1, (node_index, id.clone()));

                                        // println!("insert{} at{}",id.clone(),info.1);
                                        self.node_by_chain.insert(
                                            refname.clone(),list);
                                    };

                                    for sidechain in sidechains.into_iter() {
                                        self.sidechains_list.push(
                                            (node_index, sidechain));
                                    };
                                    break;
                                };
                            };
                        }
                    },
                    _ => ()
                }
            }
        }
        // println!("{:?}", self.node_by_chain);

        // for chains that are simply deleted or commented out
        for key in self.node_by_chain.keys() {
            if self.all_refs.contains(key) {
                continue;
            }
            for n in &self.node_by_chain[key] {
                // println!("remove node: {:?} index: {:?}", key, n);
                self.graph.remove_node(n.0);
                self.sidechains_list.retain(|v| v.0 != n.0);
            }
            self.chain_info.remove(key);
            self.chain_string.remove(key);
        }

        let all_refs = self.all_refs.clone();
        self.node_by_chain.retain(|k, _| all_refs.contains(k));

        // println!("connect clock to {:?}", self.node_by_chain);
        // connect clocks to all the nodes
        for (refname, nodes) in &self.node_by_chain {
            if refname != "_input" {
                for n in nodes {
                    self.graph.add_edge(self.clock, n.0,());
                }
            }
        }

        // make edges in each chain
        for (_refname, node_chains) in &self.node_by_chain {
            if node_chains.len() >= 2 {
                self.graph.add_edge(node_chains[0].0,node_chains[1].0,());
                for i in 0..(node_chains.len()-2) {
                    self.graph.add_edge(node_chains[i+1].0,node_chains[i+2].0,());
                };
            };
        }
        
        // make edges cross chain
        for pair in &self.sidechains_list {
            // println!("sidechain conncect {:?}", pair);
            if pair.1.contains("@rev") {
                
                let name = &pair.1[4..];
                // println!("reversed connection for {}", name);
                if !self.node_by_chain.contains_key(name) {
                    return Err(EngineError::NonExistControlNodeError(name.to_string()));
                }
                let control_node = self.node_by_chain[name].last().unwrap().0;
                self.graph.add_edge(pair.0, control_node, ());
            } else {
                if !self.node_by_chain.contains_key(&pair.1) {
                    return Err(EngineError::NonExistControlNodeError(pair.1.to_string()));
                }
                let control_node = self.node_by_chain[&pair.1].last().unwrap().0;
                self.graph.add_edge(control_node, pair.0, ());
            }
        };

        Ok(())
    }

    // TODO: find all modified,
    // pub fn find_modified(&self mut) -> Result<(), EngineError> {
    //     let lines = match GlicolParser::parse(Rule::block, &mut b) {
    //         Ok(mut v) => v.next().unwrap(),
    //         Err(e) => { println!("{:?}", e); return Err(EngineError::ParsingError)}
    //     };
    // }

    // for bela
    // pub fn make_adc_node(&mut self, chan:usize) {
    //     for _ in 0..chan {
    //         let index = self.graph.add_node(
    //             NodeData::new1( BoxedNodeSend::new( Adc {} ) )
    //         );

    //         self.adc_nodes.push(index);
    //         let source = self.graph.add_node( 
    //             NodeData::new1( BoxedNodeSend::new( AdcSource {} ) )
    //         );

    //         self.adc_source_nodes.push(source);
    //         self.graph.add_edge(source, index, ());
    //     }
    // }

    // pub fn set_adc_node_buffer(&mut self, buf: &[f32], chan: usize,
    //     frame: usize, _interleave: bool) {
    //     // , _chan: u8, _frame: u16, _interleave: bool
    //     for c in 0..chan {
    //         for f in 0..frame {
    //             self.graph[
    //                 self.adc_source_nodes[c]
    //             ].buffers[0][f] = buf[c*frame+f];
    //         }
    //     }
    // }

    /// The main interface for WebAssembly module to get the new block of audio.
    pub fn gen_next_buf_128(mut self, inbuf: &mut [f32])
    -> Result<([f32; 256], [u8;256]), EngineError> {
        // don't use self.buffer
        let mut output: [f32; 256] = [0.0; 256];
        let mut console: [u8;256] = [0; 256];
        let one_bar = (240.0 / self.bpm * self.sr as f32) as usize;

        // if self.update && (n + 128 + 2048) % one_bar < 128 {
        //     self.fade = 0;
        // }

        if self.update && (self.elapsed_samples + 128) % one_bar < 128 {
            // println!("updating... at {}", (self.elapsed_samples + 128) % one_bar);
            self.update = false;
            match self.make_graph() {
                Ok(_) => {
                    self.code_backup = self.code.clone();
                },
                Err(e) => {
                    let mut info: [u8; 256] = [0; 256];
                    console = match e {
                        EngineError::SampleNotExistError((s, e)) => { 
                            process_error_info(self.code.clone(), 1, s, e)
                        },
                        EngineError::NonExistControlNodeError(name) => {
                            info[0] = 2;
                            info[1] = 0;//position is not given here
                            let word = name.as_bytes();
                            if word.len() < 254 {
                                for i in 2..word.len()+2 {
                                    info[i] = word[i-2]
                                }
                            } else {
                                for i in 2..256 {
                                    info[i] = word[i-2]
                                }
                            }
                            info
                        },
                        EngineError::ParameterError((s, e)) => {
                            process_error_info(self.code.clone(), 3, s, e)
                        },
                        EngineError::HandleNodeError => {
                            info[0] = 4;
                            info[1] = 0;
                            info
                        },
                        EngineError::ParsingError(_e) => {
                            info[0] = 5;
                            info[1] = 0;
                            if self.code == "" {
                                self.code_backup = "_dummy: const 0.0".to_string();
                            }
                            info
                        },
                        _ => unimplemented!()
                    };
                    println!("debug {:?}", console);
                    self.soft_reset();
                    let backup = self.code_backup.clone();
                    self = self.set_code(&backup);
                    self.make_graph()?;
                }
            }
        }

        // process 0..128
        for (refname, v) in &self.node_by_chain {
            if refname.contains("~") || refname.contains("_") {
                continue;
            }
            self.graph[self.clock].buffers[0][0] = self.elapsed_samples as f32;
            for i in 0..128 {
                self.graph[
                    self.node_by_chain["_input"][0].0
                ].buffers[0][i] = inbuf[i];
            }
            self.processor.process(&mut self.graph, v.last().unwrap().0);
        }

        // sendout 0..128
        for (refname, v) in &self.node_by_chain {
            if refname.contains("~") || refname.contains("_") {
                continue;
            }
            let bufleft = &self.graph[v.last().unwrap().0].buffers[0];
            let bufright = match &self.graph[v.last().unwrap().0].buffers.len() {
                1 => {bufleft},
                2 => {&self.graph[v.last().unwrap().0].buffers[1]},
                _ => {unimplemented!()}
            };

            for i in 0..128 {
                // let s = match self.fade {
                //     k if k > 4095 => 1.0,
                //     _ => self.window[self.fade] as f32 * -1.0 + 1.0
                // };
                // self.fade += 1;
                // let scale = 1.0;bufleft[i] * bufright[i] * 
                output[i] += bufleft[i] * self.track_amp;
                output[i+128] += bufright[i] * self.track_amp;
                // output[i] += s;
                // output[i+128] += s;
            }
        }
        self.elapsed_samples += 128;

        Ok((output, console))
    }

    pub fn soft_reset(&mut self) {
        self.sidechains_list.clear();
        self.node_by_chain.clear();
        self.chain_info.clear();
        self.chain_string.clear();
        self.graph.clear();
    }

    pub fn reset(&mut self) {
        self.elapsed_samples = 0;
        self.update = false;
        self.code = "".to_string();
        self.code_backup = "".to_string();
        self.soft_reset();
    }

    pub fn set_code(self, code: &str) -> Self {
        // self.code = code.to_string();
        // self.update = true;
        Self {code: code.to_string(), update: true, ..self}
    }

    pub fn set_track_amp(&mut self, amp: f32) {
        self.track_amp = amp;
    }

    pub fn make_chain(&mut self, mut nodes: Vec<GlicolNodeData>) -> Vec<NodeIndex> {
        let mut indexes = vec![];
        while nodes.len() > 0 {
            let head = nodes.remove(0);
            let index = self.graph.add_node(head);
            let i_len = indexes.len();
            if i_len > 0 {
                self.graph.add_edge(indexes[i_len-1], index, ());
            }
            indexes.push(index);
        }
        indexes
    }

    pub fn make_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, ());
    }

    pub fn process(&mut self, target: NodeIndex) {
        self.processor.process(&mut self.graph, target);
    }

    pub fn next_block(mut self) -> Result<([f32; 256], [u8;256]), EngineError> {
        println!("{}", self.code);
        self.make_graph()?;
        self.gen_next_buf_128(&mut [0.0;128])
    }
}

#[macro_export]
macro_rules! chain {
    ([$($node: expr),*] in $engine:ident) => {
        $engine.make_chain(vec![$($node,)*])   
    };
}

#[derive(Debug)]
pub enum EngineError {
    ParsingError(pest::error::Error<parser::Rule>),
    HandleNodeError,
    NonExistControlNodeError(String),
    ParameterError((usize, usize)),
    SampleNotExistError((usize, usize)),
    InsufficientParameter((usize, usize)),
    NotModuableError((usize, usize)),
    ParaTypeError((usize, usize)),

}