use dasp_graph::{NodeData, BoxedNodeSend};
use glicol::*;
use glicol_synth::*;
use glicol_synth::operation::{mul::*, add::*};
use glicol_synth::oscillator::sin_osc::*;
use glicol_synth::signal::const_sig::*;
use glicol_synth::signal::dummy::Clock;

const SIZE: usize = 128;

// TODO: make graph is one thing, but we need to update the graph
// It's about adding nodes, remove nodes, and modify nodes
// adding or removing are relatively easier
// for modifying the value, its better to have a dummy node and modify the node directly
fn set_paras() {
    let mut engine = Engine::new(44100);
    let out = chain!([const_sig!(42.), mul!(SIZE=>1.)] in engine); // yet another way to make chain
    
    // Clock is a dummy node that requires mannual settings
    let dummy = engine.graph.add_node(  NodeData::new1(BoxedNodeSend::new(Clock{}))  );
    engine.make_edge(dummy, out[1]);
    // ... mannual setting, e.g. by a GUI callback
    engine.graph[dummy].buffers[0][0] = 0.5;
    engine.graph[dummy].buffers[0][1] = 0.4;
    // ...
    engine.process(out[1]);
    println!("\n\nFirst block {:?}\n\nThe `mul 1.0` is completely overwritten.", engine.graph[out[1]].buffers);
}

fn main() {
    // raw();
    set_paras();
}