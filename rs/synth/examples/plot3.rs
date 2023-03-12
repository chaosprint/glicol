use gnuplot::*;
use glicol_synth::{ 
    AudioContextBuilder,
    operator::{Mul, Add},
    // sampling::{Sampler},
    oscillator::SinOsc,
    delay::{DelayMs},
    signal::Impulse,
};

fn main () {

    let mut context = AudioContextBuilder::<128>::new()
    .sr(44100).channels(1).build();
    let node_a = context.add_mono_node( Impulse::new().freq(1000.0) );
    // let node_b = context.add_stereo_node( Sampler::new((&[1.0, 0.0, 0.0], 1)) );
    // let node_c = context.add_stereo_node( Mul::new(0.5) );
    let node_b = context.add_mono_node(DelayMs::new().delay(0.0, 1));
    let (node_d, node_e, node_f) = (
        context.add_mono_node(SinOsc::new()),
        context.add_mono_node(Mul::new(100.)),
        context.add_mono_node(Add::new(200.))
    );
    // let node_j = context.add_mono_node(Mul::new(100.));
    
    context.chain(vec![
        node_a,
        node_b,
        // node_j,
        context.destination
    ]);
    context.chain(vec![
        node_d,
        node_e,
        node_f,
        node_b
    ]);

    // plot part
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..( 512 / 128 ) {
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