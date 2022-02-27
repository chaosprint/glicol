use petgraph::stable_graph::{StableDiGraph};
use glicol_synth::{NodeData, BoxedNodeSend, Processor, Buffer, Input, Node, node::Pass};

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

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

#[derive(Debug, Copy, Clone)]
struct Mul<const N:usize> { val: f32 }

impl<const N:usize> Mul<N> {
    pub fn new(val: f32) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::new1( BoxedNodeSend::<N>::new( Self { val } ) )
    }
}

impl<const N:usize> Node<N> for Mul<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = inputs[0].buffers()[0][i] * self.val;
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        if info.0 == 0 {
            self.val = info.1.parse::<f32>().unwrap();
        }
    }
}


#[derive(Debug, Copy, Clone)]
struct Add<const N:usize> { val: f32 }

impl<const N:usize> Add<N> {
    pub fn new(val: f32) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::new1( BoxedNodeSend::<N>::new( Self { val } ) )
    }
}

impl<const N:usize> Node<N> for Add<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = inputs[0].buffers()[0][i] + self.val;
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        if info.0 == 0 {
            self.val = info.1.parse::<f32>().unwrap();
        }
    }
}


fn main() {
    let mut graph = GlicolGraph::<128>::with_capacity(1024, 1024);

    // bug before: if we have a graph with: node_a -> node_b; node_a -> node_c; the node will be processed twice.
    let node_index_a = graph.add_node( Counter::<128>::new());
    let node_index_b = graph.add_node( Mul::<128>::new(0.5));
    // let node_index_c = graph.add_node( Add::<128>::new(0.3));
    // let node_index_d = graph.add_node(  NodeData::new1( BoxedNodeSend::<128>::new( Pass { } ) ) );
    graph.add_edge(node_index_a, node_index_b, ());
    // graph.add_edge(node_index_a, node_index_c, ());
    // graph.add_edge(node_index_b, node_index_d, ());
    // graph.add_edge(node_index_c, node_index_d, ());

    let mut processor = GlicolProcessor::with_capacity(1024);

    // if only process once
    processor.process(&mut graph, node_index_b);
    println!("node_index_a result {:?}", graph[node_index_b].buffers);

    // first round 
    // processor.process(&mut graph, node_index_b);
    // processor.process(&mut graph, node_index_c);
    // println!("node_index_a result {:?}", graph[node_index_a].buffers);
    // println!("node_index_b result {:?}", graph[node_index_b].buffers);
    // println!("node_index_c result {:?}", graph[node_index_c].buffers);

    // second round    
    // processor.processed.clear(); // this alters the behaviour
    // graph[node_index_a].node.send_msg((0, "440")); // this also alters the behaviour
    // processor.process(&mut graph, node_index_b);
    // processor.process(&mut graph, node_index_c);
    // println!("result after send msg node_index_a {:?}", graph[node_index_a].buffers);
    // println!("result after send msg node_index_b {:?}", graph[node_index_b].buffers);
    // println!("result after send msg node_index_c {:?}", graph[node_index_c].buffers);
}