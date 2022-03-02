use glicol_synth::{
    AudioContextBuilder,
    audiocontext,
    oscillator::SinOsc,
    operator::Mul,
};

const SAMPLE_RATE: usize = 44100;

fn main() {
    let mut context = audiocontext!(128, {
        sr: SAMPLE_RATE,
        channels: 1
    });

    let (nodeid_list, _) = context.add_node_chain(vec![
        SinOsc {
            freq: 44100./128.,
            sr: SAMPLE_RATE, // you can replace these two lines with
            phase: 0.0, // ..SinOsc::default()
        }.to_boxed_nodedata(1),
        Mul::new(0.5).to_boxed_nodedata(1)
    ]);

    context.connect(nodeid_list[nodeid_list.len()-1], context.destination);
    println!("first block {:?}", context.next_block());
}