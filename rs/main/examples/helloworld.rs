use glicol_synth::*;
use glicol_synth::oscillator::sin_osc::*;
use glicol_synth::dynamic::script::*;
use glicol_synth::signal::const_sig::*;
use glicol::Engine;

fn main () {
    let mut engine = Engine::<128>::new(44100);
    // TODOs: this shoud be 
    // let mut engine = Engine::new().sr(44100).seed(42).build();

    let i_source = engine.graph.add_node(sin_osc!(128 => {freq: 440.0}));
    // let i_source = engine.graph.add_node(Script::new().code("
    // let x = [];
    // for i in 0..128 {
    //     let pha = phase / (44100.0 / 440.0);
    //     x.push(sin(pha * 2.0 * PI()));
    //     phase += 1.0;
    // };
    // x".to_owned()).build());
    // let i_source = engine.graph.add_node(ConstSig::<128>::new(81.));
    for _ in 0..10 {
        engine.processor.process(&mut engine.graph, i_source);
        println!("{:?}", engine.graph[i_source].buffers[0]);
    }
}