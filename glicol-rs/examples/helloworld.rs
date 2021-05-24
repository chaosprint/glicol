use glicol::*;
use glicol::node::sin_osc::*;
// use dasp_graph::BoxedNodeSend;
// use dasp_graph::NodeData;

fn main () {
    let mut engine = Engine::new(44100);
    let i_source = engine.graph.add_node(sin_osc!({freq: 440.0}));
    engine.processor.process(&mut engine.graph, i_source);
    println!("{:?}", engine.graph[i_source].buffers[0]);
}