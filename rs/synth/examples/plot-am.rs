use gnuplot::*;
use glicol_synth::{ 
    AudioContextBuilder,
    oscillator::{SinOsc, SquOsc},
    operator::Mul,
    operator::Add,
    signal::ConstSig,
};

fn main () {

    let mut context = AudioContextBuilder::<128>::new()
    .sr(44100).channels(1).build();
    let node_a = context.add_mono_node( SinOsc::new().freq(10.) );
    let node_b = context.add_mono_node( Mul::new(0.5) );
    // context.connect(node_a, context.destination);

    // let node_c = context.add_mono_node( SquOsc::new().freq(10.) );
    let node_d = context.add_mono_node( Mul::new(8.0) );
    let node_e = context.add_mono_node( Add::new(1.0) );
    let node_f = context.add_mono_node( SinOsc::new().freq(100.) );
    context.chain(vec![node_a, node_b, context.destination]);
    context.chain(vec![node_f, node_d, node_e, node_b]);
    // context.connect_with_order(node_e, node_b, 1);

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