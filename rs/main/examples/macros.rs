use glicol::*;
use glicol_synth::*;
use glicol_synth::operation::mul::*;
use glicol_synth::signal::noise::*;

const SIZE: usize = 128;

fn main () {
    let mut e = Engine::<SIZE>::new(44100);

    // equal to: let i = e.make_chain(vec![noise!(SIZE, 42), mul!(SIZE, 0.01*0.1)]);
    let i = chain!([noise!(SIZE => 42), mul!(SIZE => 0.01*0.1)] in e);
    e.process(i[1]);
    println!("{:?}", e.graph[i[1]].buffers);
}