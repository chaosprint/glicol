use dasp_graph::{NodeData, BoxedNodeSend};
use glicol::*;
use glicol_synth::*;
use glicol_synth::operation::{mul::*, add::*};
use glicol_synth::oscillator::sin_osc::*;
use glicol_synth::signal::const_sig::*;
use glicol_synth::signal::dummy::Clock;

const SIZE: usize = 128;

/// In this example, raw APIs are used, and there is almost no macro used
fn raw() {
    
    // we create an engine using the factory mode
    // this is equivalent to the engine macro;
    let mut engine = Engine::new(44100);

    // add new node to the engine, 
    let index_source = engine.graph.add_node(

        // the node can be built manually like this
        SinOsc::new().freq(440.0).build()
        // this is equivalant to
        // sin_osc!(128 => {freq: 440.0})
        // which is much more convenient
    );

    let index_mul = engine.graph.add_node(mul!(SIZE => 0.5));
    engine.graph.add_edge(index_source, index_mul, ());

    // the previous three sentences, can be replaced by this one sentence

    // let indexes = engine.make_chain(vec![ sin_osc!(128 => {freq: 440.0}), mul!(SIZE => 0.5) ]);
    // or
    // let indexes = chain!([ sin_osc!(128 => {freq: 440.0}), mul!(SIZE => 0.5)] in engine);

    // this is also a raw API relying on dasp_graph
    engine.processor.process(&mut engine.graph, index_source);
    // equavalent to engine.process(indexes[1]);
    println!("First block, channel 0: {:?}", engine.graph[index_source].buffers[0]);
}

// a simplified version
// fn raw() {
//     let mut engine = engine!{sr: 44100, seed: 56, block: 128};
//     let ind = chain!([ sin_osc!(128 => {freq: 440.0}), mul!(SIZE => 0.5)] in engine);
//     engine.process(ind[1]);
//     println!("First block, channel 0: {:?}", engine.graph[ind[1]].buffers[0]);
// }

fn main() {
    raw();
}