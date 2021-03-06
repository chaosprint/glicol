use glicol_synth::*;
use glicol_synth::oscillator::sin_osc::*;
use glicol::Engine;

fn main () {
    let mut engine = Engine::new(44100);
    let i_source = engine.graph.add_node(sin_osc!({freq: 440.0}));
    engine.processor.process(&mut engine.graph, i_source);
    println!("{:?}", engine.graph[i_source].buffers[0]);
}