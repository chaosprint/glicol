use glicol_synth::{
    AudioContextBuilder,
    AudioContextConfig,
    AudioContext,
    audiocontext,
    node::oscillator::SinOsc,
    
};

fn main() {
    // let mut context = AudioContextBuilder::<128>::new()
    // .sr(44100)
    // .channels(2)
    // .max_nodes(1024)
    // .max_edges(1024)
    // .build();

    // // alternative
    // let mut context = AudioContext::<128>::new(
    //     AudioContextConfig {
    //         sr: 44100,
    //         max_nodes: 256,
    //         max_edges: 256,
    //         ..AudioContextConfig::default()
    //     }
    // );

    let mut context = audiocontext!(128, {
        sr: 44100,
        channels: 2
    });

    let index = context.add_mono_node(
        // alternative: SinOsc::new().freq(440.).sr(44100)
        SinOsc {
            freq: 440.0,
            sr: 44100, // you can replace these two lines with
            phase: 0.0, // ..SinOsc::default()
        }
    );

    // all the process will happen to the destination node
    context.connect(index, context.destination);

    // that's all, you can use this graph.next_block() in a callback loop
    println!("first block {:?}", context.next_block());
}

// real-time communication
// graph.send_msg( index, 0, Message::Float(42.) );
// println!("after msg {:?}", graph.next_block());
