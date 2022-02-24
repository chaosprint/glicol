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
struct Phasor<const N:usize> { freq: f32, sr: usize, phase: f32 }

impl<const N:usize> Phasor<N> {
    pub fn new(freq: f32) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::new1( BoxedNodeSend::<N>::new( Self {freq, sr: 44100, phase: 0.0} ) )
    }
}

impl<const N:usize> Node<N> for ConstSig<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            phase += freq / 44100.0;
            output[0][i] = self.val;
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
    let index = graph.add_node( ConstSig::<128>::new(42.)) ;
    let mut processor = GlicolProcessor::with_capacity(1024);
    processor.process(&mut graph, index);
    println!("result {:?}", graph[index].buffers);
    graph[index].node.send_msg((0, "440.0"));
    processor.process(&mut graph, index);
    println!("result after send msg {:?}", graph[index].buffers);
}