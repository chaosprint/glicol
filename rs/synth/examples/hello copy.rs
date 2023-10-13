use glicol_synth::{
    AudioContextBuilder,
    signal::ConstSig,
    Message
};

fn main() {
    let mut context = AudioContextBuilder::<128>::new()
    .sr(44100)
    .channels(1)
    .build();

    let node_a = context.add_mono_node(ConstSig::new(42.));

    // all the process will happen to the destination node
    context.connect(node_a, context.destination);

    // that's all, you can use this graph.next_block() in a callback loop
    println!("first block {:?}", context.next_block());

    // message
    context.send_msg(node_a, Message::SetToNumber(0, 100.) );
    println!("second block, after msg {:?}", context.next_block());
}