#[test]
fn hello_standalone() {
    use glicol::Engine;
    // use glicol::node::oscillator::{SinOsc};
    // use glicol::node::operator::{Mul};
    use glicol::node::source::ConstSig;
    use glicol::node::Para::Number as Num;
    
    let mut engine = Engine::new();
    let i = engine.graph.add_node(ConstSig::new(Num(42.0)));
    engine.processor.process(&mut engine.graph, i);
    for i in engine.graph[i].buffers[0].iter() {
        assert_eq!(*i, 42.)
    }
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