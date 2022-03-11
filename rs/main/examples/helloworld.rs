use glicol_synth::*;
use glicol_synth::oscillator::sin_osc::*;
use glicol_synth::dynamic::script::*;
use glicol_synth::signal::const_sig::*;
use glicol::Engine;

fn main () {
    let mut engine = Engine::<128>::new();
    // let i_source = engine.graph.add_node(sin_osc!(128 => {freq: 440.0}));
    let i_source = engine.graph.add_node(Script::new().code(r#""#.to_owned()).build());
    // let i_source = engine.graph.add_node(ConstSig::<128>::new(81.));
    engine.processor.process(&mut engine.graph, i_source);
    println!("{:?}", engine.graph[i_source].buffers[0]);
}