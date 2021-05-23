use glicol::{Engine, sin, mono_node};
use glicol::node::oscillator::SinOsc;
use dasp_graph::BoxedNodeSend;
use dasp_graph::NodeData;

#[test]
// very raw API
fn raw() {
    let mut engine = Engine::new(44100);
    let index_source = engine.graph.add_node(
        NodeData::new1(
            BoxedNodeSend::new(
                SinOsc{freq: 440.0, ..Default::default()}
            )
        )
    );
    engine.processor.process(&mut engine.graph, index_source);
    println!("{:?}", engine.graph[index_source].buffers[0]);
    // println!("First block of buffer: {:?}", engine.graph[i].buffers[0]);
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

fn new_node() {
    // some abstraction
    let engine = Engine::new(44100);

    // TODOs:
    let i_source = engine.new_node("sin", ["440"]);
    let i_sourcemul = engine.new_node("mul", [""]);
    let i_mod = engine.new_node("sin", ["10"]);
    let i_modmul = engine.new_node("mul", ["0.4"]);
    let i_modadd = engine.new_node("add", ["0.5"]);
    engine.process([i_sourcemul]);
}

fn chain() {
    let engine = Engine::new(44100);
    // less code for users to write in Rust
    // TODO: write the chain method, make_edges, and process method

     // return a vec of nodeindex
    let i_mod = engine.chain([("sin", ["10"]), ("mul", ["0.1"]), ("add", ["0.5"])]);
    let out = engine.chain([("sin", ["440"]), ("mul", [""])]);
    engine.make_edges([(i_mod[2], out[1])]);
    engine.process(out[1]);
    println!("{:?}", engine.get_buffers(out[1]));
}

/// make graph is one thing, but we need to update the graph
/// It's about adding nodes, remove nodes, and modify nodes
fn set_paras() {
    let engine = Engine::new(44100);
    // but another important thing is to set the paras in real-time
    engine.add_chain("refname", ["sin 440", "mul _mod"]);
    engine.add_chain("_mod", ["sin 0.5", "mul 0.1", "add 0.5", "refname 2"]);
    engine.set("_mod", 2, "0.5");
    // with dasp graph crate there is no direct way to do it
    // but we can have a dummy const node connect to each params
}