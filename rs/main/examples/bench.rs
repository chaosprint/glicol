use glicol_synth::*;
use glicol_synth::oscillator::sin_osc::*;
use glicol_synth::dynamic::script::*;
use glicol_synth::signal::const_sig::*;
use glicol::Engine;
use std::time::{Duration, Instant};
fn main () {
    let mut engine = Engine::<128>::new(44100);
    // TODOs: this shoud be 
    // let mut engine = Engine::new().sr(44100).seed(42).build();

    // let i_source = engine.graph.add_node(sin_osc!(128 => {freq: 440.0}));
    let i_source = engine.graph.add_node(Script::new().code(r#"
        // output.clear();
        // for i in 0..128 {
        //     output.push(sin(2*PI()*phase/(44100/440)));
        //     phase += 1;
        // };
        // output.pad(128, 0.0);
        // output.map(|v, i| sin(2*PI()*(phase+i)/(44100/440)) );
        // phase += 128;
        output.clear();
        for i in 0..128 {
            output.push( sin(2*PI()*phase) );
            phase += 440.0 / 44100.0;
            // if phase > 1.0 {
            //     phase -= 1.0
            // }
        };
        output
    "#.to_owned()).build());
    // let i_source = engine.graph.add_node(ConstSig::<128>::new(81.));
    for i in 0..200 {
        let start = Instant::now();
        engine.processor.process(&mut engine.graph, i_source);
        // println!("{:?}", engine.graph[i_source].buffers[0]);
        println!("Iteration {} costs {:?}", i, start.elapsed());
    }
}