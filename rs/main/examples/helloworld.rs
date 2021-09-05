use glicol_synth::*;
use glicol_synth::oscillator::sin_osc::*;
use glicol::Engine;

fn main () {
    let mut engine = Engine::<128>::new(44100);
    // TODOs: this shoud be 
    // let mut engine = Engine::new().sr(44100).seed(42).build();

    let i_source = engine.graph.add_node(sin_osc!(128 => {freq: 440.0}));
    engine.processor.process(&mut engine.graph, i_source);
    println!("{:?}", engine.graph[i_source].buffers[0]);
}