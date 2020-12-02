// you should install gnuplot on your os
use gnuplot::*;
use glicol::Engine;
use glicol::EngineError;

fn main () -> Result<(), EngineError> {
    let mut engine = Engine::new();
    engine.set_code("abc: saw 60 >> ~mul
    
    ~mul: mul 0.5");
    
    // ~filter: mul 0.3");
    engine.update();
    engine.make_graph()?;

    println!("audio_nodes {:?}", engine.audio_nodes);
    for e in engine.graph.raw_edges() {
        println!("raw_edges {:?}", e);
    }

    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(512.0*2.0/64.0) as usize {
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
    Ok(())
}