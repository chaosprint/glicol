use glicol::*;
use glicol::node::const_sig::*;

fn main() {
    let mut engine = Engine::new(44100);
    let i = engine.graph.add_node(const_sig!(0.5));
    engine.processor.process(&mut engine.graph, i);
    println!("First block of buffer: {:?}", engine.graph[i].buffers[0]);
}