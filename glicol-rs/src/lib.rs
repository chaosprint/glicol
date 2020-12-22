use std::{collections::HashMap, num::ParseFloatError};

extern crate pest;
extern crate pest_derive;
use pest::Parser;
use pest::iterators::Pairs;
mod parser;
use parser::*;

use dasp_graph::{NodeData, Input, BoxedNodeSend, Processor};
use petgraph::graph::{NodeIndex};
use petgraph::Directed;
use petgraph::stable_graph::{StableGraph, StableDiGraph};

mod node;
use node::make_node;
use node::adc::{Adc, AdcSource};
use node::system::{Clock, AudioIn};

mod utili;
use utili::{midi_or_float, code_hack, lcs};

pub type NodeResult =Result<
    (NodeData<BoxedNodeSend>, Vec<String>), EngineError>;

pub struct Engine {
    pub elapsed_samples: usize,
    pub graph: StableGraph<NodeData<BoxedNodeSend>, (), Directed, u32>,
    processor: Processor<StableDiGraph<NodeData<BoxedNodeSend>, (), u32>>,
    sidechains_list: Vec<(NodeIndex, String)>,
    pub adc_source_nodes: Vec<NodeIndex>,
    pub adc_nodes: Vec<NodeIndex>,
    pub samples_dict: HashMap<String, &'static[f32]>,
    pub sr: u32,
    pub bpm: f32,
    pub chain_string: HashMap<String, String>,
    pub node_by_chain: HashMap<String, Vec<(NodeIndex, String)>>,
    pub chain_info: HashMap<String, Vec<String>>,
    // pub node_index_dict: HashMap<String, Vec<(NodeIndex, Vec<String>)>>,
    // pub node_string_dict: HashMap<String, Vec<Vec<String>>>,
    pub clock: NodeIndex,
    audio_in: NodeIndex,
    code: String,
    code_backup: String,
    update: bool,
    // pub updatefree: Vec<String>,
    pub all_refs: Vec<String>,
}

impl Engine {
    pub fn new() -> Engine {
        // Chose a type of graph for audio processing.
        type MyGraph = StableGraph<NodeData<BoxedNodeSend>, (), Directed, u32>;
        // Create a short-hand for our processor type.
        type MyProcessor = Processor<MyGraph>;

        let max_nodes = 1024;
        let max_edges = 1024;
        let g = MyGraph::with_capacity(max_nodes, max_edges);
        let p = MyProcessor::with_capacity(max_nodes);

        Engine {
            graph: g,
            processor: p,
            code: "default: const 0".to_string(),
            code_backup: "default: const 0".to_string(),
            samples_dict: HashMap::new(),
            adc_source_nodes: Vec::new(),
            adc_nodes: Vec::new(),
            sidechains_list: Vec::new(),
            chain_string: HashMap::new(),
            node_by_chain: HashMap::new(),
            // node_index_dict: HashMap::new(),
            // node_string_dict: HashMap::new(),
            chain_info: HashMap::new(),
            clock: NodeIndex::new(0),
            audio_in: NodeIndex::new(1),
            sr: 44100,
            bpm: 120.0,
            elapsed_samples: 0,
            update: false,
            // updatefree: Vec::new(),
            all_refs: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.elapsed_samples = 0;
        self.update = false;
        self.code = "default: const 0".to_string();
        self.sidechains_list.clear();
        self.graph.clear();
    }

    pub fn update(&mut self) {
        self.update = true;
    }

    pub fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }

    // pub fn input(&mut self, inputs: &[Input]) {
    //     // self.graph[self.control_nodes["~input"]].buffers[0]
    //     // = inputs[0].buffers()[0].clone();
    // }

    // error only comes from this method
    pub fn make_graph(&mut self) -> Result<(), EngineError>{
        // self.node_by_chain.clear();
        self.graph.clear_edges();
        self.all_refs.clear();
        // self.sidechains_list.clear();

        if self.graph.node_count() < 2 {
            self.clock = self.graph.add_node(
                NodeData::new1(BoxedNodeSend::new(Clock{})));
            self.audio_in = self.graph.add_node(
                NodeData::new1(BoxedNodeSend::new(AudioIn{})));
        }

        self.all_refs.push("~input".to_string());
        self.node_by_chain.insert(
            "~input".to_string(),
            vec![(self.audio_in, "~input".to_string())]
        );

        let mut b = code_hack(&mut self.code)?;
        println!("{}",&b);

        let lines = match GlicolParser::parse(Rule::block, &mut b) {
            Ok(mut v) => v.next().unwrap(),
            Err(e) => { println!("{:?}", e); return Err(EngineError::ParsingError)}
        };

        // let mut previous_nodes = Vec::<NodeIndex>::new();
        let mut current_ref_name: &str = "";

        // add function to Engine HashMap Function Chain Vec accordingly
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

                        let new: Vec<String> = element.clone().into_inner()
                        .map(|v|v.as_str().to_string().chars()
                        .filter(|c| !c.is_whitespace()).collect()).collect();
                        // new.reverse();

                        let (add, rem, del) = match self.chain_info
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

                            for info in &add {
                                println!("info {:?} != {:?} ?", &id, &info.0);
                                if info.0 == id {
                                    let (node_data, sidechains) = make_node(
                                        name, &mut paras, &self.samples_dict)?;
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

                                        println!("insert{} at{}",id.clone(),info.1);
                                        self.node_by_chain.insert(
                                            refname.clone(),list);
                                    };

                                    for sidechain in sidechains.into_iter() {
                                        self.sidechains_list.push(
                                            (node_index, sidechain));
                                    };
                                }
                            };
                        }
                    },
                    _ => ()
                }
            }
        }

        // for chains that are simply deleted or commented out
        for (key, _) in &self.node_by_chain {
            if !self.all_refs.contains(&key) {
                for n in &self.node_by_chain[key] {
                    self.graph.remove_node(n.0);
                    self.sidechains_list.retain(|v| v.0 != n.0);
                }
            }
        }

        let all_refs = self.all_refs.clone();
        let mut list = self.node_by_chain.clone();
        list.retain(|k, _| all_refs.contains(k));
        self.node_by_chain = list;
        // .iter_mut().map(|(k, v)|{v.reverse();}).collect();
        let all_refs = self.all_refs.clone();
        self.chain_string.retain(|k, _| all_refs.contains(k));
        let all_refs = self.all_refs.clone();
        self.chain_info.retain(|k, _| all_refs.contains(k));
        // println!("node_by_chain {:?}", self.node_by_chain);

        // connect clocks to all the nodes
        for (refname, nodes) in &self.node_by_chain {
            if refname != "~input" {
                for n in nodes {
                    self.graph.add_edge(self.clock, n.0,());
                }
            }
        }

        // println!("node_by_chain {:?}", self.node_by_chain);
        // println!("sidechainlist {:?}", self.sidechains_list);

        // make edges in each chain
        for (_refname, node_chains) in &self.node_by_chain {
            if node_chains.len() >= 2 {
                // println!("a");
                self.graph.add_edge(node_chains[0].0,node_chains[1].0,());
                // println!("b");
                for i in 0..(node_chains.len()-2) {
                    self.graph.add_edge(node_chains[i+1].0,node_chains[i+2].0,());
                };
            };
        }
        
        // make edges cross chain
        for pair in &self.sidechains_list {
            println!("{}", pair.1);
            if pair.1.contains("@rev") {
                
                let name = &pair.1[4..];
                println!("reversed connection for {}", name);
                if !self.node_by_chain.contains_key(name) {
                    return Err(EngineError::NonExistControlNodeError);
                }
                let control_node = self.node_by_chain[name].last().unwrap().0;
                self.graph.add_edge(pair.0, control_node, ());
            } else {
                if !self.node_by_chain.contains_key(&pair.1) {
                    return Err(EngineError::NonExistControlNodeError);
                }
                let control_node = self.node_by_chain[&pair.1].last().unwrap().0;
                self.graph.add_edge(control_node, pair.0, ());
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

    pub fn gen_next_buf_64(&mut self, inbuf: &mut [f32])
    -> Result<[f32; 128], EngineError> {
        
        // using self.buffer will cause errors on bela
        let mut output: [f32; 128] = [0.0; 128];
        for (refname, v) in &self.node_by_chain {
            if refname.contains("~") {
                continue;
            };
            self.graph[self.clock].buffers[0][0] = self.elapsed_samples as f32;
            for i in 0..64 {
                self.graph[self.node_by_chain["~input"][0].0
                ].buffers[0][i] = inbuf[i];
            }
            self.processor.process(&mut self.graph, v.last().unwrap().0);

            let bufleft = &self.graph[v.last().unwrap().0].buffers[0];
            let bufright = match &self.graph[v.last().unwrap().0].buffers.len() {
                1 => {bufleft},
                2 => {&self.graph[v.last().unwrap().0].buffers[1]},
                _ => {unimplemented!()}
            };
            for i in 0..64 {
                output[i] += bufleft[i];
                output[i+64] += bufright[i];
            }
        }
        self.elapsed_samples += 64;
        Ok(output)
    }

    pub fn gen_next_buf_128(&mut self, inbuf: &mut [f32])
    -> Result<([f32; 256], [u8;256]), EngineError> {

        // you just cannot use self.buffer
        let mut output: [f32; 256] = [0.0; 256];
        let mut console: [u8;256] = [0; 256];

        let is_near_bar_end = (self.elapsed_samples + 128) % 88200 < 128;
        
        // for wasm live coding
        if self.update && is_near_bar_end {
            self.update = false;

            match self.make_graph() {
                Ok(_) => {
                    self.code_backup = self.code.clone();
                },
                Err(e) => {
                    // println!("{:?}", e);
                    let mut info: [u8; 256] = [0; 256];
                    console = match e {
                        EngineError::SampleNotExistError((s, e)) => { 
                            let l = self.code.clone()[..s].matches("\n").count() as u8;
                            info[0] = 1;
                            info[1] = l;
                            // println!("{}", self.code);
                            let word = self.code[s..e].as_bytes();
                            for i in 2..word.len()+2 {
                                info[i] = word[i-2]
                            }
                            info   
                        },
                        EngineError::NonExistControlNodeError => {
                            info[0] = 2;
                            info[1] = 0;
                            info
                        },
                        EngineError::ParameterError => {
                            info[0] = 3;
                            info[1] = 0;
                            info
                        },
                        EngineError::HandleNodeError => {
                            info[0] = 4;
                            info[1] = 0;
                            info
                        },
                        _ => unimplemented!()
                    };
                    self.code = self.code_backup.clone();
                    self.make_graph()?; // this should be fine
                }
            }
        }

        let first64 = self.gen_next_buf_64(inbuf)?;
        for i in 0..64 {
            output[i] = first64[i];
            output[i+128] = first64[i+64];
        }
        let second64 = self.gen_next_buf_64(inbuf)?;
        for i in 0..64 {
            output[i+64] = second64[i];
            output[i+128+64] = second64[i+64]
        }
        Ok((output, console))
    }
}

#[derive(Debug)]
pub enum EngineError {
    ParsingError,
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

#[macro_export]
/// this works well for nodes whose inner states are only floats
/// e.g. oscillator, filter, operator
macro_rules! handle_params {
    ( 
        { $($id: ident: $default: expr),* }
        $(,{$( $extra_params: ident : $val: expr),* })?
        $(,[$( ( $related: ident, $extra_id: ident, $handler: expr) ),* ])?
    ) => {
        pub fn new(paras: &mut Pairs<Rule>) ->
        Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

            let mut sidechains = Vec::<String>::new();
            let mut params_val = std::collections::HashMap::<&str, f32>::new();
            let mut sidechain_ids = Vec::<u8>::new();
            let mut _sidechain_id: u8 = 0;

            // TODO: need to handle unwarp
            $(
                let current_param: String = paras.next().unwrap().as_str().to_string();
                let parse_result = current_param.parse::<f32>();
                match parse_result {
                    Ok(val) => {
                        params_val.insert(stringify!($id), val);
                    },
                    Err(_) => {
                        sidechains.push(current_param);
                        params_val.insert(stringify!($id), $default);
                        sidechain_ids.push(_sidechain_id);
                    }
                };
                _sidechain_id += 1;
            )*

            $(
                $(
                    let $extra_id = $handler(params_val[stringify!($related)]);
                )*
            )?

            Ok((NodeData::new1( BoxedNodeSend::new( Self {
                $(
                    $id: params_val[stringify!($id)],
                )*
                $(
                    $(
                        $extra_params: $val,
                    )*
                )?
                $(
                    $(
                        $extra_id,
                    )*
                )?
                sidechain_ids
            })), sidechains))
        }
    };
}

#[macro_export]
macro_rules! ndef {
    ($struct_name: ident, $channel_num: ident, {$code_str: expr}) => {
        pub struct $struct_name {
            engine: Engine
        }
        
        impl $struct_name {
            pub fn new(paras: &mut Pairs<Rule>) -> Result<
            (NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
                // let param_a = paras.as_str().parse::<f32>().unwrap();
                let mut engine = Engine::new();
                // let code: &'static str = &$code_str.replace("$1", paras.as_str());
                // let mut code = $code_str;
                // if code.contains("{}") {
                //     code = format!($code_str, a=paras.as_str()
                // }
                engine.set_code(&format!($code_str, a=paras.as_str()));
                // engine.set_params(paras);
                // println!("{}", engine.code);
                engine.make_graph()?;
                engine.update();
                Ok((NodeData::$channel_num(BoxedNodeSend::new( Self {
                    engine
                })), vec![]))
            }
        }
        
        impl Node for $struct_name {
            fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
                // self.engine.input(inputs); // mono or stereo?
                let mut input = inputs[0].buffers()[0].clone();
                let buf = self.engine.gen_next_buf_64(&mut input).unwrap();
                match output.len() {
                    1 => {
                        for i in 0..64 {
                            output[0][i] = buf[i];
                        }
                    },
                    2 => {
                        for i in 0..64 {
                            output[0][i] = buf[i];
                            output[1][i] = buf[i+64];
                        }
                    },
                    _ => {}
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let mut engine = Engine::new();
        engine.set_code("aa: sin 60 >> mul ~am
    
        ~am: sin 0.3 >> linrange 0.1 0.9");
    
        engine.update();
        engine.make_graph();

        for _ in 0..(43000.0/128.0) as usize {
            let out = engine.gen_next_buf_128(&mut [0.0;128]).unwrap().0;
        }
        engine.set_code("aa: sin 80 >> mul ~am
    
        ~am: sin 0.3 >> linrange 0.1 0.9");

        engine.update();
        engine.make_graph();

        for _ in 0..(43000.0/128.0) as usize {
            let out = engine.gen_next_buf_128(&mut [0.0;128]).unwrap().0;
        }
    }
}