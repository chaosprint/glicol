pub mod synth;

use synth::{
    oscillator::{SinOsc},
    signal::{ConstSig},
    operator::{Mul},
};

use std::collections::HashMap;
use petgraph::graph::{NodeIndex};
use petgraph::stable_graph::{StableDiGraph};
use dasp_graph::{NodeData, BoxedNodeSend, Processor,  }; //Input, NodeBuffer

// extern crate glicol_parser;
use glicol_parser::{single_chain};
use slice_diff_patch::*;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

pub struct Engine<'a, const N: usize> {
    pub graph: GlicolGraph<N>,
    pub processor: GlicolProcessor<N>,
    ast: HashMap<&'a str, Vec<(&'a str, &'a str)>>,
    index_info: HashMap<&'a str, Vec<NodeIndex>>,
}

impl<const N: usize> Engine<'static, N> {
    pub fn new() -> Self {
        Self {
            graph: GlicolGraph::<N>::with_capacity(1024, 1024),
            processor: GlicolProcessor::<N>::with_capacity(1024),
            ast: HashMap::new(),
            index_info: HashMap::new(),
        }
    }

    pub fn send_msg(
        &mut self, 
        chain_name: &str, 
        node_index_in_chain: u8, 
        msg: (u8, &str)
    ) {
        // we store the chain order in a vec?
        // does the order matters?
        let index = self.index_info[chain_name][node_index_in_chain as usize];
        self.graph[index].node.send_msg(msg);
    }

    pub fn set_code(&mut self, code: &'static str) {
        // can panic now with error TODO: error handling
        let result = single_chain(code).unwrap().1; // the result should be Ok(("unparsed str", ("name", [Node]) ))
        let name = result.0;
        let node_chain = result.1;
        if self.ast.contains_key(name) {
            let mut old = vec![];
            let mut new = vec![];

            for pair in &self.ast[name] {
                old.push(pair.0);
                old.push(pair.1);
            }

            for pair in node_chain {
                new.push(pair.0);
                new.push(pair.1);
            }

            // let old: Vec<String> = self.ast[name].iter().map(|name_para_pair|{
            //     let mut pair_str = name_para_pair.0.to_owned();
            //     pair_str.push_str(" ");
            //     pair_str.push_str(name_para_pair.1);
            //     pair_str
            // }).collect();

            // let new: Vec<String> = node_chain.iter().map(|name_para_pair|{
            //     let mut pair_str = name_para_pair.0.to_owned();
            //     pair_str.push_str(" ");
            //     pair_str.push_str(name_para_pair.1);
            //     pair_str
            // }).collect();

            let diff = diff_diff(&old, &new);
            let lcs = lcs_diff(&old, &new);
            let wu = wu_diff(&old, &new);

            println!("old {:?}", &old);
            println!("new {:?}", &new);
            println!("diff {:?}", diff);
            println!("lcs {:?}", lcs);
            println!("wu {:?}", wu);
            for change in lcs {
                match change {
                    Change::Update((index, value)) => {
                        let nodeplace = (index - 1) / 2;
                        println!("nodeindex{:?}", nodeplace);
                        println!("self.index_info[name][nodeindex] {:?}", self.index_info[name][nodeplace]);
                        let nodeindex = self.index_info[name][nodeplace];
                        println!("value{:?}", value);
                        self.graph[nodeindex].node.send_msg((0, value))
                    },
                    Change::Remove(index) => {
                        // delete both the node in ast and self.nodeindex_hashmap

                    },
                    Change::Insert((index, value)) => {
                        
                    }
                }
            }
            panic!();
            // lcs should be [Update(), Remove() ]
            // do the action to the node index self.node_chain[name]
            
        } else {
            // println!("node_chain{:?}, " , &node_chain);
            
            let chain: Vec<NodeIndex> = node_chain.iter().map(|node_info: &(&str, &str)| -> NodeIndex {
                match node_info.0 {
                    // need to consider how this step should be safe
                    "const_sig" => self.graph.add_node( ConstSig::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
                    "sin" => self.graph.add_node( SinOsc::<N>::new().freq(node_info.1.parse::<f32>().unwrap()).build() ),
                    "mul" => self.graph.add_node( Mul::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
                    _ => unimplemented!()
                }
            }).collect();
            self.ast.insert(name, node_chain);
            self.index_info.insert(name, chain);
            // panic!()
            // self.ast_to_graph();
            // let node_chain: Vec<NodeIndex> = chain.iter().map(|node_info: &(&str, &str)| -> NodeIndex {
            //     match node_info.0 {
            //         // need to consider how this step should be safe
            //         "const_sig" => self.graph.add_node( ConstSig::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
            //         "sin" => self.graph.add_node( SinOsc::<N>::new().freq(node_info.1.parse::<f32>().unwrap()).build() ),
            //         "mul" => self.graph.add_node( Mul::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
            //         _ => unimplemented!()
            //     }
            // }).collect();
        }
    }

    // convert the ast from the parsing to a graph full of nodes
    // handle the connection later for lazy evaluation 
    // ast: &HashMap< &'static str, Vec<GlicolNodeInfo<'static> > >
    // pub fn ast_to_graph<'a>(&mut self) {

    //     for (key, chain) in &self.ast {
            
    //         let node_chain: Vec<NodeIndex> = chain.iter().map(|node_info: &(&str, &str)| -> NodeIndex {
    //             match node_info.0 {
    //                 // need to consider how this step should be safe
    //                 "const_sig" => self.graph.add_node( ConstSig::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
    //                 "sin" => self.graph.add_node( SinOsc::<N>::new().freq(node_info.1.parse::<f32>().unwrap()).build() ),
    //                 "mul" => self.graph.add_node( Mul::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
    //                 _ => unimplemented!()
    //             }
    //         }).collect();
    //         self.index_info.insert(key, node_chain);
    //     }
    // }

    pub fn handle_graph_connection(&mut self) {
        // 1. connect pararell nodes in each chain

        // 2. finish the sidechain connection

        // self.index = self.graph.add_node( ConstSig::<N>::new(42.) )
    }

    pub fn next_block(&mut self) {  //  -> &Vec<Buffer<N>> 
        for (name, index_list) in &self.index_info {
            // let index_list = &chain;
            if name.chars().next().unwrap() != '~' {
                self.processor.process(&mut self.graph, *index_list.last().unwrap());
                println!("result {:?}", &self.graph[*index_list.last().unwrap()].buffers);
                // &self.graph[self.index].buffers
            }
        }
    }
}
