pub mod synth;

use synth::oscillator::{SinOsc};
use synth::signal::{ConstSig};

use std::collections::HashMap;
use petgraph::graph::{NodeIndex};
use petgraph::stable_graph::{StableDiGraph};
use dasp_graph::{NodeData, BoxedNodeSend, Processor,  }; //Input, NodeBuffer

extern crate glicol_parser;
use glicol_parser::{single_chain};

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

        // println!("node_chain {:?}", node_chain);
        self.ast.insert(name, node_chain);
        // &self.ast
        self.ast_to_graph();
    }

    // convert the ast from the parsing to a graph full of nodes
    // handle the connection later for lazy evaluation 
    // ast: &HashMap< &'static str, Vec<GlicolNodeInfo<'static> > >
    pub fn ast_to_graph<'a>(&mut self) {

        for (key, chain) in &self.ast {
            
            let node_chain: Vec<NodeIndex> = chain.iter().map(|node_info: &(&str, &str)| -> NodeIndex {
                match node_info.0 {
                    // need to consider how this step should be safe
                    "const_sig" => self.graph.add_node( ConstSig::<N>::new(node_info.1.parse::<f32>().unwrap()) ),
                    "sin" => self.graph.add_node( SinOsc::<N>::new().freq(node_info.1.parse::<f32>().unwrap()).build() ),
                    _ => unimplemented!()
                }
            }).collect();
            self.index_info.insert(key, node_chain);
        }
    }

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
