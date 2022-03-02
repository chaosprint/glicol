use glicol_synth::{
    AudioContextBuilder,
    oscillator::SinOsc,
    operator::Mul,
    Message
};

const SAMPLE_RATE: usize = 44100;

fn main() {
    let mut context = AudioContextBuilder::<128>::new()
    .sr(44100)
    .channels(1)
    .build();

    let a = context.add_mono_node( SinOsc::default() );
    let b = context.add_mono_node( Mul::new(0.5) );
    context.connect(a, b);
    context.send_msg(b, Message::SetParaToNumber((0, 0.25)));
    context.send_msg(b, Message::MainInput(a.index()));
    // context.send_msg(b, Message::SidechainInput(a.index()));
    context.connect(b, context.destination);
    println!("first block {:?}", context.next_block());
}

// real-time communication
// graph.send_msg( index, 0, Message::Float(42.) );
// println!("after msg {:?}", graph.next_block());