use glicol_synth::{
    AudioContextBuilder,
    effect::Plate,
    signal::ConstSig,
    Message
};

fn main() {
    let mut context = AudioContextBuilder::<8>::new()
    .sr(44100)
    .channels(2)
    .build();

    let c = context.add_stereo_node(ConstSig::new(1.));
    let node_a = context.add_stereo_node(Plate::new(0.5));

    // all the process will happen to the destination node
    context.chain(vec![c, node_a, context.destination]);

    // that's all, you can use this graph.next_block() in a callback loop
    println!("first block {:?}", context.next_block());

    // message
    // context.send_msg(node_a, Message::SetToNumber(0, 1.) );
    // println!("second block, after msg {:?}", context.next_block());
}