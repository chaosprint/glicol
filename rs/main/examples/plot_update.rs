// you should install gnuplot on your os
use gnuplot::*;
use glicol::Engine;
use glicol::EngineError;

fn main () {
    let mut engine = Engine::new(44100);
    // engine.elapsed_samples = 512;
    engine.set_code("aa: sin 44");
    // engine.update = true;
    engine.make_graph().unwrap();
    // println!("\n\nnode_by_chain {:?}\n\n", engine.node_by_chain);
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut y2 = Vec::<f32>::new();
    let mut n = 0;
    for _ in 0..(70000.0/128.0) as usize {
        let out = engine.gen_next_buf_128(&mut [0.0;128]).unwrap().0;
        for i in 0..128 {
            x.push(n);
            n += 1;
            y.push(out[i]);
            y2.push(out[i+128])
        }
    }

    engine.set_code("aa: si");
    engine.make_graph();

    // // println!("\n\nnode_by_chain {:?}\n\n", engine.node_by_chain);
    for _ in 0..(70000.0/128.0) as usize {
        let out = engine.gen_next_buf_128(&mut [0.0;128]).unwrap().0;
        for i in 0..128 {
            x.push(n);
            n += 1;
            y.push(out[i]);
            y2.push(out[i+128])
        }
    }

    const LO: usize = 0;
    const HI: usize = 130000;

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Glicol output", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .lines(
            // &x,
            // &y,
            &x[LO..HI],
            &y[LO..HI],
            &[Caption("left")],
        ).lines(
            // &x,
            // &y2,
            &x[LO..HI],
            &y2[LO..HI],
            &[Caption("right")],
        );
    fg.show().unwrap();
}