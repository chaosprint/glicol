use glicol::*;
use glicol::node::mul::*;
use glicol::node::noise::*;

fn main () {
    let mut e = Engine::new(44100);
    let i = chain!([noise!(42), mul!(0.5)] in e);
    e.process(i[1]);
    println!("{:?}", e.graph[i[1]].buffers);
}