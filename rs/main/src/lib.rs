//! # Glicol: A Computer Music Language Written in Rust
//! This is the engine of Glicol
//! The engine has two part one is the language and the other is the audio
//! The audio engine can be useful for some real world projects
//! If you are targeting WebAssembly, this can be a useful resource.
#![allow(warnings)]
use std::{collections::HashMap};
use dasp_graph::{NodeData, BoxedNodeSend};
use petgraph::graph::{NodeIndex};
use petgraph::stable_graph::{StableDiGraph};
use pest::Parser;
use pest::iterators::Pairs;

use glicol_synth::Para;
use glicol_synth::make_node;
use glicol_synth::signal::dummy::{Clock, AudioIn};
use glicol_synth::{oscillator, signal, filter, operation, sampling, effect, pass::*};
use glicol_synth::{GlicolNodeData, GlicolGraph,
    GlicolProcessor, NodeResult, GlicolError};

use glicol_parser::*;

use glicol_ext::{make_node_ext, preprocessor};

mod utili;
use utili::{preprocess_signal, preprocess_mul, lcs, process_error_info};
use regex::Regex;


/// The engine of Glicol
/// This engine can takes Glicol code, convert it to audio graph and process it
/// The engine can also be used independantly as a wrapper to dasp_graph crate (see the examples folder)
pub struct Engine<const N: usize> {
    pub elapsed_samples: usize,
    pub graph: GlicolGraph<N>,
    pub processor: GlicolProcessor<N>,
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

impl<const N: usize> Engine<N> {
    pub fn new(sr: usize) -> Self {
        let max_nodes = 1024;
        let max_edges = 1024;
        let g = GlicolGraph::<N>::with_capacity(max_nodes, max_edges);
        let p = GlicolProcessor::<N>::with_capacity(max_nodes);

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

    pub fn preprocess(&mut self) -> Result<(), EngineError> {
        let mut processed_code = "".to_owned();
        let mut appendix_string_full = "".to_owned();
        
        let mut target_code = self.code.clone();

        let lines = match GlicolParser::parse(Rule::block, &mut self.code) {
            Ok(mut res) => {
                if res.as_str() < &mut target_code {
                    return Err(EngineError::ParsingIncompleteError(res.as_str().len()));
                }
                res.next().unwrap()
            },
            Err(e) => { println!("{:?}", e); panic!(); return Err(EngineError::ParsingError(e))}
        };

        let mut current_ref_name = "".to_owned();
        for line in lines.into_inner() {
           
            let inner_rules = line.into_inner();
            for element in inner_rules {
                match element.as_rule() {
                    Rule::reference => {
                        current_ref_name = element.as_str().to_owned();
                        processed_code.push_str(&current_ref_name);
                        processed_code.push_str(": ");
                        // println!("current_ref_name {:?}", current_ref_name);
                    },
                    Rule::chain => {
                        let mut nodes = vec![];
                        for node in element.into_inner() {
                            let mut name_and_paras = node.into_inner();
                            let name_and_paras_str: String = name_and_paras.as_str().to_string();
                            let node_name = name_and_paras.next().unwrap();
                            let mut paras = name_and_paras.clone(); // the name is ripped above

                            if ["sin", "saw", "squ", "tri"].contains(&node_name.as_str()) {
                                let mut s = "const_sig ".to_owned();
                                s.push_str(paras.as_str());
                                nodes.push(s);
                                let mut pseudo = node_name.as_str().to_owned();
                                pseudo.push_str(" 1");
                                nodes.push(pseudo);

                                // let (res_in_place, res_appendix) = preprocessor(name_and_paras_str);
                                // nodes.push(res_in_place);
                                // processed_code.push_str(res_appendix);
                            } else {
                                let (inplace_string, appendix_string) = preprocessor(&current_ref_name, node_name.as_str(), &mut paras)?;
                                nodes.push(inplace_string);
                                appendix_string_full.push_str(&appendix_string);
                                appendix_string_full.push_str(";\n");
                            }
                        }
                        let processed_chain_str = nodes.join(" >> ");
                        processed_code.push_str(&processed_chain_str);
                        processed_code.push_str(";\n");
                        processed_code.push_str(&appendix_string_full);
                    },
                    _ => {}
                }
            }
        }
        self.code = processed_code;
        // panic!(processed_code);
        Ok(())
    }
    /// The main function to convert the code input string into graph structure inside the engine
    pub fn make_graph(&mut self) -> Result<(), EngineError>{
        self.preprocess();
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
        self.all_refs.push("~input".to_string());
        self.node_by_chain.insert(
            "~input".to_string(),
            vec![(self.audio_in, "~input".to_string())]
        );

        // println!("code before preprocess: {}",&self.code);
        // self.code = preprocess_signal(&mut self.code)?;
        // self.code = preprocess_mul(&mut self.code)?;
        println!("code after preprocess: {}",&self.code);
        
        let mut target_code = self.code.clone();

        let lines = match GlicolParser::parse(Rule::block, &mut self.code) {
            Ok(mut res) => {
                if res.as_str() < &mut target_code {
                    return Err(EngineError::ParsingIncompleteError(res.as_str().len()));
                }
                res.next().unwrap()
            },
            Err(e) => { println!("{:?}", e); panic!(); return Err(EngineError::ParsingError(e))}
        };

        let mut current_ref_name: &str = "";
        // println!("lines.into_inner() {:?}", lines.clone());
        // add nodes to nodes chain vectors in the HashMap with ref as key
        for line in lines.into_inner() {
           
            let inner_rules = line.into_inner();
            for element in inner_rules {
                match element.as_rule() {
                    Rule::reference => {
                        current_ref_name = element.as_str();
                        // println!("current_ref_name {:?}", current_ref_name);
                    },
                    Rule::chain => {
                        self.all_refs.push(current_ref_name.to_string());
                        let refname = current_ref_name.to_string();

                        // TODO: this should be solved by parser
                        // let new: Vec<String> = element.clone().into_inner()
                        // .map(|v|v.as_str().to_string().chars()
                        // .filter(|c| !c.is_whitespace()).collect()).collect();

                        let chain_plain_str: Vec<String> = element.clone().into_inner()
                        .map(|v|v.as_str().to_string()).collect();
                        // new.reverse();
                        // println!("new {:?}", chain_plain_str);

                        let (add, _rem, del) = match self.chain_info
                        .contains_key(&refname) {

                            true => {
                                let old = self.chain_info[&refname].clone();
                                self.chain_info.insert(refname.clone(), 
                                chain_plain_str.clone());
                                lcs(&old, &chain_plain_str)
                            },
                            _ => {
                                self.chain_info.insert(refname.clone(), 
                                chain_plain_str.clone());
                                let t = Vec::<String>::new();
                                lcs(&t, &chain_plain_str)
                            }
                        };

                        // println!("add, _rem, del {:?}", (add.clone(), _rem.clone(), del.clone()));

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
                        
                        for node in element.into_inner() {
                            // println!("\n\nnode {:?}\n\n", node);
                            let mut name_and_paras = node.into_inner();

                            // we use the name and para e.g. sin440 as id
                            // perhaps no need for clean space, but need to be consistent with the lcs calculation
                            let name_and_paras_str: String = name_and_paras.as_str().to_string();
                            // .chars().filter(|c| !c.is_whitespace()).collect();
                            // println!("\n\nnode id {:?}\n\n", name_and_paras_str);
                            let name_obj = name_and_paras.next().unwrap();
                            let mut paras = name_and_paras.clone(); // the name is ripped
                            // println!("\n\nname_obj {:?}\n\n", name_obj);
                            let pos = (name_obj.as_span().start(), name_obj.as_span().end());
                            let name = name_obj.as_str();
                            let dest = match name_obj.as_rule() {
                                Rule::paras => format!("@rev{}", name_obj.as_str()),
                                _ => "".to_string()
                            };
                            // println!("{:?}", &add);
                            for info in &add {
                                // println!("name_and_paras_str {:?} != info {:?} ?", &name_and_paras_str, &info.0);
                                if info.0 == name_and_paras_str {

                                    // TODO: support ref in ext
                                    // TODO: report errors for ext
                                    let (node_data, sidechains) = match make_node_ext(
                                        name, &mut paras, pos,
                                        &self.samples_dict,
                                        self.sr,
                                        self.bpm
                                    ) {
                                        Some(v) => (v, vec![]),
                                        None => make_node(
                                                    name, &mut paras, pos,
                                                    &self.samples_dict,
                                                    self.sr,
                                                    self.bpm
                                                )?
                                    };

                                    let node_index = self.graph.add_node(node_data);
                                    
                                    if !self.node_by_chain.contains_key(&refname) {
                                        // head of chain
                                        self.node_by_chain.insert(refname.clone(),
                                        vec![(node_index, name_and_paras_str.clone())]);
                                    } else {

                                        let mut list = self.node_by_chain[&refname]
                                        .clone();

                                        if &dest != "" {
                                            self.sidechains_list.push(
                                                (list.last().unwrap().0, 
                                                dest.clone()));
                                        };                     
                                        list.insert(
                                            info.1, (node_index, name_and_paras_str.clone()));

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
            if refname != "~input" {
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
            // println!("node_by_chain {:?}", self.node_by_chain);
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
    pub fn gen_next_buf(&mut self, inbuf: &mut [f32])
    -> Result<([f32; 256], [u8;256]), EngineError> {
        // don't use self.buffer
        let mut output: [f32; 256] = [0.0; 256];
        let mut console: [u8;256] = [0; 256];
        let one_bar = (240.0 / self.bpm * self.sr as f32) as usize;

        // if self.update && (n + 128 + 2048) % one_bar < 128 {
        //     self.fade = 0;
        // }

        if self.update && (self.elapsed_samples + N) % one_bar <= N {
            // println!("updating... at {}", (self.elapsed_samples + 128) % one_bar);
            self.update = false;
            // println!("updated!");
            match self.make_graph() {
                Ok(_) => {
                    self.code_backup = self.code.clone();
                    // println!("success make backup {}", self.code_backup);
                },
                Err(e) => {
                    // panic!(e);
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
                                self.code_backup = "~dummy: const 0.0".to_string();
                            }
                            info
                        },
                        EngineError::NodeNameError((n, s, e)) => {
                            process_error_info(self.code.clone(), 6, s, e)
                        },
                        EngineError::ParaTypeError((s, e)) => {
                            process_error_info(self.code.clone(), 7, s, e)
                        },
                        EngineError::NotModuableError((s, e)) => {
                            process_error_info(self.code.clone(), 8, s, e)
                        },
                        EngineError::InsufficientParameter((s, e)) => {
                            process_error_info(self.code.clone(), 9, s, e)
                        },
                        _ => {
                            info[0] = 10; // type
                            info[1] = 0; // unknown position
                            if self.code == "" {
                                self.code_backup = "~dummy: const 0.0".to_string();
                            }
                            info
                        }
                    };
                    // println!("debug {:?} code backup is {} !", console, self.code_backup);
                    self.soft_reset();
                    self.set_code(&self.code_backup.clone());
                    self.make_graph()?;
                }
            }
        }

        // println!("&self.node_by_chain{:?}", &self.node_by_chain);
        // process 0..128
        for (refname, v) in &self.node_by_chain {
            if refname.contains("~") {
                continue;
            }
            // this must be inside to prevent double processing
            self.graph[self.clock].buffers[0][0] = self.elapsed_samples as f32;
            for i in 0..N {
                self.graph[
                    self.node_by_chain["~input"][0].0
                ].buffers[0][i] = inbuf[i];
            }
            self.processor.process(&mut self.graph, v.last().unwrap().0);
        }

        // sendout 0..128
        for (refname, v) in &self.node_by_chain {
            if refname.contains("~") {
                continue;
            }
            let bufleft = &self.graph[v.last().unwrap().0].buffers[0];
            // println!(" {} bufleft {:?}", refname, bufleft);
            let bufright = match &self.graph[v.last().unwrap().0].buffers.len() {
                1 => {bufleft},
                2 => {&self.graph[v.last().unwrap().0].buffers[1]},
                _ => {unimplemented!()} // no multi-chan for now
            };

            for i in 0..N {
                // let s = match self.fade {
                //     k if k > 4095 => 1.0,
                //     _ => self.window[self.fade] as f32 * -1.0 + 1.0
                // };
                // self.fade += 1;
                // let scale = 1.0;bufleft[i] * bufright[i] * 
                output[i] += bufleft[i] * self.track_amp;
                output[i+N] += bufright[i] * self.track_amp;
                // output[i] += s;
                // output[i+128] += s;
            }
        }
        self.elapsed_samples += N;

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

    pub fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
        self.update = true;
        // Self {code: code.to_string(), update: true, ..self}
    }

    pub fn set_track_amp(&mut self, amp: f32) {
        self.track_amp = amp;
    }

    pub fn make_chain(&mut self, mut nodes: Vec<GlicolNodeData<N>>) -> Vec<NodeIndex> {
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
        // println!("{}", self.code);
        self.make_graph()?;
        self.gen_next_buf(&mut [0.0;N])
    }
}

#[macro_export]
macro_rules! chain {
    ([$($node: expr),*] in $engine:ident) => {
        $engine.make_chain(vec![$($node,)*])   
    };
}

#[derive(Debug, Eq, PartialEq)]
pub enum EngineError {
    NonExistControlNodeError(String), // handled
    ParameterError((usize, usize)), // handled
    SampleNotExistError((usize, usize)), // handled
    InsufficientParameter((usize, usize)),
    NotModuableError((usize, usize)),
    ParaTypeError((usize, usize)),
    NodeNameError((String, usize, usize)),  // handled
    ParsingError(pest::error::Error<glicol_parser::Rule>), // handled
    HandleNodeError, // handled
    ParsingIncompleteError(usize),
}

impl From<GlicolError> for EngineError {
    fn from(e: GlicolError) -> EngineError {
        match e {
            GlicolError::NonExistControlNodeError(v) => EngineError::NonExistControlNodeError(v),
            GlicolError::ParameterError((s,e)) => EngineError::ParameterError((s,e)),
            GlicolError::SampleNotExistError((s,e))  => EngineError::SampleNotExistError((s,e)),
            GlicolError::InsufficientParameter((s,e)) => EngineError::InsufficientParameter((s,e)),
            GlicolError::NotModuableError((s,e)) => EngineError::NotModuableError((s,e)),
            GlicolError::ParaTypeError((s,e)) => EngineError::ParaTypeError((s,e)),
            GlicolError::NodeNameError((st, s,e)) => EngineError::NodeNameError((st, s,e)),
            _ => unimplemented!()
        }
    }
}