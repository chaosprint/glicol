use dasp_graph::{NodeData, BoxedNodeSend};
use glicol::*;
use glicol::node::mul::*;
use glicol::node::add::*;
use glicol::node::sin_osc::*;
use glicol::node::const_sig::*;
use glicol::node::system::Clock;

// very raw API
fn raw() {
    let mut engine = Engine::new(44100);
    let index_source = engine.graph.add_node(
        SinOsc::new().freq(440.0).build()
        // this is equivalant to
        // sin_osc!({freq: 440.0})
    );
    let index_mul = engine.graph.add_node(mul!(0.5));
    engine.graph.add_edge(index_source, index_mul, ());
    engine.processor.process(&mut engine.graph, index_source);
    println!("First block, channel 0: {:?}", engine.graph[index_source].buffers[0]);
}

// more ergonomic for making connections
fn connection() {
    let mut engine = Engine::new(44100);
    // return a vec of nodeindex
    let i_mod = engine.make_chain(vec![sin_osc!({freq: 10.0}), mul!(0.5), add!(0.5)]);
    let out = engine.make_chain(vec![sin_osc!({freq: 440.0}), mul!()]);
    engine.make_edge(i_mod[2], out[1]);
    engine.process(out[1]); // this is a simplified method for calling processor on graph
    println!("First block {:?}", engine.graph[out[1]].buffers);
}

// TODO: make graph is one thing, but we need to update the graph
// It's about adding nodes, remove nodes, and modify nodes
// adding or removing are relatively easier
// for modifying the value, its better to have a dummy node and modify the node directly
fn set_paras() {
    let mut engine = Engine::new(44100);
    let out = chain!([const_sig!(42.), mul!(1.)] in engine); // yet another way to chain
    
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
    // connection();
    set_paras();
}