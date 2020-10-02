extern crate quaverseries_rs;
use quaverseries_rs::Engine;

// you should install gnuplot on your os
use gnuplot::*;

fn main () {
    let mut engine = Engine::new();

    // "&fm: sin 30 >> mul 300 >> add 500\n\n~hi: sin &fm >> mul 0.3"
    // engine.set_code("&env: imp 1.0 >> env_perc 0.1 1.0\n\n~lead: sin 100 >> mul &env");
    // "&trigger: loop 60 58 _67 _62

    // &env: &trigger >> env_perc 0.01 0.1 >> mul 0.5
    
    // ~lead: sin 100.0 >> mul &env"
    engine.set_code("&cut: sin 1 >> mul 1000 >> add 2000\n\n~aa: noiz 0 >> lpf &cut 1.0");
    engine.update();
    engine.make_graph();

    // println!("audio_nodes {:?}", engine.audio_nodes);
    // for e in engine.graph.raw_edges() {
    //     println!("raw_edges {:?}", e);
    // }

    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(88200.0*2.0/64.0) as usize {
        let out = engine.gen_next_buf_64();
        for i in 0..64 {
            x.push(n);
            n += 1;
            y.push(out[i]);
        }
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("A plot", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .lines(
            &x,
            &y,
            &[],
        );
    fg.show().unwrap();
}