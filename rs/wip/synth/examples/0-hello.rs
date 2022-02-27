use glicol_synth::{
    graph::StableGraph,
    node::oscillator::SinOsc,
};

fn main() {
    // let mut graph = StableGraph::<128>::new().chan(4).with_capacity(256, 256);
    // alternative
    let mut graph = StableGraph::<16>::new(); // stereo, 1024, 1024

    let index = graph.add_mono_node(
        // alternative: SinOsc::new().freq(440.).sr(44100)
        SinOsc {
            freq: 440.0,
            sr: 44100, // you can replace these two lines with
            phase: 0.0, // ..SinOsc::default()
        }
    );

    // all the process will happen to the destination node
    graph.connect(index, graph.destination);

    // that's all, you can use this graph.next_block() in a callback loop
    println!("first block {:?}", graph.next_block());
}

// real-time communication
// graph.send_msg( index, 0, Message::Float(42.) );
// println!("after msg {:?}", graph.next_block());
