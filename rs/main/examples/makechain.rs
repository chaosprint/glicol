/// Amplitude modulation example, excerpt from the apis.rs
use glicol::*;
use glicol_synth::oscillator::sin_osc::*;
use glicol_synth::operation::mul::*;
use glicol_synth::operation::add::*;
use glicol_synth::*;

fn main () {
    let mut engine = Engine::new(44100);
    // return a vec of nodeindex
    let i_mod = engine.make_chain(vec![sin_osc!(128 => {freq: 10.0}), mul!(128 => 0.5), add!(0.5)]);
    let out = engine.make_chain(vec![sin_osc!(128 => {freq: 440.0}), mul!(128 => 0.0)]);
    engine.make_edge(i_mod[2], out[1]);
    engine.process(out[1]); // this is a simplified method for calling processor on graph
    println!("{:?}", engine.graph[out[1]].buffers);
}