// you should install gnuplot on your os
use gnuplot::*;
use glicol::Engine;

fn main () {
    let mut engine = Engine::<128>::new(44100);

    // engine.set_code("a: tri 10");
    engine.set_code("~trigger: speed 8.0 >> seq 60

    // ~env: ~trigger >> envperc 0.01 0.1
    
    ~pitch: ~trigger >> mul 261.626
    o: squ ~pitch");

    // engine.set_code("aa: tri ~mod; ~mod: sin 2.0 >> mul 40 >> add 60");
    // engine.set_code("~a: choose 48 55 51 58

    // ~b: choose 36 60 0 0 0 0 0
    
    // ~trigger: speed _ >> seq ~a ~b >> mul 2.0
    
    // ~env: ~trigger >> envperc _ >> mul 0.2
    
    // ~pitch: ~trigger >> mul 261.626
    
    // lead: saw ~pitch >> mul ~env >> rlpf ~cut 3.0 
    // >> mul 0.6 >> plate 0.1
    
    // ~cut: squ _ >> mul 3700.0 >> add 4000.0");
    // engine.set_code("aa: seq 60 >> sawsynth 0.01 0.02");
    // engine.set_code("aa: imp 1.0");
    // engine.set_code("aa: imp 2.0 >> shape 0.1, 1.0 | 0.2, 0.5 | 0.5, 0.0");
    // engine.set_code("aa: sin 44 >> pan -0.9");
    // engine.set_code("bb: imp 10.0 >> delay ~rand

    // ~rand: pha 1.0 >> mul 0.05 >> add 0.01");
    // engine.set_code("aa: squ 99 >> onepole 0.1 >> mul 0.1 >> add 0.2");
    // engine.set_code("aa: imp 0.1 >> delay ~mod
    
    // ~mod: squ 99 >> onepole 0.9 >> mul 0.1 >> add 0.2");
    // engine.set_code("out: imp 0.1 >> delay ~mod
    
    // ~mod: squ 0.1 >> mul 0.1 >> add 0.2");
    // engine.set_code("test: pha 1.0");
    // engine.set_code("out: imp 10 >> delay ~mod; ~mod: pha 0.1 >> add 0.01;");
    // engine.set_code("out_a: imp 10.0; out_b: imp 10.0 >> mul ~amp >> delay ~line; ~line: pha 1.0 >> mul 0.4 >> add 0.1; ~amp: pha 1.0 >> mul -1.0");
    // engine.set_code("out: seq 60 >> ks 60 0.99 0.01");
    // engine.set_code("~left: sin 10; ~right: sin 20; out: balance ~left ~right 0.5;");
    // engine.set_code("tt: sin 44 >> amplfo 1.0");
    plot(engine, 44100);
}

fn plot(mut engine: Engine::<128>, step: usize) {
    // engine.make_graph().unwrap();
    
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut y2 = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(step / 128) {
        let out = engine.gen_next_buf(&mut [0.0;128]).unwrap().0;
        // let out = engine.gen_next_buf_64().unwrap();
        for i in 0..128 {
            x.push(n);
            n += 1;
            y.push(out[i]);
            y2.push(out[i+128])
        }
        // print!("out: {:?}", out);
    }
    println!("node_by_chain {:?}", engine.node_by_chain);
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Glicol output", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .lines(
            &x,
            &y,
            &[Caption("left")],
        ).lines(
            &x,
            &y2,
            &[Caption("right")],
        );
    fg.show().unwrap();
}