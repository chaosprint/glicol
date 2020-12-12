use std::{collections::HashMap, num::ParseFloatError};

extern crate pest;
extern crate pest_derive;
use pest::Parser;
use pest::iterators::Pairs;
mod parser;
use parser::*;

// use dasp_graph::{Buffer, , Node};
use dasp_graph::{NodeData, Input, Buffer, BoxedNodeSend, Processor};
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
use node::filter::{LPF, HPF, Allpass, Comb};
use node::map::{LinRange};
use node::rand::{Choose};
use node::buf::{Buf};
use node::state::{State};
use node::pan::{Pan, Mix2};
use node::delay::{Delay};
use node::system::{Clock, AudioIn};
use node::reverb::{Plate};

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
    clock: NodeIndex,
    audio_in: NodeIndex,
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
        // let clock = g.add_node(NodeData::new1(BoxedNodeSend::new(Clock{})));

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
            clock: NodeIndex::new(0),
            audio_in: NodeIndex::new(1),
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

    pub fn input(&mut self, inputs: &[Input]) {
        self.graph[self.control_nodes["~input"]].buffers[0] = inputs[0].buffers()[0].clone();
    }

    // error only comes from this method
    pub fn make_graph(&mut self) -> Result<(), EngineError>{
        self.audio_nodes.clear();
        self.control_nodes.clear();
        self.graph.clear();
        self.sidechains_list.clear();

        self.clock = self.graph.add_node(NodeData::new1(BoxedNodeSend::new(Clock{})));
        self.audio_in = self.graph.add_node(NodeData::new1(BoxedNodeSend::new(AudioIn{})));
        self.control_nodes.insert("~input".to_string(), self.audio_in);

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
                                // "freeverb" => FreeVerbNode::new(&mut paras)?,
                                "pan" => Pan::new(&mut paras)?,
                                "delay" => Delay::new(&mut paras)?,
                                "apf" => Allpass::new(&mut paras)?,
                                "comb" => Comb::new(&mut paras)?,
                                "mix" => Mix2::new(&mut paras)?,
                                "plate" => Plate::new(&mut paras)?,
                                _ => Pass::new(name)?
                            };
                    
                            let node_index = self.graph.add_node(node_data);

                            self.graph.add_edge(self.clock, node_index, ());
                    
                            // connect to previous node, or redirect the previous node to a control node
                            if previous_nodes.len() > 0 {
                                if dest != "" {
                                    self.sidechains_list.push((previous_nodes[0], dest));
                                } else {
                                    self.graph.add_edge(previous_nodes[0], node_index, ());
                                }
                            }
                    
                            // only process the last nodes of chains in the audio nodes vec
                            if current_ref_name.contains("~") {
                                
                                self.control_nodes.insert(current_ref_name.to_string(), node_index);                            
                                // for all the audio nodes, we need to have an individual clock
                                // otherwise it will be processed several times
    
                                // self.clocks.insert(current_ref_name.to_string(), clock_node_index);
                            } else {
                                self.audio_nodes.insert(current_ref_name.to_string(), node_index);
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
                println!("reversed connection for {}", name);
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

    pub fn gen_next_buf_64(&mut self) -> Result<[f32; 128], EngineError> {
        
        // using self.buffer will cause errors on bela
        let mut output: [f32; 128] = [0.0; 128];
        for (_ref_name, node) in &self.audio_nodes {

            self.graph[self.clock].buffers[0][0] = self.elapsed_samples as f32;
            self.processor.process(&mut self.graph, *node);

            let bufleft = &self.graph[*node].buffers[0];
            let bufright = match &self.graph[*node].buffers.len() {
                1 => {bufleft},
                2 => {&self.graph[*node].buffers[1]},
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

    // , input: Input
    pub fn gen_next_buf_128(&mut self, inbuf: &mut [f32]) -> Result<([f32; 256], [u8;256]), EngineError> {
        // you just cannot use self.buffer
        let mut output: [f32; 256] = [0.0; 256];
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
            // println!("{:?}", *node);
            self.graph[self.clock].buffers[0][0] = self.elapsed_samples as f32;
            // let mut sum = 0.0;
            for i in 0..64 {
                // sum += inbuf[i];
                self.graph[self.control_nodes["~input"]].buffers[0][i] = inbuf[i];
            }
            // assert!(sum > 0.0);
            self.processor.process(&mut self.graph, *node);
        }

        for (_ref_name, node) in &self.audio_nodes {
            let bufleft = &self.graph[*node].buffers[0];
            let bufright = match &self.graph[*node].buffers.len() {
                1 => {bufleft},
                2 => {&self.graph[*node].buffers[1]},
                _ => {unimplemented!()}
            };
            for i in 0..64 {
                output[i] += bufleft[i];
                output[128+i] += bufright[i];
                // output[i] += inbuf[i];
                // output[128+i] += inbuf[i];
            }
        }
        self.elapsed_samples += 64;

        // process 64..128,and output stereo
        for (_ref_name, node) in &self.audio_nodes {
            // println!("{:?}", *node);
            self.graph[self.clock].buffers[0][0] = self.elapsed_samples as f32;
            for i in 0..64 {
                self.graph[self.control_nodes["~input"]].buffers[0][i] = inbuf[i+64];
            }
            self.processor.process(&mut self.graph, *node);
        }

        // process all audio nodes first; get audio nodes out values now
        for (_ref_name, node) in &self.audio_nodes {
            let bufleft = &self.graph[*node].buffers[0];
            let bufright = match &self.graph[*node].buffers.len() {
                1 => {bufleft},
                2 => {&self.graph[*node].buffers[1]},
                _ => {unimplemented!()}
            };

            for i in 0..64 {
                output[i+64] += bufleft[i];
                output[i+64+128] += bufright[i];
                // output[i+64] += inbuf[i+64];
                // output[i+64+128] += inbuf[i+64];
            }
        }

        self.elapsed_samples += 64;
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
            let mut sidechain_id: u8 = 0;

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
                        sidechain_ids.push(sidechain_id);
                    }
                };
                sidechain_id += 1;
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
macro_rules! create_node_with_code {
    ($code: expr) => {
        println!(stringify!($code))
    };
}

// #[macro_export]
// macro_rules! new_node {
//     ($node_name: ident) => {
//         pub struct $node_name {
            
//         }
//     };
// }