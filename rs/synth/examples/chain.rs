use glicol_synth::{
    AudioContextBuilder,
    oscillator::SinOsc,
    operator::Mul,
    signal::ConstSig,
    Message
};

fn main() {
    let mut context = AudioContextBuilder::<8>::new()
    .sr(44100)
    .channels(1)
    .build();

    let node_a = context.add_mono_node(ConstSig::new(24.));
    let node_b = context.add_mono_node(Mul::new(0.5));
    // context.chain(vec![node_a, node_b, context.destination]);

    let node_c = context.add_mono_node(ConstSig::new(0.1));
    context.chain(vec![node_c, node_b]);

    context.chain(vec![node_a, node_b, context.destination]);
    println!("first block {:?}", context.next_block());
}