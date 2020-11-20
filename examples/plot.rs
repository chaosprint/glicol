// you should install gnuplot on your os
use gnuplot::*;
use glicol::Engine;

fn main () {
    let mut engine = Engine::new();
    engine.set_code("~a: choose 60 50 90 80");
    engine.update();
    match engine.make_graph() {
        Ok(_) => {},
        Err(_) => {}
    };

    // println!("audio_nodes {:?}", engine.audio_nodes);
    // for e in engine.graph.raw_edges() {
    //     println!("raw_edges {:?}", e);
    // }

    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(256.0*2.0/64.0) as usize {
        // let out = engine.gen_next_buf_128().unwrap().0;
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