#![allow(warnings)]
use std::{collections::HashMap};
use dasp_graph::{NodeData, BoxedNodeSend, Processor, Buffer, Input, Node};
use petgraph::graph::{NodeIndex};
use petgraph::stable_graph::{StableDiGraph};
use pest;
use pest::Parser;
use pest::iterators::Pairs;
use glicol_parser::*;

pub mod macros; use macros::*;

pub mod oscillator; use oscillator::*;
use {sin_osc::SinOsc, saw_osc::SawOsc, squ_osc::SquOsc, tri_osc::TriOsc};

pub mod signal; use signal::*;
use {imp::*, const_sig::ConstSig, noise::Noise, dummy::Clock, dummy::AudioIn, phasor::Phasor};

pub mod operation; use operation::*;
use {mul::Mul, add::Add, script::Script};

pub mod filter; use filter::*;
use {rlpf::*, rhpf::*, apfgain::*, apfdecay::*, onepole::*,comb::*};

pub mod sampling; use sampling::*;
use {seq::*, sampler::*,speed::*, choose::*};

pub mod envelope; use envelope::*;
use {envperc::*, shape:: Shape};

pub mod pass; use pass::*;

pub mod effect; use effect::*;
use {delayn::*, delay::*, pan::*, balance::*};

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;
pub type NodeResult<const N: usize> = Result<(GlicolNodeData<N>, Vec<String>), GlicolError>;

#[derive(Debug)]
pub enum GlicolError {
    NonExistControlNodeError(String),
    ParameterError((usize, usize)),
    SampleNotExistError((usize, usize)),
    InsufficientParameter((usize, usize)),
    NotModuableError((usize, usize)),
    ParaTypeError((usize, usize)),
    NodeNameError((String, usize, usize)),
    ParsingError(pest::error::Error<glicol_parser::Rule>),
    ParsingIncompleteError(usize),
}


// std::convert::From<std::option::NoneError
// impl From<std::option::NoneError> for GlicolError {
//     fn from(error: std::option::NoneError) -> Self {
//         GlicolError::ParameterError((0, 0))
//     }
// }

pub fn make_node<const N: usize>(
    name: &str,
    paras: &mut Pairs<Rule>,
    pos: (usize, usize),
    samples_dict: &HashMap<String, &'static[f32]>,
    sr: usize,
    bpm: f32,
) -> NodeResult<N> {

    // println!("makenode for name: {}, para: {}", name, paras.as_str());
    // TODO: handle this in the parser
    // if !["", ""].contains(&name) {
    //     return Err(GlicolError::NodeNameError((paras.as_str().to_string(), paras.as_span().start(), paras.as_span().end())))
    // };
    
    let alias = match name {
        "sp" => "sampler",
        "*" => "mul",
        "noiz" => "noise",
        "lpf" => "rlpf",
        "hpf" => "rhpf",
        _ => {
            if name.contains("~") {
                "pass"
            } else {
                name
            }
        }
    };

    // println!("name after alis {} {:?}", alias, paras);

    if paras.as_str() == "_" {
        let nodedata = match alias {

            // oscillators are preprocessed, so do not use _ here
            // change it in the preprocessor
            
            // "sin" => sin_osc!(N => {freq: 44100.0, sr: sr}),
            // "saw" => saw_osc!(N => {freq: 44100.0, sr: sr}),
            // "squ" => squ_osc!(N => {freq: 44100.0, sr: sr}),
            // "tri" => tri_osc!(N => {freq: 44100.0, sr: sr}),
            "script" => Script::<N>::new().build(),
            "const_sig" => const_sig!(N => 1.0),
            "mul" => mul!(N => 1.0),
            "add" => add!(N => 0.0),
            "rlpf" => rlpf!(N => {cutoff: 1000.0, q: 1.0, sr: sr}),
            "rhpf" => rhpf!(N => {cutoff: 1000., q: 1., sr: sr}),
    
            "noise" => noise!(N => 42),
            "imp" => imp!(N => {freq: 1.0, sr: sr}),
            "sampler" => {
                sampler!(N => samples_dict[&paras.as_str().replace("\\", "")])},
            "seq" => {
                // let info = process_seq(paras.as_str()).unwrap();
                seq!(N => {events: process_seq(paras)?.0, sidechain_lib: process_seq(paras)?.1, sr: sr, bpm: bpm})
            },
            "shape" => shape!(N => {sr: sr, points: get_shape_points(paras)?}),
            "speed" => speed!(N => 1.0),
            "choose" => choose!(N => get_notes(paras)?),
            "delayn" => delayn!(N => 0),
            "delay" => delay!(N => {delay: 0., sr: sr}),
            "onepole" => onepole!(N => 0.5),
            "comb" => comb!(N => {delay: 0.5, gain: 0.5, feedforward: 0.5, feedback: 0.5}),
            "apfdecay" => apfdecay!(N => {delay: 0.5, decay: 2.0}),
            "apfgain" => apfgain!(N => {delay: 0.5, gain: 0.5}),
            "pan" => pan!(N => 0.0),
            "balance" => balance!(N => 0.0),
            "pha" => phasor!(N => {freq: 1.0, sr: sr}),
            "pass" => Pass::<N>::new(),
            "envperc" => envperc!(N => {attack: 0.01, decay: 0.1, sr: sr}),
            _ => {
                // let a = paras.next().unwrap();
                return Err(GlicolError::NodeNameError((name.to_owned(), 0,0)))
            }
            // "buf" => Buf::new(&mut paras, 
            //     samples_dict)?,
            // "linrange" => LinRange::new(&mut paras)?,
            // "pha" => Phasor::new(&mut paras)?,
            // "state" => State::new(&mut paras)?,
            // "monosum" => MonoSum::new(&mut paras)?,
        };
        return Ok((nodedata, vec![]))
    }

    let modulable = match alias {
        "imp" => vec![Para::Number(1.0)],
        "sin" => vec![Para::Modulable],
        "saw" => vec![Para::Modulable],
        "squ" => vec![Para::Modulable],
        "tri" => vec![Para::Modulable],
        "const_sig" => vec![Para::Modulable],
        "mul" => vec![Para::Modulable],
        "add" => vec![Para::Modulable],
        "rlpf" => vec![Para::Modulable, Para::Number(1.0)],
        "rhpf" => vec![Para::Modulable, Para::Number(1.0)],
        "noise" => vec![Para::Number(42.0)],
        "envperc" => vec![Para::Number(0.01), Para::Number(0.1)],
        "sampler" => {
            // check potential errors
            if !samples_dict.contains_key(&paras.as_str().replace("\\", "")) {
                let p = paras.next().unwrap();
                let pos = (p.as_span().start(), p.as_span().end());
                return Err(GlicolError::SampleNotExistError(pos))
            }
            vec![]
        }, // bypass the process_parameters
        "seq" => vec![],
        "shape"=> vec![],
        "script" => vec![],
        "speed" => vec![Para::Modulable],
        "choose" => { vec![] },
        "delayn" => vec![Para::Number(1.0)],
        "delay" => vec![Para::Modulable],
        "onepole" => vec![Para::Modulable],
        "comb" => vec![Para::Number(10.), Para::Number(0.9), Para::Number(0.5), Para::Number(0.5)],
        "apfdecay" => vec![Para::Number(10.), Para::Number(0.8)],
        "apfgain" => vec![Para::Modulable, Para::Number(0.5)],
        "pan" => vec![Para::Modulable],
        "balance" => vec![Para::Modulable, Para::Number(0.5), Para::Modulable, Para::Number(0.5)],
        "pha" => vec![Para::Modulable],
        "pass" => vec![],
        _ => {
            match paras.next() {
                Some(_) => {vec![]},
                None => return Err(GlicolError::NodeNameError((name.to_string(), pos.0, pos.1)))
            }
        },
    };

    // println!("{:?}", paras);
    // this func checks if the parameters are correct
    let (p, mut refs) = process_parameters(paras, modulable)?;
    println!("process_parameters para result: {:?}", p);

    if alias == "seq" {refs = process_seq(paras)?.2}
    if alias == "pass" {refs = vec![name.to_owned()]}
    
    let nodedata = match alias {
        "script" => Script::<N>::new().code(paras.as_str().replace("\"", "")).build(),
        "sin" => sin_osc!(N => {freq: get_num(&p[0]), sr: sr}),
        "saw" => saw_osc!(N => {freq: get_num(&p[0]), sr: sr}),
        "squ" => squ_osc!(N => {freq: get_num(&p[0]), sr: sr}),
        "tri" => tri_osc!(N => {freq: get_num(&p[0]), sr: sr}),
        "const_sig" => const_sig!(N => get_num(&p[0])),
        "mul" => mul!(N => get_num(&p[0])),
        "add" => add!(N => get_num(&p[0])),
        "rlpf" => rlpf!(N => {cutoff: get_num(&p[0]), q: get_num(&p[1]), sr: sr}),
        "rhpf" => rhpf!(N => {cutoff: get_num(&p[0]), q: get_num(&p[1]), sr: sr}),

        "noise" => noise!(N => get_num(&p[0]) as u64),
        "imp" => imp!(N => {freq: get_num(&p[0]), sr: sr}),
        "sampler" => {
            sampler!(N => samples_dict[&paras.as_str().replace("\\", "")])},
        "seq" => {
            // let info = process_seq(paras.as_str()).unwrap();
            seq!(N => {events: process_seq(paras)?.0, sidechain_lib: process_seq(paras)?.1, sr: sr, bpm: bpm})
        },
        "shape" => shape!(N => {sr: sr, points: get_shape_points(paras)?}),
        "speed" => speed!(N => get_num(&p[0])),
        "choose" => choose!(N => get_notes(paras)?),
        "delayn" => delayn!(N => get_num(&p[0]) as usize),
        "delay" => delay!(N => {delay: get_num(&p[0]), sr: sr}),
        "onepole" => onepole!(N => get_num(&p[0])),
        "comb" => comb!(N => {delay: get_num(&p[0]), gain: get_num(&p[1]), feedforward: get_num(&p[2]), feedback: get_num(&p[3])}),
        "apfdecay" => apfdecay!(N => {delay: get_num(&p[0]), decay: get_num(&p[1])}),
        "apfgain" => apfgain!(N => {delay: get_num(&p[0]), gain: get_num(&p[1])}),
        "pan" => pan!(N => get_num(&p[0])),
        "balance" => balance!(N => get_num(&p[1])),
        "pha" => phasor!(N => {freq: get_num(&p[0]), sr: sr}),
        "pass" => Pass::<N>::new(),
        "envperc" => envperc!(N => {attack: get_num(&p[0]), decay: get_num(&p[1]), sr: sr}),
        _ => {
            // let a = paras.next().unwrap();
            return Err(GlicolError::NodeNameError((name.to_owned(), 0,0)))
        }
        // "buf" => Buf::new(&mut paras, 
        //     samples_dict)?,
        // "linrange" => LinRange::new(&mut paras)?,
        // "pha" => Phasor::new(&mut paras)?,
        // "state" => State::new(&mut paras)?,
        // "monosum" => MonoSum::new(&mut paras)?,
    };
    Ok((nodedata, refs))
}


#[derive(Debug, Clone, PartialEq)]
/// Parameter of a node can be f32, String, or NodeIndex for sidechain
pub enum Para {
    Number(f32),
    Symbol(String),
    Index(NodeIndex),
    Modulable
}

fn get_num(p: &Para) -> f32 {
    match p {
        Para::Number(v) => *v,
        Para::Modulable => 0.0,
        _ => 0.0
    }
}

type Events = Vec::<(f64, String)>;
type Sidechain = HashMap::<String, usize>;

fn get_shape_points(paras: &mut Pairs<Rule>) -> Result<Vec<(f32, f32)>, GlicolError> {
    // println!("\n\nget shape points from {:?}\n\n", paras.as_str());
    let pattern = paras.clone().as_str();
    let p = paras.clone().next();
    
    let pos = match p {
        Some(p) => (p.as_span().start(), p.as_span().end()),
        None => (0,0)
    };

    let points_str =  paras.as_str().split('|');
    let mut points = vec![];
    for point_str in points_str {
        let v_full = point_str.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let mut v = v_full.split(",");
        // let v2 = v.map(|s| s.parse::<f32>() ).collect();
        let x_str = v.next();
        let x = match x_str {
            Some(value) => {
                if value.parse::<f32>().is_ok() {
                    value.parse::<f32>().unwrap()
                } else {
                    return Err(GlicolError::ParameterError(pos))
                }
            },
            None => {
                return Err(GlicolError::InsufficientParameter(pos))
            }
        };
        let y_str = v.next();
        let y = match y_str {
            Some(value) => {
                if value.parse::<f32>().is_ok() {
                    value.parse::<f32>().unwrap()
                } else {
                    return Err(GlicolError::ParameterError(pos))
                }
            },
            None => {
                return Err(GlicolError::InsufficientParameter(pos))
            }
        };
        points.push((x, y));
    }
    Ok(points)
}

fn process_seq(paras: &mut Pairs<Rule>) -> Result<(Events, Sidechain, Vec<String>), GlicolError> {
    let pattern = paras.clone().as_str();
    let p = paras.clone().next();
    
    let pos = match p {
        Some(p) => (p.as_span().start(), p.as_span().end()),
        None => (0,0)
    };

    // println!("pos {:?}", pos);

    let mut events = Vec::<(f64, String)>::new();
    let mut sidechain_count = 0;
    let mut sidechains = Vec::new();
    let mut sidechain_lib = Sidechain::new();
    let split: Vec<&str> = pattern.split(" ").collect();
    let len_by_space = split.len();
    let compound_unit = 1.0 / len_by_space as f64;

    for (i, compound) in split.iter().enumerate() {
        let c = compound.replace("_", "$_$");
        let notes = c.split("$").filter(|x|x!=&"").collect::<Vec<_>>();

        let notes_len = notes.len();
        for (j, x) in notes.iter().enumerate() {
            let relative_time = i as f64 / len_by_space as f64 
            + (j as f64/ notes_len as f64 ) * compound_unit;

            // println!("x is {}", x);
            
            if !x.parse::<f32>().is_ok() && x != &"_" && !x.starts_with('~') {
                return Err(GlicolError::ParameterError(pos))
            };

            if x.starts_with('~') {
                sidechains.push(x.to_string());
                sidechain_lib.insert(x.to_string(), sidechain_count);
                sidechain_count += 1;
                events.push((relative_time, x.to_string()))
            };
            if x != &"_" {
                events.push((relative_time, x.to_string()))
            } 
        }
    }
    // println!("event: {:?}", events);
    Ok((events, sidechain_lib, sidechains))
}

fn get_notes(paras: &mut Pairs<Rule>) -> Result<Vec::<f32>, GlicolError> {
    let split: Vec<&str> = paras.as_str().split(" ").collect();
    let mut note_list = Vec::<f32>::new();
    // println!("split{:?}", split);
    for note in split {
        match note.parse::<f32>() {
            Ok(v) => note_list.push(v),
            Err(_) => {
                let p = paras.next().unwrap();
                let pos = (p.as_span().start(), p.as_span().end());
                return Err(GlicolError::ParameterError(pos))
            }
        }
    }
    // println!("note_list{:?}", note_list);
    Ok(note_list)
}

pub fn process_parameters(paras: &mut Pairs<Rule>, mut modulable: Vec<Para>) -> Result<(Vec<Para>, Vec<String>), GlicolError> {
    let mut refs = vec![];
    let info = paras.clone().as_str();
    // println!("process_parameters {:?}{:?}", paras.as_str(), modulable);
    for i in 0..modulable.len() {
        let para = paras.next();
        let mut pos = (0, 0);
        match para {
            Some(p) => {
                pos = (p.as_span().start(), p.as_span().end());
                let key = p.as_str();
                match key.parse::<f32>() {
                    Ok(v) => modulable[i] = Para::Number(v),
                    Err(_) => {
                        if key.contains("~") {
                            if modulable[i] != Para::Modulable { 
                                println!("process_parameters key {:?}", key);
                                return Err(GlicolError::NotModuableError(pos)) 
                            } else {
                                refs.push(key.to_string());
                            }
                        } else if key.contains("\\") {
                            modulable[i] =  Para::Symbol(key.to_string())
                        } else {
                            return Err(GlicolError::ParameterError(pos))
                        }
                    }
                }
            },
            None => {
                println!("need more paras in processing paras {}", info);
                return Err(GlicolError::InsufficientParameter(pos))
            }
        // .chars().filter(|c| !c.is_whitespace()).collect();
        };
    };
    return Ok((modulable, refs))
}

pub struct SimpleGraph<const N: usize> {
    pub graph: GlicolGraph<N>,
    processor: GlicolProcessor<N>,
    clock: NodeIndex,
    elapsed_samples: usize,
    pub node_by_chain: HashMap<String, Vec<NodeIndex>>,
}

impl<const N: usize> SimpleGraph<N> {

    //, sr: usize, bpm: f32
    pub fn new(code: &str) -> Self {
        let mut graph = GlicolGraph::<N>::with_capacity(1024, 1024);
            // let processor = GlicolProcessor::with_capacity(1024);
        let mut sidechains_list = Vec::<(NodeIndex, String)>::new();
        let mut node_by_chain = HashMap::new();

        let clock = graph.add_node(
            NodeData::new1(BoxedNodeSend::<N>::new(Clock{})));
        let audio_in = graph.add_node(
                NodeData::new1(BoxedNodeSend::<N>::new(AudioIn{})));
        node_by_chain.insert(
            "~input".to_string(),
            vec![audio_in]
        );

        // println!("code in simplegraph {}", code);
        let mut parsing_result = GlicolParser::parse(Rule::block, code).unwrap();
        let mut current_ref_name: &str = "";

        // add nodes to nodes chain vectors in the HashMap with ref as key
        for line in parsing_result.next().unwrap().into_inner() {
            let inner_rules = line.into_inner();
            for element in inner_rules {
                match element.as_rule() {
                    Rule::reference => {
                        current_ref_name = element.as_str();
                    },
                    Rule::chain => {
                        // self.all_refs.push(current_ref_name.to_string());
                        let refname = current_ref_name.to_string();
                        
                        for node in element.into_inner() {
                            let mut paras = node.into_inner();
                            // let id: String = paras.as_str().to_string()
                            // .chars().filter(|c| !c.is_whitespace()).collect();
                            let first = paras.next().unwrap();
                            let pos = (first.as_span().start(), first.as_span().end());
                            let name = first.as_str();
                            // println!("name inside {}",name );

                            // if the name is a ref
                            let dest = match first.as_rule() {
                                Rule::paras => format!("@rev{}", first.as_str()),
                                _ => "".to_string()
                            };
                            // println!("dest {}",dest);
                            let (node_data, sidechains) = make_node(
                                name, &mut paras, pos,
                                &HashMap::new(),
                                44100,
                                120.0
                            ).unwrap();

                            let node_index = graph.add_node(node_data);

                            // println!("name {} got thie index {:?}", name, node_index);
                                
                            if !node_by_chain.contains_key(&refname) {
                                // head of chain
                                node_by_chain.insert(refname.clone(),
                                vec![node_index]);
                            } else {
                                let mut list = node_by_chain[&refname].clone();
                                if &dest != "" {
                                    sidechains_list.push(
                                        (*list.last().unwrap(), 
                                        dest));
                                };
                                list.push(node_index);
                                node_by_chain.insert(
                                    refname.clone(),list);
                            };
                            for sidechain in sidechains.into_iter() {
                                sidechains_list.push(
                                    (node_index, sidechain));
                            };
                        }
                    },
                    _ => ()
                }
            }
        }
        // finish parsing, move to handle edge connection

        // connect clocks to all the nodes
        for (refname, nodes) in &node_by_chain {
            if refname != "~input" {
                for n in nodes {
                    graph.add_edge(clock, *n,());
                }
            }
        }

        // make edges in each chain
        for (_refname, node_chains) in &node_by_chain {
            if node_chains.len() >= 2 {
                // println!("connect for _refname {} node_chains {:?}", _refname, node_chains);
                graph.add_edge(node_chains[0],node_chains[1],());
                for i in 0..(node_chains.len()-2) {
                    graph.add_edge(node_chains[i+1],node_chains[i+2],());
                };
            };
        }
        
        // println!("sidechain_list {:?}", sidechains_list);
        // make edges cross chain
        for pair in &sidechains_list {
            // println!("work on sidechain pair: {:?}", pair);
            if pair.1.contains("@rev") {
                let name = &pair.1[4..];
                // println!("reversed connection for {}", name);
                if !node_by_chain.contains_key(name) {
                    panic!("NonExistControlNodeError {}", name.to_string());
                }
                let control_node = *node_by_chain[name].last().unwrap();
                println!("reversed connection for {} {:?} {:?}", name, pair.0, control_node);
                graph.add_edge(pair.0, control_node, ());
            } else {
                if !node_by_chain.contains_key(&pair.1) {
                    panic!("NonExistControlNodeError {}", pair.1.to_string());
                }
                let control_node = *node_by_chain[&pair.1].last().unwrap();
                graph.add_edge(control_node, pair.0, ());
            }
        };
        // println!("node_by_chain {:?}", node_by_chain);
        Self {
            graph,
            clock,
            processor: GlicolProcessor::with_capacity(1024),
            elapsed_samples: 0,
            node_by_chain
        }
    }

    pub fn next_block(&mut self, inbuf: &mut [f32]) -> [f32; 256] {
        let mut output: [f32; 256] = [0.0; 256];

        // println!("&self.node_by_chain in SimpleGraph {:?}", &self.node_by_chain);
        // process 0..128
        for (refname, v) in &self.node_by_chain {
            if refname.contains("~") {
                continue;
            }
            // this must be inside
            self.graph[self.clock].buffers[0][0] = self.elapsed_samples as f32;
            for i in 0..N {
                self.graph[
                    self.node_by_chain["~input"][0]
                ].buffers[0][i] = inbuf[i];
            }
            // println!("*v.last().unwrap(){:?}",*v.last().unwrap());
            self.processor.process(&mut self.graph, *v.last().unwrap());
        }

        // sendout 0..128
        for (refname, v) in &self.node_by_chain {
            if refname.contains("~") {
                continue;
            }
            let bufleft = &self.graph[*v.last().unwrap()].buffers[0];
            let bufright = match &self.graph[*v.last().unwrap()].buffers.len() {
                1 => {bufleft},
                2 => {&self.graph[*v.last().unwrap()].buffers[1]},
                _ => {unimplemented!("// no multi-chan for now")}
            };

            for i in 0..N {
                output[i] += bufleft[i];
                output[i+N] += bufright[i];
            }
        }
        self.elapsed_samples += N;
        output
    }
}