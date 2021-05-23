use glicol::{Engine, sin, mono_node};
use glicol::node::oscillator::SinOsc;
use dasp_graph::BoxedNodeSend;
use dasp_graph::NodeData;

fn main () {
    let mut engine = Engine::new(44100);
    let i_source = engine.graph.add_node(sin!{freq: 440.0});
    engine.processor.process(&mut engine.graph, i_source);
    println!("{:?}", engine.graph[i_source].buffers[0]);
}