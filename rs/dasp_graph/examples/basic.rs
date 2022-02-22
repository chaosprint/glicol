use petgraph::graph::{NodeIndex};
use petgraph::stable_graph::{StableDiGraph};
use dasp_graph::{NodeData, BoxedNodeSend, Processor, Buffer, Input, Node};
use core::pin::Pin;
use std::ops::Deref;

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

#[derive(Debug, Copy, Clone)]
struct ConstSig<const N:usize> { val: f32 }

impl<const N:usize> ConstSig<N> {
    pub fn new(val: f32) -> NodeData<BoxedNodeSend<N>, N> {
        // NodeData::new1( Self {val} )
        NodeData::new1( BoxedNodeSend::<N>::new( Self {val} ) )
    }

    pub fn newval(mut self, newval: f32)  {
        self.val = newval
    }
}

impl<const N:usize> Node<N> for ConstSig<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = self.val;
        }
    }
    fn talk(&mut self, info: &str) {
        self.val = info.parse::<f32>().unwrap();
    }
}

// impl<const N: usize> From<BoxedNodeSend<N>> for BoxedNodeSend<N> {
//     fn from() -> Box<dyn Node<N> + Send> {
        
//     }
// }

fn main() {
    let mut graph = GlicolGraph::<128>::with_capacity(1024, 1024);
    let index = graph.add_node( ConstSig::<128>::new(42.)) ;
    let mut processor = GlicolProcessor::with_capacity(1024);
    processor.process(&mut graph, index);
    println!("result {:?}", graph[index].buffers);
    graph[index].node.talk("440.0");
    processor.process(&mut graph, index);
    println!("result {:?}", graph[index].buffers);
}