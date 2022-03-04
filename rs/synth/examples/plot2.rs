use gnuplot::*;
use glicol_synth::{ 
    AudioContextBuilder,
    oscillator::{SawOsc},
    filter::{ ResonantLowPassFilter},
    signal::ConstSig,
};

fn main () {

    let mut context = AudioContextBuilder::<128>::new()
    .sr(44100).channels(1).build();
    let node_a = context.add_mono_node( SawOsc::new().freq(30.) );
    let node_b = context.add_mono_node( ResonantLowPassFilter::new().cutoff(100.0) );
    let node_c = context.add_mono_node( ConstSig::new(50.0) );
    context.chain(vec![node_a, node_b, context.destination]);
    context.connect(node_c, node_b);

    // plot part
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..( 44100 / 128) {
        let buf = context.next_block();
        for i in 0..128 {
            x.push(n);
            n += 1;
            y.push(buf[0][i]); // use the buf here
        };
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Glicol output", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .lines(
            &x,
            &y,
            &[Caption("left")],
        );
    fg.show().unwrap();
}