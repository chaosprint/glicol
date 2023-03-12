/// deprecated!

use gnuplot::*;
use glicol_synth::{ 
    AudioContextBuilder,
    signal::Impulse,
    operator::Mul,
    // effect::FreeverbNode,
};

fn main () {

    let mut context = AudioContextBuilder::<128>::new()
    .sr(44100).channels(2).build();
    let node_a = context.add_mono_node( Impulse::new().freq(0.1) );
    let node_a2 = context.add_stereo_node( Mul::new(0.9) );
    // let node_b = context.add_stereo_node( FreeverbNode::new() );
    // context.connect(node_a, context.destination);
    context.chain(vec![node_a, node_a2, context.destination]);

    // plot part
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut y2 = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..( 44100 / 128) {
        let buf = context.next_block();
        for i in 0..128 {
            x.push(n);
            n += 1;
            y.push(buf[0][i]); // use the buf here
            y2.push(buf[1][i]);
        };
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Impulse response", &[Font("Courier", 8.) ])
        .set_legend(Graph(0.8), Graph(0.9), &[], &[Font("Courier", 8.) ])
        .lines(
            &x,
            &y,
            &[Caption("left"), Color("#e879f9"), LineWidth(2.0)],
        ).lines(
            &x,
            &y2,
            &[Caption("right"), Color("#38bdf8"), LineWidth(2.0)],
        );
    fg.show().unwrap();
}