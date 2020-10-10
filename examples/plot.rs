// you should install gnuplot on your os
use gnuplot::*;
use glicol::Engine;

fn main () {
    let mut engine = Engine::new();
    engine.set_code("~amp: seq 50 _~a__ >> envperc 0.01 1.0\n\n_lead: sin 50 >> mul ~amp\n\n~a: noiz 0");
    engine.update();
    engine.make_graph();

    // println!("audio_nodes {:?}", engine.audio_nodes);
    // for e in engine.graph.raw_edges() {
    //     println!("raw_edges {:?}", e);
    // }

    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(44100.0*2.0/64.0) as usize {
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