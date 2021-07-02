use gnuplot::*;
use glicol::*;
use glicol_synth::*;
use glicol_synth::oscillator::saw_osc::*;
use glicol_synth::oscillator::squ_osc::*;
use glicol_synth::oscillator::tri_osc::*;
use glicol_synth::operation::mul::*;
fn main () {
    let mut engine = Engine::new(44100);
    let out = engine.make_chain(vec![tri_osc!({freq: 441.0}), mul!(1.)]);
    engine.process(out[1]); // this is a simplified method for calling processor on graph
    println!("{:?}", engine.graph[out[1]].buffers);

    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut n = 0;

    for i in 0..128 {
        x.push(n);
        n += 1;
        y.push(engine.graph[out[1]].buffers[0][i]);
    };
    let mut fg = Figure::new();

    fg.axes2d()
        .set_title("Glicol output", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .lines(
            &x,
            &y,
            &[Caption("left")],
        );
    fg.show().unwrap();
}