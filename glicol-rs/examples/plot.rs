// you should install gnuplot on your os
use gnuplot::*;
use glicol::Engine;
use glicol::EngineError;

fn main () -> Result<(), EngineError> {
    let mut engine = Engine::new();
    engine.set_code("hi: sin 440.0

    // if this doesn't play, check your browser console
    // chrome or firefox are recommended
    
    // this is a comment
    // uncomment the line below, and click on the update button to update the sound
    // another: sin 441.0
    
    // try to control the volume by adding another node function
    // another: sin 441.0 >> mul 0.5
    
    // this example shows the basic usage of nodes
    // a node can have several inputting signals but only one output signal
    // here \"sin\" is a node that outputs sine wave signal based on its argument frequency
    // in this example, \"sin\" has no input signal
    // \"mul\" has one input from its left hand side
    // \"mul\" processes the input signal by multiplying the input signal with its first argument
    
    // everything before the colon, e.g. \"hi\" or \"another\", is called [reference]
    // this will be explained in the next page (am)");

    engine.update();
    engine.make_graph()?;

    println!("node_by_chain {:?}", engine.node_by_chain);
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut y2 = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(3000.0/128.0) as usize {
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