use gnuplot::*;
use glicol_synth::{ 
    AudioContextBuilder,
    effect::Plate,
    signal::Impulse,
};

fn main () {

    let mut context = AudioContextBuilder::<128>::new()
    .sr(44100).channels(1).build();

    let c = context.add_mono_node(Impulse::new().freq(10.));
    let node_a = context.add_stereo_node(Plate::new(0.5));

    // all the process will happen to the destination node
    context.chain(vec![c, node_a, context.destination]);

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