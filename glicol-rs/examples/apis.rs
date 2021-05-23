use glicol::*;
use glicol::node::const_sig::*;

// very raw API
fn raw() {
    let mut engine = Engine::new(44100);
    let index_source = engine.graph.add_node(
        SinOsc::new().freq(440.0).build()
        // this is equivalant to
        // sin_osc!{freq: 440.0}
    );
    engine.processor.process(&mut engine.graph, index_source);
    println!("First block, channel 0: {:?}", engine.graph[index_source].buffers[0]);
}

// manually make edge connections
fn connection() {
    // let i_sin_modulator = engine.graph.add_node(
    //     Map( vec![Num(-1.0), Num(1.0), Num(100.0), Num(300.0)] )
    // );
    // engine.graph.add_edge(i_sin_modulator, i_sin_modulator);

    // let index_mod = chain![SinOsc(1.2), Map(1.0)];
    // let index_out = chain![SinOsc(440.), Mul(index_mod)]

    // let i_source = engine.graph.add_node(SinOsc(vec![Num(440.)]));
    // let i_mul = engine.graph.add_node(Mul(vec![]));
    // engine.graph.add_edge(i_sin_a, i_mul);

    // engine.next_stereo::<128>();
    // engine.process();
    // assert_eq!(engine.buffer[0], 0.)
}

// the abovementioned two APIs are not ergonomic enough
fn chain() {
    let engine = Engine::new(44100);

    // less code for users to write in Rust
    // TODO: write the chain method, make_edges, and process method

    // return a vec of nodeindex
    let i_mod = engine.chain([sin_osc!{freq: 10.0}, mul!{val: 0.5}, add!{val: 0.5}]);
    let out = engine.chain([sin_osc!{freq: 440.0}, mul!{}]);
    engine.make_edges([(i_mod[2], out[1])]);
    engine.process(out[1]); // this is a simplified method for calling processor on graph
    // println!("{:?}", engine.get_buffers(out[1]));
}

// make graph is one thing, but we need to update the graph
// It's about adding nodes, remove nodes, and modify nodes
// adding or removing are relatively easier
// for modifying the value, its better to have a dummy node and modify the node directly
fn set_paras() {
    let engine = Engine::new(44100);
    let out = engine.chain([sin_osc!{freq: 440.0}, mul!{}]);
    let dummy = engine.graph.add_node(dummy!{});
    engine.graph.add_edge(dummy, out[1], ()); // don't forget the ()
    
    // ... some GUI interaction callback
    // engine.graph[dummy].buffers[0].iter_mut(|s| s = ...)
}

fn main() {

}