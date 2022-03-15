use glicol_synth::{SinOsc, NodeData, BoxedNodeSend, Processor, Message};
use petgraph::stable_graph::{StableDiGraph};

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

fn main() {
    let mut graph = GlicolGraph::<128>::with_capacity(1024, 1024);

    let index = graph.add_node( SinOsc::default().to_boxed_nodedata(1) );

    let mut processor = GlicolProcessor::with_capacity(1024);
    processor.process(&mut graph, index);

    println!("result {:?}", graph[index].buffers);

    graph[index].node.send_msg(Message::SetToNumber((0, 42.)));
    processor.process(&mut graph, index);
    println!("result after send msg {:?}", graph[index].buffers);
}