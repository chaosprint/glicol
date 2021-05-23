use glicol::Engine;
use glicol::node::source::ConstSig;

fn main() {
    let mut engine = Engine::new();
    let i = engine.graph.add_node(ConstSig::new("0.5"));
    engine.processor.process(&mut engine.graph, i);
    println!("First block of buffer: {:?}", engine.graph[i].buffers[0]);
}