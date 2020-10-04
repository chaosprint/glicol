// you should install gnuplot on your os
use gnuplot::*;
use quaverseries_rs::Engine;

fn main () {
    let mut engine = Engine::new();
    engine.set_code("~test: choose 30 50 70");
    // engine.set_code("&trigger: speed 16.0 >> loop 45 48 50 48 55 53 55 57

    // &env: &trigger >> envperc 0.01 0.1 >> mul 0.5
    
    // &pitch: &trigger >> mul 261.626
    
    // ~lead: saw &pitch >> mul &env >> lpf &mod 2.0
    
    // &mod: sin 0.2 >> linrange 1000 3000");
    // engine.set_code("&a: noiz 0 >> mul 1 >> add 40

    //     &trigger: speed 16 >> loop &a
        
    //     &env: &trigger >> envperc 0.01 0.1 >> mul 0.5
        
    //     &pitch: &trigger >> mul 261.626
        
    //     ~lead: saw &pitch >> mul &env >> lpf &cut 1.0
        
    //     &cut: sin 0.2 >> mul 1000 >> add 2000");
    // engine.set_code("&a: noiz 0 >> mul 10 >> add 60

    // &trigger: speed 16 >> loop &a
    
    // &env: &trigger >> envperc 0.01 0.1 >> mul 0.5
    
    // &pitch: &trigger >> mul 261.626
    
    // ~lead: saw &pitch >> mul &env");
    engine.update();
    engine.make_graph();

    // println!("audio_nodes {:?}", engine.audio_nodes);
    // for e in engine.graph.raw_edges() {
    //     println!("raw_edges {:?}", e);
    // }

    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(128.0*2.0/64.0) as usize {
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


    // "&fm: sin 30 >> mul 300 >> add 500\n\n~hi: sin &fm >> mul 0.3"
    // engine.set_code("&env: imp 1.0 >> env_perc 0.1 1.0\n\n~lead: sin 100 >> mul &env");
    // "&trigger: loop 60 58 _67 _62

    // &env: &trigger >> env_perc 0.01 0.1 >> mul 0.5
    
    // ~lead: saw 100.0 >> mul &env"
    // "&cut: sin 1 >> mul 1000 >> add 2000\n\n~aa: noiz 0 >> lpf &cut 1.0"

    // "&trigger: loop 60 &a _67 _62

    // &a: sin 10 >> mul 20 >> add 50

    // &env: &trigger >> env_perc 0.01 0.1 >> mul 0.5

    // ~lead: saw 100.0 >> mul &env"
    // "&a: noiz 0 >> mul 10 >> add 60

    // &trigger: loop &a
    
    // &env: &trigger >> env_perc 0.01 0.1 >> mul 0.5
    
    // &pitch: &trigger >> mul 261.626
    
    // ~lead: saw &pitch >> mul &env"
    
    // engine.set_code("~test: speed 16 >> loop &a &b
    
    // &a: noiz 0 >> add 60
    
    // &b: noiz 0 >> add 40");