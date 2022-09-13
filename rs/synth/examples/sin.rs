use glicol_synth::{
    AudioContextBuilder,
    oscillator::SinOsc,
    operator::Mul,
    Message
};

fn main() {
    let mut context = AudioContextBuilder::<8>::new()
    .sr(44100)
    .channels(2)
    .build();

    let node_a = context.add_mono_node(SinOsc::new().freq(440.0));
    let node_b = context.add_stereo_node(Mul::new(0.1));
    context.connect(node_a, node_b);
    context.connect(node_b, context.destination);

    println!("first block {:?}", context.next_block());
    // message
    context.send_msg(node_a, Message::SetToNumber(0, 100.) );
    println!("second block, after msg {:?}", context.next_block());
}