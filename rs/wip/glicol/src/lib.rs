pub mod synth;

// use synth::oscillator::{SinOsc, SawOsc};
use synth::signal::{ConstSig};

use std::collections::HashMap;
use petgraph::graph::{NodeIndex};
use petgraph::stable_graph::{StableDiGraph};
use dasp_graph::{NodeData, BoxedNodeSend, Processor,  }; //Input, NodeBuffer

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

pub struct Engine<'a, const N: usize> {
    pub graph: GlicolGraph<N>,
    pub processor: GlicolProcessor<N>,
    index_info: HashMap<&'a str, Vec<NodeIndex>>,
}

impl<const N: usize> Engine<'static, N> {
    pub fn new() -> Self {
        Self {
            graph: GlicolGraph::<N>::with_capacity(1024, 1024),
            processor: GlicolProcessor::<N>::with_capacity(1024),
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

    // convert the ast from the parsing to a graph full of nodes
    // handle the connection later for lazy evaluation
    pub fn ast_to_graph<'a>(&mut self, ast: HashMap< &'static str, Vec<GlicolNodeInfo<'static> > > ) {

        for (key, chain) in &ast {
            
            let node_chain: Vec<NodeIndex> = chain.iter().map(|node_info| -> NodeIndex {
                match node_info {
                    // need to consider how this step should be safe
                    GlicolNodeInfo::ConstSig(v) => self.graph.add_node( ConstSig::<N>::new(42.) ),
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
