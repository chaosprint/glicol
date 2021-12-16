use dasp_graph::{NodeData, BoxedNodeSend};
use glicol::*;
use glicol_synth::*;
use glicol_synth::operation::{mul::*, add::*};
use glicol_synth::oscillator::sin_osc::*;
use glicol_synth::signal::const_sig::*;
use glicol_synth::signal::dummy::Clock;

const SIZE: usize = 128;

fn connection() {
    let mut engine = Engine::new(44100);
    // return a vec of nodeindex
    let i_mod = engine.make_chain(vec![sin_osc!(SIZE => {freq: 4410.0}), mul!(SIZE => 0.5), add!(0.5)]);
    let out = engine.make_chain(vec![const_sig!(20.0), mul!(SIZE)]);
    engine.make_edge(i_mod[2], out[1]);
    engine.process(out[1]); // this is a simplified method for calling processor on graph
    println!("First block {:?}", engine.graph[out[1]].buffers);
}

fn main() {
    connection();
}