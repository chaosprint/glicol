// use glicol_synth::*;
// use glicol_synth::oscillator::sin_osc::*;
// use glicol_synth::dynamic::script::*;
// use glicol_synth::signal::const_sig::*;
// use glicol::Engine;
// use std::time::{Duration, Instant};
// fn main () {
//     let mut engine = Engine::<128>::new(44100);
//     // TODOs: this shoud be 
//     // let mut engine = Engine::new().sr(44100).seed(42).build();

//     // let i_source = engine.graph.add_node(sin_osc!(128 => {freq: 440.0}));
//     let i_source = engine.graph.add_node(Script::new().code("
//     let x = [];
//     x.pad(128, 42.0);
//     x.map(|n|200.0);
//     x".to_owned()).build());
//     // let i_source = engine.graph.add_node(ConstSig::<128>::new(81.));
//     for _ in 0..10 {
//         let start = Instant::now();
//         engine.processor.process(&mut engine.graph, i_source);
//         println!("{:?}", engine.graph[i_source].buffers[0]);
//         println!("Time elapsed in expensive_function() is: {:?}", start.elapsed());
//     }
// }