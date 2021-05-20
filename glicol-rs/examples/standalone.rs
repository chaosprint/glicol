use glicol::Engine;
use glicol::node::oscillator::{SinOsc};
use glicol::node::Para::Number as Num;

fn main() {
    let mut engine = Engine::new();
    let i = engine.graph.add_node(SinOsc::new(Num(440.0)));
    engine.processor.process(&mut engine.graph, i);
    println!("First block of buffer: {:?}", engine.graph[i].buffers[0]);
}