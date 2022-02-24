use petgraph::stable_graph::{StableDiGraph};
use dasp_graph::{NodeData, BoxedNodeSend, Processor, Buffer, Input, Node};

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
}

impl<const N:usize> Node<N> for ConstSig<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = self.val;
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        if info.0 == 0 {
            self.val = info.1.parse::<f32>().unwrap();
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Counter<const N:usize> { n: usize }

impl<const N:usize> Counter<N> {
    pub fn new() -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::new1( BoxedNodeSend::<N>::new( Self { n:0 } ) )
    }
}

impl<const N:usize> Node<N> for Counter<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = self.n as f32;
            self.n += 1;
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        if info.0 == 0 {
            self.n = info.1.parse::<f32>().unwrap() as usize;
        }
    }
}

fn main() {
    let mut graph = GlicolGraph::<128>::with_capacity(1024, 1024);

    // bug before: if we have a graph with: node_a -> node_b; node_a -> node_c; the node will be processed twice.
    let node_index_a = graph.add_node( Counter::<128>::new());
    let node_index_b = graph.add_node( Counter::<128>::new());
    let node_index_c = graph.add_node( Counter::<128>::new());
    let edge_a_b = graph.add_edge(node_index_a, node_index_b, ());
    let edge_a_c = graph.add_edge(node_index_a, node_index_c, ());

    let mut processor = GlicolProcessor::with_capacity(1024);

    // first round 
    processor.process(&mut graph, node_index_b);
    processor.process(&mut graph, node_index_c);
    println!("result {:?}", graph[node_index_a].buffers);

    // second round    
    processor.processed.clear(); // this alters the behaviour
    // graph[node_index_a].node.send_msg((0, "440")); // this also alters the behaviour
    processor.process(&mut graph, node_index_b);
    processor.process(&mut graph, node_index_c);
    println!("result after send msg {:?}", graph[node_index_a].buffers);
}