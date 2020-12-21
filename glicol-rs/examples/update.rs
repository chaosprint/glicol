// you should install gnuplot on your os
use gnuplot::*;
use glicol::Engine;
use glicol::EngineError;

fn main () -> Result<(), EngineError> {
    let mut engine = Engine::new();
    engine.set_code("aa: const 10 >> sin 1");

    engine.update();
    engine.make_graph()?;

    println!("node_by_chain {:?}", engine.node_by_chain);

    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut y2 = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(43000.0/128.0) as usize {
        let out = engine.gen_next_buf_128(&mut [0.0;128]).unwrap().0;
        // let out = engine.gen_next_buf_64().unwrap();
        for i in 0..128 {
            x.push(n);
            n += 1;
            y.push(out[i]);
            y2.push(out[i+128])
        }
    }

    engine.set_code("aa: const 20 >> sin 1");
    engine.update();
    engine.make_graph()?;
    println!("node_by_chain {:?}", engine.node_by_chain);

    for _ in 0..(43000.0/128.0) as usize {
        let out = engine.gen_next_buf_128(&mut [0.0;128]).unwrap().0;
        
        // let out = engine.gen_next_buf_64().unwrap();
        for i in 0..128 {
            x.push(n);
            n += 1;
            y.push(out[i]);
            y2.push(out[i+128])
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