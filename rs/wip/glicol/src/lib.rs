pub mod synth;

use synth::{
    oscillator::{SinOsc},
    signal::{ConstSig},
    operator::{Mul},
};

use std::collections::HashMap;
use petgraph::graph::{NodeIndex};
use petgraph::stable_graph::{StableDiGraph};
use dasp_graph::{NodeData, BoxedNodeSend, Processor, node::Sum }; //Input, NodeBuffer

use glicol_parser::*; 
// use pest::Parser;
// use glicol_parser::{single_chain};
use lcs_diff::diff; use lcs_diff::DiffResult;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

pub struct Engine<'a, const N: usize> {
    pub graph: GlicolGraph<N>,
    pub processor: GlicolProcessor<N>,
    code: &'static str,
    ast: HashMap<&'a str, (Vec<&'a str>, Vec<&'a str>)>,
    index_info: HashMap<&'a str, Vec<NodeIndex>>,
    output_index: NodeIndex,
}

impl<const N: usize> Engine<'static, N> {
    pub fn new() -> Self {
        let mut graph = GlicolGraph::<N>::with_capacity(1024, 1024);
        let output_index = graph.add_node(NodeData::new2(BoxedNodeSend::<N>::new(Sum{})));
        Self {
            graph,
            processor: GlicolProcessor::<N>::with_capacity(1024),
            ast: HashMap::new(),
            code: "",
            index_info: HashMap::new(),
            output_index
        }
    }

    pub fn send_msg(
        &mut self, 
        chain_name: &str, 
        node_index_in_chain: u8, 
        msg: (u8, &str)
    ) {
        let index = self.index_info[chain_name][node_index_in_chain as usize];
        self.graph[index].node.send_msg(msg);
    }

    pub fn set_code(&mut self, code: &'static str) {
        self.code = code;
    }

    pub fn parse(&mut self) {
        let new_ast = get_glicol_ast(&self.code).unwrap();
        for (key, node_info_tuple) in &new_ast {
            if self.ast.contains_key(key) {
                let old_chain = &self.ast[key].0;
                let new_chain = &node_info_tuple.0;
                for action in diff(old_chain, new_chain) {
                    match action {
                        DiffResult::Common(v) => {
                            println!("common {:?}", v)
                        },
                        DiffResult::Removed(v) => {
                            println!("Removed {:?}", v)
                        },
                        DiffResult::Added(v) => {
                            println!("Added {:?}", v)
                        },
                    }
                }
                println!("diff {:?}", diff(old_chain, new_chain));
            } else {
                self.ast.insert(key, node_info_tuple.clone());
                self.add_whole_chain(key, node_info_tuple.clone());
            }
        }
    }

    pub fn add_whole_chain(&mut self, key: &'static str, node_info_tuple: (Vec<&'static str>, Vec<&'static str>)) {
        let mut index_list = vec![];
        for i in 0..node_info_tuple.0.len() {
            let index = self.graph.add_node(
                match node_info_tuple.0[i] {
                    "sin" => SinOsc::<N>::new().freq(node_info_tuple.1[i].parse::<f32>().unwrap()).build(),
                    "mul" => Mul::<N>::new(node_info_tuple.1[i].parse::<f32>().unwrap()),
                    "constsig" => ConstSig::<N>::new(node_info_tuple.1[i].parse::<f32>().unwrap()),
                    _ => unimplemented!()
                }
            );
            index_list.push(index);
        }
        self.index_info.insert(key, index_list);
    }

    pub fn handle_connection(&mut self) {
        self.graph.clear_edges();
        for (key, chain) in &self.index_info {
            match chain.len() {
                0 => {},
                1 => {
                    self.graph.add_edge(chain[0], self.output_index, ());
                },
                2 => {
                    self.graph.add_edge(chain[0], chain[1], ());
                    self.graph.add_edge(chain[1], self.output_index, ());
                },
                _ => {
                    for i in 0..chain.len() - 1 {
                        if i == chain.len() - 1 {
                            self.graph.add_edge(chain[i], self.output_index ,());
                        } else {
                            self.graph.add_edge(chain[i],chain[i+1] ,());
                        }
                    }
                }
            }
        }
    }

    pub fn set_code2(&mut self) {
         // can panic now with error TODO: error handling
        //  let result = single_chain(code).unwrap().1; // the result should be Ok(("unparsed str", ("name", [Node]) ))
        //  let name = result.0;
        //  let node_chain = result.1;
 
        //  // the idea is that we get a new ast,
        //  // the ast should be a hashmap.
        //  // { chain_name: [...nodes] }
        //  // we compare the new key vec in the new ast with the old key vec
        //  // with lcs, we get the deleted, added and common
        //  // we further compare the nodes inside those common chains
 
        //  if self.ast.contains_key(name) {
        //      let mut old = vec![];
        //      let mut new = vec![];
 
        //      for pair in &self.ast[name] {
        //          old.push(pair.0);
        //          old.push(pair.1);
        //      }
 
        //      for pair in node_chain {
        //          new.push(pair.0);
        //          new.push(pair.1);
        //      }
 
        //      // let old: Vec<String> = self.ast[name].iter().map(|name_para_pair|{
        //      //     let mut pair_str = name_para_pair.0.to_owned();
        //      //     pair_str.push_str(" ");
        //      //     pair_str.push_str(name_para_pair.1);
        //      //     pair_str
        //      // }).collect();
 
        //      // let new: Vec<String> = node_chain.iter().map(|name_para_pair|{
        //      //     let mut pair_str = name_para_pair.0.to_owned();
        //      //     pair_str.push_str(" ");
        //      //     pair_str.push_str(name_para_pair.1);
        //      //     pair_str
        //      // }).collect();
 
        //      let diff = diff_diff(&old, &new);
        //      let lcs = lcs_diff(&old, &new);
        //      let wu = wu_diff(&old, &new);
 
        //      println!("old {:?}", &old);
        //      println!("new {:?}", &new);
        //      println!("diff {:?}", diff);
        //      println!("lcs {:?}", lcs);
        //      println!("wu {:?}", wu);
        //      for change in lcs {
        //          match change {
        //              Change::Update((index, value)) => {
        //                  let nodeplace = (index - 1) / 2;
        //                  println!("nodeindex{:?}", nodeplace);
        //                  println!("self.index_info[name][nodeindex] {:?}", self.index_info[name][nodeplace]);
        //                  let nodeindex = self.index_info[name][nodeplace];
        //                  println!("value{:?}", value);
        //                  self.graph[nodeindex].node.send_msg((0, value))
        //              },
        //              Change::Remove(index) => {
        //                  // delete both the node in ast and self.nodeindex_hashmap
 
        //              },
        //              Change::Insert((index, value)) => {
                         
        //              }
        //          }
        //      }
        //      panic!();
        //      // lcs should be [Update(), Remove() ]
        //      // do the action to the node index self.node_chain[name]
             
        //  } else {
        //      // println!("node_chain{:?}, " , &node_chain);
             
        //      let chain: Vec<NodeIndex> = node_chain.iter().map(|node_info: &(&str, &str)| -> NodeIndex {
        //          match node_info.0 {
        //              // need to consider how this step should be safe
        //              "const_sig" => self.graph.add_node( ConstSig::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
        //              "sin" => self.graph.add_node( SinOsc::<N>::new().freq(node_info.1.parse::<f32>().unwrap()).build() ),
        //              "mul" => self.graph.add_node( Mul::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
        //              _ => unimplemented!()
        //          }
        //      }).collect();
        //      self.ast.insert(name, node_chain);
        //      self.index_info.insert(name, chain);
        //      // panic!()
        //      // self.ast_to_graph();
        //      // let node_chain: Vec<NodeIndex> = chain.iter().map(|node_info: &(&str, &str)| -> NodeIndex {
        //      //     match node_info.0 {
        //      //         // need to consider how this step should be safe
        //      //         "const_sig" => self.graph.add_node( ConstSig::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
        //      //         "sin" => self.graph.add_node( SinOsc::<N>::new().freq(node_info.1.parse::<f32>().unwrap()).build() ),
        //      //         "mul" => self.graph.add_node( Mul::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
        //      //         _ => unimplemented!()
        //      //     }
        //      // }).collect();
        //  }
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

    // pub fn handle_graph_connection(&mut self) {
    //     // 1. connect pararell nodes in each chain

    //     // 2. finish the sidechain connection

    //     // self.index = self.graph.add_node( ConstSig::<N>::new(42.) )
    // }

    pub fn next_block(&mut self) {  //  -> &Vec<Buffer<N>> 
        // for (name, index_list) in &self.index_info {
            // let index_list = &chain;
            // if name.chars().next().unwrap() != '~' {
            self.processor.process(&mut self.graph, self.output_index);
            println!("result {:?}", &self.graph[self.output_index].buffers);
                // &self.graph[self.index].buffers
            // }
        // }
        // self.processor.processed.clear();
    }
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
