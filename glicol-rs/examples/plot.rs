// you should install gnuplot on your os
use gnuplot::*;
use glicol::Engine;
use glicol::EngineError;

fn main () -> Result<(), EngineError> {
    let mut engine = Engine::new();
    engine.set_code("aa: sin 60 >> plate 0.1");

    engine.update();
    engine.make_graph()?;

    println!("audio_nodes {:?}", engine.graph.raw_nodes()[0].weight.buffers[0]);

    // println!("audio_nodes {:?}", engine.audio_nodes);
    // println!("control_nodes {:?}", engine.control_nodes);
    // for e in engine.graph.raw_edges() {
    //     println!("raw_edges {:?}", e);
    // }

    // println!("{:?}", engine.graph.raw_nodes()[0]);

    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut y2 = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..6400 as usize {
        // let out = engine.gen_next_buf_128().unwrap().0;
        let out = engine.gen_next_buf_64().unwrap();
        for i in 0..64 {
            x.push(n);
            n += 1;
            y.push(out[i]);
            y2.push(out[i+64])
        }
    }

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
    Ok(())
}