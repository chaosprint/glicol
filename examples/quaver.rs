extern crate find_folder;

extern crate quaverseries_rs;
use quaverseries_rs::Engine;

// you should install gnuplot on your os
use gnuplot::*;

fn main () {
    let mut engine = Engine::new();
    let assets = find_folder::Search::ParentsThenKids(5, 5).for_folder("assets").unwrap();
    engine.set_code(std::fs::read_to_string(assets.join("env.qvs")).unwrap());
    engine.update();
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(88200.0*2.0/128.0) as usize {
        let out = engine.gen_next_buf_128();
        for i in 0..128 {
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